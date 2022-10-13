import { useEffect, useRef } from "preact/hooks";
import { effect, useComputed, useSignal } from "@preact/signals";

import { Editor } from "../components/Editor.tsx";
import { NewItem } from "../components/NewItem.tsx";
import { Item } from "../components/Item.tsx";

const DISCONNECTED = "🔴 Disconnected";
const CONNECTING = "🟡 Connecting...";
const CONNECTED = "🟢 Connected";

function prepPreview(msg) {
  return msg;
  const plain = msg.types["public.utf8-plain-text"];
  if (plain != null) return atob(plain);
  return "n/a";
}

export default function ZeStream(props: PageProps) {
  const menu = useRef();

  const status = useSignal(DISCONNECTED);
  const messages = useSignal([]);
  const inEdit = useSignal(false);
  const inNew = useSignal(false);
  const preview = useSignal("...");

  const selected = useSignal(0);
  const selectedId = useComputed(() => messages.value.length - selected.value);

  const command = useSignal("cat");

  const handler = (event) => {
    console.log(event);
    switch (true) {
      case event.ctrlKey && event.key == "ArrowUp":
      case event.ctrlKey && event.key == "p":
        if (selected.value > 0) selected.value--;
        event.preventDefault();
        break;

      case event.ctrlKey && event.key == "ArrowDown":
      case event.ctrlKey && event.key == "n":
        if (selected.value < messages.value.length - 1) selected.value++;
        event.preventDefault();
        break;

      case event.ctrlKey && event.key == "Enter":
        if (!inNew.value) {
          inNew.value = !inNew.value;
          event.preventDefault();
        }
        break;

      case event.metaKey && event.key == "Enter":
        inEdit.value = !inEdit.value;
        event.preventDefault();
        break;
    }
  };

  const getNewItem = (value) => {
    console.log("getNewItem", value);
    inNew.value = false;
    if (value == "") return;
    const uri = `${props.source}`;
    console.log(uri);
    fetch(uri, {
      method: "PUT",
      body: value,
    });
  };

  useEffect(() => {
    document.addEventListener("keydown", handler);
    return () => {
      document.removeEventListener("keydown", handler);
    };
  }, []);

  useEffect(() => {
    return effect(() => {
      const p = menu.current;
      const item = p.children.item(selected.value);
      if (!item) return;

      const offsetTop = item.offsetTop - p.offsetTop;
      const offsetBottom = offsetTop + item.offsetHeight;
      const scrollBottom = p.scrollTop + p.clientHeight;

      if (offsetTop < p.scrollTop) {
        p.scrollTop = offsetTop;
        return;
      }

      if (offsetBottom > scrollBottom) {
        p.scrollTop = offsetBottom - p.clientHeight;
        return;
      }
    });
  }, []);

  useEffect(() => {
    return effect(() => {
      const id = selectedId.value;
      if (!inEdit.value) return;
      const uri = `${props.source}pipe/${id}`;
      console.log(uri);
      fetch(uri, {
        method: "POST",
        body: command.value,
      }).then((resp) =>
        resp.text().then((body) => {
          preview.value = body;
        })
      );
    });
  }, []);

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
      // let data = JSON.parse(e.data);
      messages.value = [e.data, ...messages.value];
    });
  }, []);

  return (
    <div style="display: flex; flex-direction: column; height:100%; overflow: auto">
      <p>Status: {status} Selected: {selected.value}</p>
      <div style="display: grid; height:100%; grid-template-columns: 40ch 1fr; overflow: auto; gap: 1em;">
        {inNew.value && <NewItem onDone={getNewItem} />}

        <div ref={menu} style="height: 100%; overflow: auto;">
          {messages.value.map((msg, i) => (
            <Item index={i} selected={selected}>
              {prepPreview(msg)}
            </Item>
          ))}
        </div>

        <div style={{
		overflow: "auto",
		display: "grid",
		gridTemplateColumns: "1fr" + (inEdit.value ? " 2fr" : ""),
		height: "100%",
	}}>
          <div style="
		overflow: auto;
		display: grid;
		grid-template-rows: 1fr;
		height:100%;
	">
            <div style="white-space: pre; height: 100%; overflow: auto;">
              {
                /*
	    JSON.stringify(messages.value[selected.value], null, 4)
	    */

                messages.value[selected.value]
              }
            </div>
            {inEdit.value && (
              <Editor
                command={command}
              />
            )}
          </div>

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
