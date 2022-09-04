/** @jsx h  */
import { h } from "preact";
import { useEffect, useState, useReducer } from "preact/hooks";

import { Item } from "../components/Item.tsx";

const DISCONNECTED = "🔴 Disconnected";
const CONNECTING = "🟡 Connecting...";
const CONNECTED = "🟢 Connected";

export default function ZeStream(props: PageProps) {
  const [status, setStatus] = useState(DISCONNECTED);
  const [messages, addMessage] = useReducer<string[], string>(
    (msgs, msg) => [msg, ...msgs],
    [],
  );

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
        {messages.map((msg, i) => (
	<Item selected={ i==1}>{ msg }</Item>
        ))}
    </div>
  );
}
