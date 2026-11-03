# @seggwat/chat

The `<seggwat-chat>` web component: SeggWat live support chat inside your own
page, for signed-in users. It loads the chat module hosted by SeggWat (the same
one the feedback widget uses) and mounts it in place. Your team answers from
the SeggWat dashboard under **Messages → Live chat**.

The Rust packages ship this file already. Use this package when you render the
chat yourself, from any framework.

## Markup contract

```html
<seggwat-chat data-api="https://seggwat.com" data-project="PROJECT_KEY" data-token="END_USER_JWT">
  Loading chat…
</seggwat-chat>
<script type="module" src="seggwat-chat.js"></script>
```

- Turn on **Live Chat** in the project's widget settings (Pro plan).
- `data-token` is the same HS256 end-user JWT the board uses (`sub`, `aud` =
  `project_key` = the project key, `iat`, `exp`). Logged-out visitors get no
  chat: render a login link instead of the element.
- The element is `32rem` tall by default; size it with CSS. The accent colour
  follows `--seggwat-hub-accent-from`.
- One chat per page. Your site's origin must be in the project's allowed origins.
