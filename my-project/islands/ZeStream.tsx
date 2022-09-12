/** @jsx h  */
import { h } from "preact";
import { useEffect, useReducer, useState } from "preact/hooks";

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

  const [selected, setSelected] = useState(0);

  const [inEdit, setInEdit] = useState(true);

  const handler = (event) => {
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
          if (x < messages.length - 1) return x + 1;
          return x;
        });
        event.preventDefault();
        break;

      case event.key == "Enter":
        setEdit(true);
        event.preventDefault();
        break;

      default:
        console.log(event);
    }
  };

  useEffect(() => {
    document.addEventListener("keydown", handler);
    return () => {
      document.removeEventListener("keydown", handler);
    };
  }, [messages]);

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
    <div style="display: grid; height:100%; overflow: auto">
      <p>Status: {status}</p>
      <div style="display: grid; height:100%; grid-template-columns: 40ch 1fr; overflow: auto">
        <div style="overflow: auto">
          {messages.map((msg, i) => (
            <Item index={i} selected={selected} setSelected={setSelected}>
              {prepPreview(msg)}
            </Item>
          ))}
        </div>
        <div style="overflow: auto">
          <pre>{JSON.stringify(messages[selected], null, 4)}</pre>
        </div>
      </div>
    </div>
  );
}
