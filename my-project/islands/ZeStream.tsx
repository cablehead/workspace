import { useEffect, useReducer, useState } from "preact/hooks";
import { useSignal } from "@preact/signals";

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
  const [status, setStatus] = useState(DISCONNECTED);

  const [messages, addMessage] = useReducer<string[], string>(
    (msgs, msg) => [msg, ...msgs],
    [],
  );

  const inEdit = useSignal(false);
  const numMessages = useSignal(0);
  const preview = useSignal("...");

  const [selected, setSelected] = useState(0);

  const handler = (event) => {
    console.log(event);
    switch (true) {
      case event.key == "ArrowUp":
      case event.ctrlKey && event.key == "p":
        setSelected((x) => {
          if (x > 0) x -= 1;
          return x;
        });
        event.preventDefault();
        break;

      case event.ctrlKey && event.key == "n":
      case event.key == "ArrowDown":
        setSelected((x) => {
          if (x < numMessages.value - 1) return x + 1;
          return x;
        });
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
    numMessages.value = messages.length;
  }, [messages]);

  useEffect(() => {
    document.addEventListener("keydown", handler);
    return () => {
      document.removeEventListener("keydown", handler);
    };
  }, []);

  useEffect(() => {
    const item = document.getElementsByClassName("message-item")[selected];
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

  useEffect(() => {
    const events = new EventSource(props.source);
    setStatus(CONNECTING);

    events.addEventListener("open", () => setStatus(CONNECTED));
    events.addEventListener("error", () => {
      switch (events.readyState) {
        case EventSource.OPEN:
          setStatus(CONNECTED);
          break;
        case EventSource.CONNECTING:
          setStatus(CONNECTING);
          break;
        case EventSource.CLOSED:
          setStatus(DISCONNECTED);
          break;
      }
    });
    events.addEventListener("message", (e) => {
      let data = JSON.parse(e.data);
      addMessage(data);
    });
  }, []);

  return (
    <div style="display: flex; flex-direction: column; height:100%; overflow: auto">
      <p>Status: {status}</p>
      <div style="display: grid; height:100%; grid-template-columns: 40ch 1fr; overflow: auto; gap: 1em;">
        <div style="height: 100%; overflow: auto;">
          {messages.map((msg, i) => (
            <Item index={i} selected={selected} setSelected={setSelected}>
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
            {JSON.stringify(messages[selected], null, 4)}
          </div>
          {inEdit.value && (
            <Editor source={props.source} id={numMessages.value - selected} preview={ preview }>
              hi
            </Editor>
          )}
	  {inEdit.value && (
          <div style="white-space: pre; height: 100%; overflow: auto;">
            { preview }
          </div>
	  )}
        </div>
      </div>
    </div>
  );
}
