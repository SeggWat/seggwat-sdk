# @seggwat/board

The `<seggwat-board>` web component. It makes the vote buttons on a
server-rendered SeggWat board work: it marks the viewer's own votes and sends
vote/unvote from the browser straight to SeggWat.

The Rust packages ship this file already. Use this package when you render a
board yourself, from any framework.

## Markup contract

```html
<seggwat-board data-api="https://seggwat.com" data-project="PROJECT_KEY" data-token="END_USER_JWT">
  <button type="button" data-seggwat-vote="IDEA_ID" aria-pressed="false">
    <span data-count>12</span>
  </button>
</seggwat-board>
<script type="module" src="seggwat-board.js"></script>
```

- `data-token` is an HS256 JWT your server signs with the project's end-user
  signing secret (`sub`, `aud` = `project_key` = the project key, `iat`, `exp`).
  Leave out the token and the script for logged-out visitors, and render
  login links instead.
- The script sets `aria-pressed` and updates `[data-count]`. Style the voted
  state with `[aria-pressed="true"]`.
- Your site's origin must be in the project's allowed origins.
