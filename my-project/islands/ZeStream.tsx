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
  const [selected, setSelected] = useState(0);
  const handler = (event) => {
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
        console.log("messages.length:", messages.length);
        setSelected((x) => {
          if (x < messages.length - 1) return x + 1;
          return x;
        });
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
  }, []);

  return (
    <div>
      <p>Status: {status}</p>
      {messages.map((msg, i) => <Item selected={i == selected}>{msg}</Item>)}
    </div>
  );
}
