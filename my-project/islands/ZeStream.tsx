/** @jsx h  */
import { h } from "preact";
import { useEffect, useState, useReducer } from "preact/hooks";

const DISCONNECTED = "🔴 Disconnected";
const CONNECTING = "🟡 Connecting...";
const CONNECTED = "🟢 Connected";

export default function ZeStream(props: PageProps) {

console.log("props", props.source);
  const [status, setStatus] = useState(DISCONNECTED);
  const [messages, addMessage] = useReducer<string[], string>(
    (msgs, msg) => [...msgs, msg],
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
    	console.log(e);
      addMessage(e.data);
    });
  }, []);

  return (
    <div>
      <p>Status: {status}</p>
      <ul>
        {messages.map((msg) => (
          <li>
	  	foo {msg}
          </li>
        ))}
      </ul>
    </div>
  );
}
