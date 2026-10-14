# SeggWat SDK

Put your [SeggWat](https://seggwat.com) feature voting, roadmap and changelog
inside your own app instead of linking to the hosted board. SeggWat keeps the
data, and your pages are ordinary server-rendered HTML in your own layout,
indexable and styled like the rest of the site.

| package | what it is |
|---|---|
| [`seggwat`](rust/seggwat) | Rust: public board API types; feature `client` adds a cached server-side client, safe Markdown rendering and end-user token signing |
| [`seggwat-topcoat`](rust/seggwat-topcoat) | Rust: `idea_board`, `roadmap_board`, `changelog_feed` for [Topcoat](https://github.com/tokio-rs/topcoat) |
| [`seggwat-dioxus`](rust/seggwat-dioxus) | Rust: `IdeaBoard`, `RoadmapBoard`, `ChangelogFeed` for Dioxus 0.7 |
| [`@seggwat/board`](js/board) | the `<seggwat-board>` web component that makes vote buttons work, in any framework |

Leptos, React and a Node/Bun server client are planned. They follow the same
pattern, so the web component stays the same.

## How it works

```
your server ──GET (anonymous, cached 60s)──▶ seggwat.com/api/v1/p/{project}/roadmap
     │  renders HTML + signs a 1h end-user JWT for the logged-in user
     ▼
browser <seggwat-board data-token=…> ──vote/unvote with token──▶ seggwat.com
```

- **Reads run on your server.** They go out anonymously and are cached, so one
  response serves every visitor. SeggWat rate-limits public reads per client IP,
  and your whole server counts as one IP.
- **Votes run in the browser**, straight to SeggWat with a token your server
  signed for the user. Each visitor keeps their own rate limit. Proxying votes
  through your server would put all your users in one 5-votes/minute bucket.
- **Logged-out visitors** get vote buttons that link to your login page, and no
  script at all.

## SeggWat project setup

1. Project settings → **End-User Authentication** → *Signed tokens (JWT)* → generate the signing secret.
2. Add your site's origin (e.g. `https://example.com`) to the project's **allowed origins**.
3. Set `SEGGWAT_PROJECT_KEY` and `SEGGWAT_SIGNING_SECRET` on your server
   (`SEGGWAT_API_URL` is optional, default `https://seggwat.com`).
   `Seggwat::from_env(site_origin)` reads them and returns `None` without a project key.

## Topcoat

```toml
seggwat = { version = "0.1", features = ["client"] }
seggwat-topcoat = "0.1"
```

```rust
use seggwat::{Seggwat, Viewer};
use seggwat_topcoat::roadmap_board;

// your public origin; None when SEGGWAT_PROJECT_KEY is unset
static SEGGWAT: LazyLock<Option<Seggwat>> =
    LazyLock::new(|| Seggwat::from_env("https://example.com"));
let Some(seggwat) = SEGGWAT.as_ref() else { /* board disabled */ };

let viewer = user.map(|u| Viewer { id: u.id, email: Some(u.email), name: u.name });
let voting = seggwat.voting(viewer.as_ref(), "/login?next=/roadmap");
let roadmap = seggwat.roadmap().await?;
view! { roadmap_board(roadmap: &roadmap, voting: &voting) }
```

## Dioxus fullstack

Depend on `seggwat-dioxus` everywhere and enable `seggwat/client` only in the
server build:

```toml
seggwat = "0.1"
seggwat-dioxus = "0.1"

[features]
server = ["dioxus/server", "seggwat/client"]
```

```rust
#[get("/api/roadmap")]
async fn roadmap_data() -> Result<(Roadmap, Voting), ServerFnError> {
    // let viewer = <your session user>;
    let seggwat = SEGGWAT.as_ref().ok_or_else(|| ServerFnError::new("seggwat not configured"))?;
    Ok((seggwat.roadmap().await?, seggwat.voting(viewer.as_ref(), "/login")))
}

#[component]
fn RoadmapPage() -> Element {
    let data = use_server_future(roadmap_data)?;
    match data() {
        Some(Ok((roadmap, voting))) => rsx! { RoadmapBoard { roadmap, voting } },
        _ => rsx! {},
    }
}
```

## Notes

- A page that carries a viewer's token belongs to that user. Don't put it in a shared cache.
- The components use Tailwind + DaisyUI classes, and changelog bodies use `prose`
  (`@tailwindcss/typography`). When the crate comes from crates.io or git, add
  its `safelist.html` to your Tailwind `@source`s.

## Development

`just check` runs what CI runs. The vote module's source is
`js/board/seggwat-board.js`; after editing it, run `just sync-js` to copy it
into the Rust crates (a test fails if the copies drift).

## License

MIT
