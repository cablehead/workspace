import { useEffect, useRef } from "preact/hooks";
import { batch, effect, useComputed, useSignal } from "@preact/signals";

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
  const inMap = useSignal(false);
  const inEdit = useSignal(-1);
  const preview = useSignal("...");

  const selected = useSignal(0);

  const command = useSignal("cat");

  const handler = (event) => {
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

      case event.ctrlKey && event.key == "Backspace":
        let item = messages.value[selected.value];
        const uri = `${props.source}?` + new URLSearchParams({
          source_id: item.id,
          topic: "xs",
          attribute: ".delete",
        });
        fetch(uri, {
          method: "PUT",
        });
        event.preventDefault();
        break;

      // new item
      case event.ctrlKey && event.key == "Enter":
        if (inEdit.value === -1) {
          inEdit.value = 0;
          event.preventDefault();
        }
        break;

      // map item
      case event.metaKey && event.key == "Enter":
        inMap.value = !inMap.value;
        event.preventDefault();
        break;

      // edit item
      case event.key == "Enter":
        if (inEdit.value === -1) {
          console.log("hai");
          event.preventDefault();
        }
        break;
    }
  };

  const postEdit = (value) => {
    inEdit.value = -1;
    if (value == "") return;
    const uri = `${props.source}`;
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
      const id = messages.value.length > 0
        ? messages.value[selected.value].id
        : null;
      if (!inMap.value) return;
      const uri = `${props.source}pipe/${id}`;
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
      let data = JSON.parse(e.data);
      if (data.topic == "xs" && data.attribute == ".delete") {
        batch(() => {
          messages.value = messages.value.filter((item, _) =>
            item.id != data.source_id
          );
          if (selected.value >= messages.value.length) {
            selected.value = messages.value.length - 1;
          }
        });
        return;
      }
      messages.value = [data, ...messages.value];
    });
  }, []);

  return (
    <div style="display: flex; flex-direction: column; height:100%; overflow: auto">
      <p>
        Status: {status} ID:{" "}
        {messages.value.length > 0 ? messages.value[selected.value].id : 0}
      </p>
      <div style="display: grid; height:100%; grid-template-columns: 40ch 1fr; overflow: auto; gap: 1em;">
        {inEdit.value !== -1 && <NewItem onDone={postEdit} />}

        <div ref={menu} style="height: 100%; overflow: auto;">
          {messages.value.map((msg, i) => (
            <Item index={i} selected={selected}>
              {prepPreview(msg.data)}
            </Item>
          ))}
        </div>

        <div
          style={{
            overflow: "auto",
            display: "grid",
            gap: "1em",
            gridTemplateColumns: "1fr" + (inMap.value ? " 2fr" : ""),
            height: "100%",
          }}
        >
          <div style="
		overflow: auto;
		display: grid;
		grid-template-rows: 1fr;
		height:100%;
		gap: 1em;
	">
            <div style="white-space: pre; height: 100%; overflow: auto;">
              {messages.value.length > 0
                ? messages.value[selected.value].data
                : ""}
            </div>

            {inMap.value && (
              <Editor
                command={command}
              />
            )}
          </div>

          {inMap.value && (
            <div style="
	    white-space: pre; height: 100%; overflow: auto;
	    /* background-color: #efd5e5; */
	    ">
              {preview}
            </div>
          )}
        </div>
      </div>
    </div>
  );
}
