//! SeggWat ideas board, roadmap and changelog as Dioxus components, with the
//! same markup as `seggwat-topcoat`. Fetch the data on the server (server
//! function or `use_server_future`) and pass it in; the components only
//! render. The vote module updates button state in place, so keep the props
//! stable once the board is shown.

use ::dioxus::prelude::*;

pub use seggwat;
use seggwat::{
    ChangelogPage, Idea, IdeaList, Roadmap, Voting,
    ui::{category_badge, class, status_label},
};

const VOTE_JS: Asset = asset!("/assets/seggwat-board.js");

/// Votable ideas, most votes first.
#[component]
pub fn IdeaBoard(ideas: IdeaList, voting: Voting) -> Element {
    rsx! {
        Board { voting: voting.clone(),
            if ideas.items.is_empty() {
                p { class: class::EMPTY, "No ideas yet." }
            } else {
                ul { class: class::LIST,
                    for idea in ideas.items {
                        IdeaCard { key: "{idea.id}", idea, voting: voting.clone(), votable: true }
                    }
                }
            }
        }
    }
}

/// Planned, in-progress and shipped ideas as three columns.
#[component]
pub fn RoadmapBoard(roadmap: Roadmap, voting: Voting) -> Element {
    let columns = [
        ("Planned", roadmap.planned, true),
        ("In progress", roadmap.started, true),
        ("Shipped", roadmap.completed, false),
    ];
    rsx! {
        Board { voting: voting.clone(),
            div { class: class::COLUMNS,
                for (title, list, votable) in columns {
                    section { key: "{title}",
                        h2 { class: class::COLUMN_TITLE,
                            "{title} "
                            span { class: "badge badge-sm", "{list.total}" }
                        }
                        if list.items.is_empty() {
                            p { class: class::EMPTY, "Nothing here yet." }
                        } else {
                            ul { class: class::LIST,
                                for idea in list.items {
                                    IdeaCard { key: "{idea.id}", idea, voting: voting.clone(), votable }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Published changelog entries, newest first. Bodies come from
/// [`seggwat::Seggwat::changelog`], which renders the Markdown safely.
#[component]
pub fn ChangelogFeed(page: ChangelogPage) -> Element {
    if page.items.is_empty() {
        return rsx! { p { class: class::EMPTY, "No updates yet." } };
    }
    rsx! {
        div { class: class::FEED,
            for entry in page.items {
                article { key: "{entry.id}", id: "{entry.id}",
                    div { class: class::ENTRY_META,
                        if let Some(date) = entry.date() {
                            time { datetime: "{date}", "{date}" }
                        }
                        if let Some(version) = &entry.version {
                            span { class: "badge badge-sm badge-outline", "{version}" }
                        }
                        if let Some(category) = &entry.category {
                            CategoryBadge { category: category.clone() }
                        }
                    }
                    h2 { class: class::ENTRY_TITLE, "{entry.title}" }
                    // Rendered by markdown-rs, which escapes raw HTML.
                    div { class: class::PROSE, dangerous_inner_html: entry.body_html }
                }
            }
        }
    }
}

#[component]
fn CategoryBadge(category: String) -> Element {
    let (label, badge) = category_badge(&category);
    rsx! { span { class: "badge badge-sm {badge}", "{label}" } }
}

#[component]
fn Board(voting: Voting, children: Element) -> Element {
    rsx! {
        if voting.token.is_some() {
            document::Script { r#type: "module", src: VOTE_JS }
        }
        seggwat-board {
            "data-api": voting.api_base,
            "data-project": voting.project_key,
            "data-token": voting.token,
            {children}
        }
    }
}

#[component]
fn IdeaCard(idea: Idea, voting: Voting, votable: bool) -> Element {
    rsx! {
        li { class: class::CARD,
            div { class: class::CARD_BODY,
                if votable {
                    VoteButton { idea: idea.clone(), voting }
                }
                div { class: "min-w-0 flex-1",
                    h3 { class: class::TITLE, "{idea.title}" }
                    if !idea.description.is_empty() {
                        p { class: class::DESCRIPTION, "{idea.description}" }
                    }
                    div { class: class::META,
                        span { class: class::BADGE, {status_label(&idea.status)} }
                        if let Some(version) = &idea.completed_in_version {
                            span { class: "badge badge-sm badge-outline", "{version}" }
                        }
                        for tag in &idea.tags {
                            span { key: "{tag.slug}", class: class::BADGE, "{tag.name}" }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn VoteButton(idea: Idea, voting: Voting) -> Element {
    let label = format!("Vote for {}", idea.title);
    if voting.token.is_some() {
        rsx! {
            button {
                r#type: "button",
                class: class::VOTE,
                "data-seggwat-vote": idea.id,
                aria_pressed: "false",
                aria_label: label,
                Arrow {}
                span { "data-count": "", "{idea.votes_count}" }
            }
        }
    } else {
        rsx! {
            a { href: voting.login_url, class: class::VOTE, aria_label: label,
                Arrow {}
                span { "{idea.votes_count}" }
            }
        }
    }
}

#[component]
fn Arrow() -> Element {
    rsx! {
        svg {
            class: "size-4",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "2",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            "aria-hidden": "true",
            path { d: class::ARROW_PATH }
        }
    }
}
