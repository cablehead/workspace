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

  const [inEdit, setInEdit] = useState(false);

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
          if (x < messages.length - 1) return x + 1;
          return x;
        });
        event.preventDefault();
        break;

      case event.key == "Enter":
        // arg, need to subscribe to inEdit here, similar to subscribe to message
        // length
        console.log("inEdit", inEdit);
        if (inEdit) break;
        setInEdit(true);
        event.preventDefault();
        break;
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
        <div style="height: 100%; overflow: auto; display: grid; grid-template-columns: 1fr;">
          <pre style="height: 100%;">{JSON.stringify(messages[selected], null, 4)}</pre>

          {inEdit && (
            <div>
              <textarea style="height:100%; width:100%; resize: none;">
                hi
              </textarea>
            </div>
          )}
        </div>
      </div>
    </div>
  );
}
