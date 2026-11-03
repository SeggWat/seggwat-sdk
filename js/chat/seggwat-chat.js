// <seggwat-chat>: SeggWat live support chat inside your own page, shared by
// every framework package. It loads the chat module hosted by SeggWat (the
// same one the feedback widget opens), so chat fixes ship without an SDK
// release, and mounts it here. Messages go from the browser straight to
// SeggWat with a token your server signed for the signed-in user.
//
// <seggwat-chat data-api data-project data-token>…fallback…</seggwat-chat>
//
// The element is 32rem tall by default; size it with CSS. One chat per page:
// the hosted module holds a single conversation.

const STYLE_ID = "seggwat-chat-inline-css";

// Inline overrides for the hosted styles, which size the thread for the
// widget's popup. Element-qualified so they win over the popup's media query.
const INLINE_CSS = `
seggwat-chat { display: block; height: 32rem; }
seggwat-chat .seggwat-chat-thread { height: 100%; max-height: none; min-height: 0; }
`;

let loading;

function loadModule(api) {
  if (window.SeggWatChatModule) return Promise.resolve();
  loading ||= new Promise((resolve, reject) => {
    // Same id as the feedback widget's loader, so a page with both loads it once.
    if (!document.getElementById("seggwat-chat-css")) {
      const css = document.createElement("link");
      css.id = "seggwat-chat-css";
      css.rel = "stylesheet";
      css.href = `${api}/static/widgets/v1/seggwat-chat.css`;
      document.head.append(css);
    }
    const script = document.createElement("script");
    script.src = `${api}/static/widgets/v1/seggwat-chat.js`;
    script.onload = () =>
      window.SeggWatChatModule ? resolve() : reject(new Error("chat module missing"));
    script.onerror = () => reject(new Error("chat module failed to load"));
    document.head.append(script);
  });
  return loading;
}

class SeggwatChat extends HTMLElement {
  async connectedCallback() {
    const { api, project, token } = this.dataset;
    if (!token || this.mounted) return;
    if (!document.getElementById(STYLE_ID)) {
      const style = document.createElement("style");
      style.id = STYLE_ID;
      style.textContent = INLINE_CSS;
      document.head.append(style);
    }
    try {
      await loadModule(api);
    } catch (error) {
      console.error("[SeggWat Chat]", error);
      return; // keep the server-rendered fallback
    }
    if (!this.isConnected) return;
    this.mounted = true;
    window.SeggWatChatModule.mount(this, {
      apiUrl: api,
      projectKey: project,
      token,
      onTokenRejected: () => {
        this.textContent = "Your session has expired. Reload the page to keep chatting.";
      },
    });
  }

  disconnectedCallback() {
    if (!this.mounted) return;
    this.mounted = false;
    window.SeggWatChatModule.unmount();
  }
}

if (!customElements.get("seggwat-chat")) {
  customElements.define("seggwat-chat", SeggwatChat);
}
