import { LitElement, html } from "./lit.js";

class EpistemologyElement extends LitElement {
  messages = [];

  pending = false;

  constructor() {
    super();
  }

  async sendMessage() {
    const input = this.querySelector("#user-input");
    const message = input.value;
    const context = this.querySelector("#context").value;
    let newMessages = [...this.messages];
    newMessages.push({
      role: "user",
      content: message,
    });
    //filter outSystem context
    newMessages = newMessages.filter((message) => message.role !== "system");

    this.messages = newMessages;
    this.requestUpdate();

    // add new system context to front
    newMessages.unshift({
      role: "system",
      content: context,
    });

    input.value = "";
    const urlHost = window.location.host;
    const urlPath = "/api/chat";
    const url = `https://${urlHost}${urlPath}`;
    this.pending = true;
    this.requestUpdate();
    const response = await this.callChat(url, newMessages);
    newMessages.push(response);
    this.messages = newMessages.filter((message) => message.role !== "system");
    this.pending = false;
    window.scrollTo(0, document.body.scrollHeight);
    this.requestUpdate();
  }

  clearMessages() {
    const input = this.querySelector("#user-input");
    this.pending = false;
    input.value = "";
    this.messages = [];
    this.requestUpdate();
  }

  async callChat(url, messages) {
    // fetch a streaming response using fetc
    const response = await fetch(url, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({ messages }),
    });

    const reader = response.body.getReader();
    let data = "";
    while (true) {
      const { done, value } = await reader.read();
      if (done) {
        break;
      }
      data += new TextDecoder().decode(value);
    }

    // turn data into json
    data = JSON.parse(data);

    return data;
  }

  createRenderRoot() {
    return this;
  }

  render() {
    return html`${this.messages.map(
        (message, i) =>
          html`<div style="${i !== 0 ? "margin-top: 1rem" : ""}">
            <div><b>${message.role}</b></div>
            <div>${message.content}</div>
          </div> `
      )}
      <div>
        <input
          id="context"
          type="text"
          placeholder="System context"
          autocomplete="off"
          spellcheck="false"
          autocorrect="off"
        />
      </div>
      <div>
        <input
          id="user-input"
          type="text"
          placeholder="Type a message"
          autocomplete="off"
          spellcheck="false"
          autocorrect="off"
        />
      </div>
      <div>
        <button @click="${this.sendMessage}">
          ${this.pending ? "Processing" : "Send"}
        </button>
      </div>
      <div style="margin-bottom: 1rem">
        <button @click="${this.clearMessages}">${"Clear"}</button>
      </div>`;
  }
}
customElements.define("epistemology-app", EpistemologyElement);
