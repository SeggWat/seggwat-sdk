//! Renders SeggWat-shaped JSON through the Topcoat components.

#[path = "../../fixtures.rs"]
mod fixtures;

use fixtures::*;
use seggwat_topcoat::{VOTE_JS, changelog_feed, idea_board, roadmap_board};
use topcoat::{
    asset::{AssetConfig, Manifest, ManifestEntry},
    context::{Cx, CxTestBuilder},
    view::{View, ViewExt, view},
};

/// A context whose asset catalog knows the vote module, as a real app's does.
fn app_cx() -> Cx {
    let manifest = Manifest {
        version: 1,
        assets: vec![ManifestEntry {
            id: VOTE_JS.id(),
            file: "seggwat-board-abc.js".into(),
            hash: "abc".into(),
            content_type: "text/javascript".into(),
        }],
    };
    CxTestBuilder::new()
        .app_context(AssetConfig::hosted_at("/assets", manifest))
        .build()
}

async fn html(v: impl View) -> String {
    v.single().await.unwrap().render(&app_cx())
}

#[test]
fn vote_module_matches_the_js_package() {
    assert_eq!(
        include_str!("../assets/seggwat-board.js"),
        include_str!("../../../js/board/seggwat-board.js"),
        "run `just sync-js`"
    );
}

#[tokio::test]
async fn signed_in_board_has_vote_buttons_and_the_module() {
    let (ideas, voting) = (fixture_ideas(), fixture_voting(Some("tok")));
    let cx = &app_cx();
    let html = html(view! { cx => idea_board(ideas: &ideas, voting: &voting) }).await;
    assert!(html.contains(r#"data-token="tok""#), "{html}");
    assert!(html.contains(r#"data-seggwat-vote="a""#));
    assert!(html.contains(r#"aria-pressed="false""#));
    assert!(html.contains(r#"<script type="module" src="/assets/seggwat-board-abc.js">"#));
    assert!(html.contains("Idea &lt;a&gt;"), "titles are escaped");
    assert!(html.contains(">7<"));
}

#[tokio::test]
async fn signed_out_board_links_to_login_without_script() {
    let (ideas, voting) = (fixture_ideas(), fixture_voting(None));
    let cx = &app_cx();
    let html = html(view! { cx => idea_board(ideas: &ideas, voting: &voting) }).await;
    assert!(html.contains(r#"href="/login?next=/ideas""#), "{html}");
    assert!(!html.contains("data-seggwat-vote"));
    assert!(!html.contains("data-token"));
    assert!(!html.contains("<script"));
}

#[tokio::test]
async fn roadmap_votes_only_on_open_columns() {
    let (roadmap, voting) = (fixture_roadmap(), fixture_voting(Some("tok")));
    let cx = &app_cx();
    let html = html(view! { cx => roadmap_board(roadmap: &roadmap, voting: &voting) }).await;
    assert_roadmap(&html);
}

#[tokio::test]
async fn changelog_renders_meta_and_body() {
    let page = fixture_changelog();
    let cx = &app_cx();
    let html = html(view! { cx => changelog_feed(page: &page) }).await;
    assert_changelog(&html);
}
