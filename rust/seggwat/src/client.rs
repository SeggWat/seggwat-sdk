//! Server-side SeggWat client: cached anonymous reads and end-user tokens.
//!
//! Reads go out anonymously so one cached response serves every visitor.
//! SeggWat rate-limits its public reads per client IP, and all of a site's
//! server traffic shares one IP, so the cache is what keeps a busy page under
//! that limit. Per-viewer state (which ideas you voted for) and the votes
//! themselves are fetched by the browser — see [`Voting`].

use std::{
    collections::HashMap,
    sync::Mutex,
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};

use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use hmac::{Hmac, Mac};
use serde::{Serialize, de::DeserializeOwned};
use sha2::Sha256;

use crate::{ChangelogPage, IdeaList, Roadmap, Voting};

const CACHE_TTL: Duration = Duration::from_secs(60);
const TOKEN_TTL_SECS: u64 = 60 * 60;
const PAGE_SIZE: u64 = 50;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("seggwat request failed: {0}")]
    Http(#[from] reqwest::Error),
    #[error("seggwat returned {status}: {body}")]
    Status { status: u16, body: String },
    #[error("unexpected seggwat response: {0}")]
    Decode(#[from] serde_json::Error),
}

/// The signed-in user, as the host app knows them.
#[derive(Debug, Clone)]
pub struct Viewer {
    /// Stable user id in the host app.
    pub id: String,
    pub email: Option<String>,
    pub name: Option<String>,
}

pub struct Seggwat {
    api_base: String,
    project_key: String,
    site_origin: String,
    signing_secret: Option<String>,
    http: reqwest::Client,
    cache: Mutex<HashMap<String, (Instant, String)>>,
}

impl Seggwat {
    /// `api_base` is the SeggWat origin (`https://seggwat.com`). `site_origin`
    /// is this app's public origin; it must be in the project's allowed
    /// origins, because the browser sends it on votes and SeggWat checks it on
    /// reads too. `signing_secret` is the project's end-user signing secret;
    /// without it nobody can vote from this site.
    pub fn new(
        api_base: impl Into<String>,
        project_key: impl Into<String>,
        site_origin: impl Into<String>,
        signing_secret: Option<String>,
    ) -> Self {
        Self {
            api_base: api_base.into().trim_end_matches('/').to_string(),
            project_key: project_key.into(),
            site_origin: site_origin.into(),
            signing_secret,
            http: reqwest::Client::builder()
                .timeout(Duration::from_secs(10))
                .build()
                .expect("static reqwest config"),
            cache: Mutex::default(),
        }
    }

    /// Open, planned and in-progress ideas, most votes first.
    pub async fn ideas(&self) -> Result<IdeaList, Error> {
        self.get(&format!("ideas?limit={PAGE_SIZE}")).await
    }

    pub async fn roadmap(&self) -> Result<Roadmap, Error> {
        self.get(&format!("roadmap?limit={PAGE_SIZE}")).await
    }

    /// Changelog entries `offset..offset + 20`, with `body_html` filled in.
    pub async fn changelog(&self, offset: u64) -> Result<ChangelogPage, Error> {
        let page = self
            .get(&format!("changelog?offset={offset}&limit=20"))
            .await?;
        Ok(render_bodies(page))
    }

    /// Everything the vote buttons need. Pass the signed-in user, or `None`
    /// for an anonymous visitor (buttons then link to `login_url`).
    ///
    /// The token is the viewer's own and expires after an hour; do not put a
    /// page that carries one in a shared cache.
    pub fn voting(&self, viewer: Option<&Viewer>, login_url: impl Into<String>) -> Voting {
        Voting {
            api_base: self.api_base.clone(),
            project_key: self.project_key.clone(),
            token: viewer.and_then(|v| self.token(v)),
            login_url: login_url.into(),
        }
    }

    /// An HS256 end-user JWT, the format SeggWat's "signed tokens" end-user
    /// authentication expects. `None` without a signing secret.
    pub fn token(&self, viewer: &Viewer) -> Option<String> {
        let secret = self.signing_secret.as_deref()?;
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_or(0, |d| d.as_secs());
        Some(sign_hs256(
            &Claims {
                sub: &viewer.id,
                email: viewer.email.as_deref(),
                name: viewer.name.as_deref(),
                iat: now,
                exp: now + TOKEN_TTL_SECS,
                project_key: &self.project_key,
                aud: &self.project_key,
            },
            secret,
        ))
    }

    async fn get<T: DeserializeOwned>(&self, path: &str) -> Result<T, Error> {
        let url = format!("{}/api/v1/p/{}/{path}", self.api_base, self.project_key);
        if let Some((at, body)) = self.cache.lock().unwrap().get(&url)
            && at.elapsed() < CACHE_TTL
        {
            return Ok(serde_json::from_str(body)?);
        }

        let response = self
            .http
            .get(&url)
            .header("origin", &self.site_origin)
            .send()
            .await?;
        let status = response.status();
        let body = response.text().await?;
        if !status.is_success() {
            return Err(Error::Status {
                status: status.as_u16(),
                body,
            });
        }
        let value = serde_json::from_str(&body)?;
        self.cache
            .lock()
            .unwrap()
            .insert(url, (Instant::now(), body));
        Ok(value)
    }
}

fn render_bodies(mut page: ChangelogPage) -> ChangelogPage {
    for entry in &mut page.items {
        entry.body_html = markdown::to_html(&entry.body);
    }
    page
}

#[derive(Serialize)]
struct Claims<'a> {
    sub: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    email: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<&'a str>,
    iat: u64,
    exp: u64,
    project_key: &'a str,
    aud: &'a str,
}

