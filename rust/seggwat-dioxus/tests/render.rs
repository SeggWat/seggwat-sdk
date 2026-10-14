//! Renders SeggWat-shaped JSON through the Dioxus components (SSR).

#[path = "../../fixtures.rs"]
mod fixtures;

use dioxus::prelude::*;
use fixtures::*;
use seggwat_dioxus::{ChangelogFeed, IdeaBoard, RoadmapBoard};

#[test]
fn vote_module_matches_the_js_package() {
    assert_eq!(
        include_str!("../assets/seggwat-board.js"),
        include_str!("../../../js/board/seggwat-board.js"),
        "run `just sync-js`"
    );
}

#[test]
fn signed_in_board_has_vote_buttons() {
    let html = dioxus_ssr::render_element(rsx! {
        IdeaBoard { ideas: fixture_ideas(), voting: fixture_voting(Some("tok")) }
    });
    assert!(html.contains(r#"data-token="tok""#), "{html}");
    assert!(html.contains(r#"data-seggwat-vote="a""#));
    assert!(html.contains(r#"aria-pressed="false""#));
    assert!(html.contains("Idea &#60;a&#62;"), "titles are escaped");
}

#[test]
fn signed_out_board_links_to_login() {
    let html = dioxus_ssr::render_element(rsx! {
        IdeaBoard { ideas: fixture_ideas(), voting: fixture_voting(None) }
    });
    assert!(html.contains(r#"href="/login?next=/ideas""#), "{html}");
    assert!(!html.contains("data-seggwat-vote"));
    assert!(!html.contains("data-token"));
}

#[test]
fn roadmap_votes_only_on_open_columns() {
    assert_roadmap(&dioxus_ssr::render_element(rsx! {
        RoadmapBoard { roadmap: fixture_roadmap(), voting: fixture_voting(Some("tok")) }
    }));
}

#[test]
fn changelog_renders_meta_and_body() {
    assert_changelog(&dioxus_ssr::render_element(rsx! {
        ChangelogFeed { page: fixture_changelog() }
    }));
}
