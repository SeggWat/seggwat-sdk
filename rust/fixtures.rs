// Fixtures shared by the framework crates' render tests: SeggWat-shaped JSON.

fn idea(id: &str, status: &str, votes: i64) -> serde_json::Value {
    serde_json::json!({
        "id": id, "title": format!("Idea <{id}>"), "description": "desc",
        "votes_count": votes, "comments_count": 0, "status": status,
        "tags": [{ "slug": "ux", "name": "UX", "color": "#fff" }],
        "user_has_voted": false, "submitted_by_me": false, "can_edit": false,
        "edited": false, "admin_response": null, "created_at": "2026-10-01T00:00:00Z",
    })
}

fn section(items: Vec<serde_json::Value>) -> serde_json::Value {
    let total = items.len();
    serde_json::json!({ "items": items, "has_more": false, "total": total })
}

pub fn fixture_ideas() -> seggwat::IdeaList {
    serde_json::from_value(section(vec![idea("a", "Open", 7)])).unwrap()
}

pub fn fixture_roadmap() -> seggwat::Roadmap {
    serde_json::from_value(serde_json::json!({
        "planned": section(vec![idea("p", "Planned", 3)]),
        "started": section(vec![]),
        "completed": section(vec![idea("c", "Completed", 9)]),
        "portal_config": { "board_title": "Board" },
    }))
    .unwrap()
}

pub fn fixture_changelog() -> seggwat::ChangelogPage {
    let mut page: seggwat::ChangelogPage = serde_json::from_value(serde_json::json!({
        "items": [{
            "id": "e1", "title": "Dark mode", "body": "**yes**", "version": "1.2.0",
            "category": "added", "published_at": "2026-10-14T09:00:00+00:00",
        }],
        "has_more": false, "total": 1, "categories": ["added"], "components": [],
        "email_subscribe_enabled": false,
    }))
    .unwrap();
    page.items[0].body_html = "<p><strong>yes</strong></p>".into();
    page
}

pub fn fixture_voting(token: Option<&str>) -> seggwat::Voting {
    seggwat::Voting {
        api_base: "https://seggwat.test".into(),
        project_key: "proj".into(),
        token: token.map(str::to_string),
        login_url: "/login?next=/ideas".into(),
    }
}

pub fn assert_roadmap(html: &str) {
    assert!(html.contains("Planned") && html.contains("In progress") && html.contains("Shipped"));
    assert!(html.contains(r#"data-seggwat-vote="p""#), "{html}");
    assert!(
        !html.contains(r#"data-seggwat-vote="c""#),
        "shipped ideas are not votable"
    );
    assert!(html.contains("Nothing here yet."));
}

pub fn assert_changelog(html: &str) {
    assert!(html.contains(r#"datetime="2026-10-14""#), "{html}");
    assert!(html.contains("1.2.0"));
    assert!(html.contains("badge-success") && html.contains("Added"));
    assert!(html.contains("<p><strong>yes</strong></p>"));
}

pub fn assert_credit(html: &str, shown: bool) {
    // Escaping of `&` differs per framework, so match around it.
    let link = r#"href="https://seggwat.com/?utm_source=seggwat-sdk&"#;
    let rel = r#"utm_medium=board" rel="nofollow noopener""#;
    assert_eq!(html.contains(link) && html.contains(rel), shown, "{html}");
    assert_eq!(html.contains("Powered by"), shown);
}
