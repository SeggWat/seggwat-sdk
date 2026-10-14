//! The subset of SeggWat's public board API (`/api/v1/p/{project_key}/…`)
//! these components render. Unknown fields are ignored, so SeggWat can add
//! fields without breaking older apps. `Serialize` lets Dioxus server
//! functions pass the values to the client unchanged.

use serde::{Deserialize, Serialize};

/// An idea on the public board.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Idea {
    pub id: String,
    pub title: String,
    #[serde(default)]
    pub description: String,
    pub votes_count: i64,
    /// `Open`, `Planned`, `Started`, `Completed`, `Declined`, …
    pub status: String,
    #[serde(default)]
    pub tags: Vec<Tag>,
    /// Version this idea shipped in (only for `Completed`).
    #[serde(default)]
    pub completed_in_version: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tag {
    pub slug: String,
    pub name: String,
    pub color: String,
}

/// `GET /ideas` — votable ideas (SeggWat hides `Completed` by default).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IdeaList {
    pub items: Vec<Idea>,
    pub has_more: bool,
    pub total: u64,
}

/// `GET /roadmap` — one round trip for the three roadmap columns.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Roadmap {
    pub planned: IdeaList,
    pub started: IdeaList,
    pub completed: IdeaList,
}

/// `GET /changelog` — published entries, newest first.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChangelogPage {
    pub items: Vec<ChangelogEntry>,
    pub has_more: bool,
    pub total: u64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChangelogEntry {
    pub id: String,
    pub title: String,
    /// Markdown source.
    pub body: String,
    /// `body` rendered to safe HTML by the `server` client. SeggWat never
    /// sends it; it is filled in after the fetch.
    #[serde(default)]
    pub body_html: String,
    #[serde(default)]
    pub version: Option<String>,
    /// `added`, `improved`, `fixed`, `changed`, `removed` or `security`.
    #[serde(default)]
    pub category: Option<String>,
    /// RFC 3339.
    #[serde(default)]
    pub published_at: Option<String>,
}

impl ChangelogEntry {
    /// `YYYY-MM-DD` of `published_at`.
    pub fn date(&self) -> Option<&str> {
        self.published_at.as_deref().and_then(|d| d.get(..10))
    }
}

/// What a page needs to make vote buttons work in the browser.
///
/// The browser talks to SeggWat directly, so every visitor keeps their own
/// per-IP rate limit. `token` is the signed-in user's short-lived end-user JWT;
/// `None` renders vote buttons as links to `login_url`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Voting {
    /// SeggWat origin, e.g. `https://seggwat.com`.
    pub api_base: String,
    pub project_key: String,
    pub token: Option<String>,
    pub login_url: String,
}
