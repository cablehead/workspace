import { useEffect } from "preact/hooks";
import { useComputed, useSignal } from "@preact/signals";

import { Editor } from "../components/Editor.tsx";
import { Item } from "../components/Item.tsx";

const DISCONNECTED = "🔴 Disconnected";
const CONNECTING = "🟡 Connecting...";
const CONNECTED = "🟢 Connected";

function prepPreview(msg) {
  const plain = msg.types["public.utf8-plain-text"];
  if (plain != null) return atob(plain);
  return "n/a";
}

export default function ZeStream(props: PageProps) {
  const status = useSignal(DISCONNECTED);
  const messages = useSignal([]);
  const inEdit = useSignal(false);
  const preview = useSignal("...");
  const selected = useSignal(0);

  const handler = (event) => {
    console.log(event);
    switch (true) {
      case event.key == "ArrowUp":
      case event.ctrlKey && event.key == "p":
        if (selected.value > 0) selected.value--;
        event.preventDefault();
        break;

      case event.ctrlKey && event.key == "n":
      case event.key == "ArrowDown":
        if (selected.value < messages.value.length - 1) selected.value++;
        event.preventDefault();
        break;

      case event.key == "Enter":
        if (event.metaKey) {
          if (!inEdit.value) break;
          inEdit.value = false;
          event.preventDefault();
          break;
        }

        if (inEdit.value) break;
        inEdit.value = true;
        event.preventDefault();
        break;
    }
  };

  useEffect(() => {
    document.addEventListener("keydown", handler);
    return () => {
      document.removeEventListener("keydown", handler);
    };
  }, []);

  /*
  useEffect(() => {
    const item = document.getElementsByClassName("message-item")[selected.value];
    if (!item) return;

    const p = item.parentElement;
    const offsetTop = item.offsetTop - p.offsetTop;
    const offsetBottom = offsetTop + item.offsetHeight;
    const clientHeight = p.clientHeight - p.offsetTop;

    switch (true) {
      case (offsetTop < p.scrollTop):
        p.scrollTop = offsetTop;
        break;

      case (offsetBottom - p.scrollTop > clientHeight):
        p.scrollTop = offsetBottom - clientHeight;
        break;
    }
  }, [selected]);
  */

  useEffect(() => {
    const events = new EventSource(props.source);
    status.value = CONNECTING;

    events.addEventListener("open", () => status.value = CONNECTED);
    events.addEventListener("error", () => {
      switch (events.readyState) {
        case EventSource.OPEN:
          status.value = CONNECTED;
          break;
        case EventSource.CONNECTING:
          status.value = CONNECTING;
          break;
        case EventSource.CLOSED:
          status.value = DISCONNECTED;
          break;
      }
    });
    events.addEventListener("message", (e) => {
      let data = JSON.parse(e.data);
      messages.value = [data, ...messages.value];
    });
  }, []);

  return (
    <div style="display: flex; flex-direction: column; height:100%; overflow: auto">
      <p>Status: {status} Selected: {selected.value}</p>
      <div style="display: grid; height:100%; grid-template-columns: 40ch 1fr; overflow: auto; gap: 1em;">
        <div style="height: 100%; overflow: auto;">
          {messages.value.map((msg, i) => (
            <Item index={i} selected={selected}>
              {prepPreview(msg)}
            </Item>
          ))}
        </div>
        <div style="
		overflow: auto;
		display: grid;
		grid-auto-rows: 1fr;
		height:100%;
	">
          <div style="white-space: pre; height: 100%; overflow: auto;">
            {JSON.stringify(messages.value[selected.value], null, 4)}
          </div>
          {inEdit.value && (
            <Editor
              source={props.source}
              id={messages.value.length - selected.value}
              preview={preview}
            >
              hi
            </Editor>
          )}
          {inEdit.value && (
            <div style="white-space: pre; height: 100%; overflow: auto;">
              {preview}
            </div>
          )}
        </div>
      </div>
    </div>
  );
}
