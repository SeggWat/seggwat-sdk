//! SeggWat ideas board, roadmap and changelog as server-rendered Topcoat
//! components. Fetch with `seggwat::Seggwat` (feature `client`) and render.
//! Logged-out visitors need no script at all; signed-in visitors load the vote
//! module from the app's asset bundle.

use ::topcoat::{
    Result,
    asset::{Asset, asset},
    view::{Child, Unescaped, View, component, view},
};

pub use seggwat;
use seggwat::{
    ChangelogPage, Idea, IdeaList, Roadmap, Voting,
    ui::{category_badge, class, status_label},
};

/// The vote module. Boards with a token include it themselves.
pub const VOTE_JS: Asset = asset!("../assets/seggwat-board.js");

/// Votable ideas, most votes first.
#[component]
pub async fn idea_board(ideas: &IdeaList, voting: &Voting) -> Result<impl View> {
    Ok(view! {
        board(voting: voting,
            if ideas.items.is_empty() {
                <p class=(class::EMPTY)>"No ideas yet."</p>
            } else {
                <ul class=(class::LIST)>
                    for idea in &ideas.items {
                        idea_card(idea: idea, voting: voting, votable: true)
                    }
                </ul>
            }
        )
    })
}

/// Planned, in-progress and shipped ideas as three columns.
#[component]
pub async fn roadmap_board(roadmap: &Roadmap, voting: &Voting) -> Result<impl View> {
    let columns = [
        ("Planned", &roadmap.planned, true),
        ("In progress", &roadmap.started, true),
        ("Shipped", &roadmap.completed, false),
    ];
    Ok(view! {
        board(voting: voting,
            <div class=(class::COLUMNS)>
                for (title, list, votable) in columns {
                    <section>
                        <h2 class=(class::COLUMN_TITLE)>(title) <span class="badge badge-sm">(list.total)</span></h2>
                        if list.items.is_empty() {
                            <p class=(class::EMPTY)>"Nothing here yet."</p>
                        } else {
                            <ul class=(class::LIST)>
                                for idea in &list.items {
                                    idea_card(idea: idea, voting: voting, votable: votable)
                                }
                            </ul>
                        }
                    </section>
                }
            </div>
        )
    })
}

/// Published changelog entries, newest first. Bodies come from
/// [`seggwat::Seggwat::changelog`], which renders the Markdown safely.
#[component]
pub async fn changelog_feed(page: &ChangelogPage) -> Result<impl View> {
    Ok(view! {
        if page.items.is_empty() {
            <p class=(class::EMPTY)>"No updates yet."</p>
        } else {
            <div class=(class::FEED)>
                for entry in &page.items {
                    <article id=(&entry.id)>
                        <div class=(class::ENTRY_META)>
                            if let Some(date) = entry.date() {
                                <time datetime=(date)>(date)</time>
                            }
                            if let Some(version) = &entry.version {
                                <span class="badge badge-sm badge-outline">(version)</span>
                            }
                            if let Some(category) = &entry.category {
                                let (label, badge) = category_badge(category);
                                <span class=(format!("badge badge-sm {badge}"))>(label)</span>
                            }
                        </div>
                        <h2 class=(class::ENTRY_TITLE)>(&entry.title)</h2>
                        // Rendered by markdown-rs, which escapes raw HTML.
                        <div class=(class::PROSE)>(Unescaped::new_unchecked(entry.body_html.clone()))</div>
                    </article>
                }
            </div>
        }
    })
}

#[component]
async fn board(voting: &Voting, child: Child<'_>) -> Result<impl View> {
    Ok(view! {
        <seggwat-board
            data-api=(&voting.api_base)
            data-project=(&voting.project_key)
            data-token=(voting.token.as_deref())
        >
            (child)
        </seggwat-board>
        if voting.token.is_some() {
            <script type="module" src=(VOTE_JS)></script>
        }
    })
}

#[component]
async fn idea_card(idea: &Idea, voting: &Voting, votable: bool) -> Result<impl View> {
    Ok(view! {
        <li class=(class::CARD)>
            <div class=(class::CARD_BODY)>
                if votable {
                    vote_button(idea: idea, voting: voting)
                }
                <div class="min-w-0 flex-1">
                    <h3 class=(class::TITLE)>(&idea.title)</h3>
                    if !idea.description.is_empty() {
                        <p class=(class::DESCRIPTION)>(&idea.description)</p>
                    }
                    <div class=(class::META)>
                        <span class=(class::BADGE)>(status_label(&idea.status))</span>
                        if let Some(version) = &idea.completed_in_version {
                            <span class="badge badge-sm badge-outline">(version)</span>
                        }
                        for tag in &idea.tags {
                            <span class=(class::BADGE)>(&tag.name)</span>
                        }
                    </div>
                </div>
            </div>
        </li>
    })
}

#[component]
async fn vote_button(idea: &Idea, voting: &Voting) -> Result<impl View> {
    let label = format!("Vote for {}", idea.title);
    Ok(view! {
        if voting.token.is_some() {
            <button
                type="button"
                class=(class::VOTE)
                data-seggwat-vote=(&idea.id)
                aria-pressed="false"
                aria-label=(&label)
            >
                arrow()
                <span data-count="">(idea.votes_count)</span>
            </button>
        } else {
            <a href=(&voting.login_url) class=(class::VOTE) aria-label=(&label)>
                arrow()
                <span>(idea.votes_count)</span>
            </a>
        }
    })
}

#[component]
async fn arrow() -> Result<impl View> {
    Ok(view! {
        <svg class="size-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true">
            <path d=(class::ARROW_PATH)></path>
        </svg>
    })
}
