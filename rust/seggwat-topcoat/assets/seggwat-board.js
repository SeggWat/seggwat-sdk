// <seggwat-board>: vote buttons for a server-rendered SeggWat board, shared by
// every framework package. The server renders the board and the counts; this
// module only marks the viewer's own votes and sends vote/unvote straight to
// SeggWat, so each visitor is rate-limited on their own IP.
//
// <seggwat-board data-api data-project data-token>
//   <button data-seggwat-vote="IDEA_ID" aria-pressed="false"><span data-count>3</span></button>
//
// Without a token the server renders plain login links and this does nothing.

class SeggwatBoard extends HTMLElement {
  connectedCallback() {
    const { api, project, token } = this.dataset;
    if (!token || this.base) return;
    this.base = `${api}/api/v1/p/${encodeURIComponent(project)}`;
    this.token = token;
    this.addEventListener("click", (event) => {
      const button = event.target.closest("[data-seggwat-vote]");
      if (button && this.contains(button)) this.toggle(button);
    });
    this.loadOwnVotes();
  }

  buttons(id) {
    return this.querySelectorAll(`[data-seggwat-vote="${CSS.escape(id)}"]`);
  }

  render(id, voted, count) {
    for (const button of this.buttons(id)) {
      button.setAttribute("aria-pressed", String(voted));
      const label = button.querySelector("[data-count]");
      if (label && count !== undefined) label.textContent = String(count);
    }
  }

  async loadOwnVotes() {
    const url = `${this.base}/ideas?limit=100&token=${encodeURIComponent(this.token)}`;
    try {
      const response = await fetch(url);
      if (!response.ok) return;
      const { items } = await response.json();
      for (const idea of items) {
        this.render(idea.id, idea.user_has_voted, idea.votes_count);
      }
    } catch {
      // Counts stay as rendered; voting still works.
    }
  }

  async toggle(button) {
    const id = button.dataset.seggwatVote;
    const voted = button.getAttribute("aria-pressed") === "true";
    const label = button.querySelector("[data-count]");
    const count = Number(label?.textContent) || 0;
    const next = voted ? Math.max(0, count - 1) : count + 1;

    this.render(id, !voted, next);
    button.disabled = true;
    const url = `${this.base}/ideas/${encodeURIComponent(id)}/vote`;
    try {
      const response = voted
        ? await fetch(`${url}?token=${encodeURIComponent(this.token)}`, { method: "DELETE" })
        : await fetch(url, {
            method: "POST",
            headers: { "content-type": "application/json" },
            body: JSON.stringify({ token: this.token }),
          });
      if (!response.ok) throw new Error(String(response.status));
    } catch {
      this.render(id, voted, count);
    } finally {
      button.disabled = false;
    }
  }
}

if (!customElements.get("seggwat-board")) {
  customElements.define("seggwat-board", SeggwatBoard);
}