fn sign_hs256(claims: &Claims, secret: &str) -> String {
    let header = URL_SAFE_NO_PAD.encode(br#"{"alg":"HS256","typ":"JWT"}"#);
    let payload = URL_SAFE_NO_PAD.encode(serde_json::to_vec(claims).expect("claims serialize"));
    let signing_input = format!("{header}.{payload}");
    let mut mac = Hmac::<Sha256>::new_from_slice(secret.as_bytes()).expect("hmac takes any key");
    mac.update(signing_input.as_bytes());
    let signature = URL_SAFE_NO_PAD.encode(mac.finalize().into_bytes());
    format!("{signing_input}.{signature}")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn client(secret: Option<&str>) -> Seggwat {
        Seggwat::new(
            "https://seggwat.test/",
            "proj-1",
            "https://app.test",
            secret.map(str::to_string),
        )
    }

    fn viewer() -> Viewer {
        Viewer {
            id: "user-1".into(),
            email: Some("a@b.test".into()),
            name: None,
        }
    }

    #[test]
    fn token_is_hs256_bound_to_the_project() {
        let token = client(Some("s3cret")).token(&viewer()).unwrap();
        let parts: Vec<&str> = token.split('.').collect();
        assert_eq!(parts.len(), 3);

        let claims: serde_json::Value =
            serde_json::from_slice(&URL_SAFE_NO_PAD.decode(parts[1]).unwrap()).unwrap();
        assert_eq!(claims["sub"], "user-1");
        assert_eq!(claims["aud"], "proj-1");
        assert_eq!(claims["project_key"], "proj-1");
        assert_eq!(claims["email"], "a@b.test");
        assert!(claims.get("name").is_none());
        assert_eq!(
            claims["exp"].as_u64().unwrap() - claims["iat"].as_u64().unwrap(),
            TOKEN_TTL_SECS
        );

        let mut mac = Hmac::<Sha256>::new_from_slice(b"s3cret").unwrap();
        mac.update(format!("{}.{}", parts[0], parts[1]).as_bytes());
        mac.verify_slice(&URL_SAFE_NO_PAD.decode(parts[2]).unwrap())
            .expect("signature verifies with the project secret");
    }

    #[test]
    fn changelog_markdown_is_rendered_without_raw_html_or_script_links() {
        let page: ChangelogPage = serde_json::from_value(serde_json::json!({
            "items": [{
                "id": "e1",
                "title": "t",
                "body": "**New** <script>alert(1)</script> [x](javascript:alert(1))",
            }],
            "has_more": false,
            "total": 1,
            "categories": [],
            "components": [],
            "email_subscribe_enabled": false,
        }))
        .unwrap();
        let html = &render_bodies(page).items[0].body_html;
        assert!(html.contains("<strong>New</strong>"));
        assert!(!html.contains("<script>"));
        assert!(!html.contains("javascript:"));
    }

    #[test]
    fn no_secret_means_no_token() {
        let voting = client(None).voting(Some(&viewer()), "/login");
        assert_eq!(voting.token, None);
        assert_eq!(voting.api_base, "https://seggwat.test");
    }
}
