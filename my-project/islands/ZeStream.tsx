/** @jsx h  */
import { h } from "preact";
import { useEffect, useReducer, useState } from "preact/hooks";

import { Item } from "../components/Item.tsx";

const DISCONNECTED = "🔴 Disconnected";
const CONNECTING = "🟡 Connecting...";
const CONNECTED = "🟢 Connected";

export default function ZeStream(props: PageProps) {
  const [status, setStatus] = useState(DISCONNECTED);
  const [messages, addMessage] = useReducer<string[], string>(
    (msgs, msg) => {
      return [msg, ...msgs];
    },
    [],
  );
  const [selected, setSelected] = useState(1);

  const handler = (event) => {
    event.preventDefault();
    switch (event.key) {
      case "ArrowLeft":
        console.log("ArrowLeft");
        break;
      case "ArrowRight":
        console.log("ArrowRight");
        break;
      case "ArrowUp":
        setSelected((x) => {
          if (x > 0) x -= 1;
          return x;
        });
        break;
      case "ArrowDown":
        setSelected((x) => x + 1);
        break;
    }
  };

  useEffect(() => {
    console.log("useEffect", this, messages);
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
      let plain = atob(data.types["public.utf8-plain-text"]);
      addMessage(plain);
    });

    document.addEventListener("keydown", handler, true);
  }, []);

  return (
    <div>
      <p>Status: {status}</p>
      {messages.map((msg, i) => <Item selected={i == selected}>{msg}</Item>)}
    </div>
  );
}
