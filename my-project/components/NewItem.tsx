import { useEffect, useRef } from "preact/hooks";

export function NewItem(props) {
  const textarea = useRef();

  useEffect(() => {
    textarea.current.focus();
    textarea.current.select();
  }, []);

  const handler = (event) => {
    console.log("Editor", event);
    switch (true) {
      case event.ctrlKey && event.key == "Enter":
        props.onDone(textarea.current.value);
        textarea.current.value = "";
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

  return (
    <textarea
      ref={textarea}
      style="
  position: fixed;
  top: 30vh;
  bottom: 30vh;
  left: 20vw;
  right: 20vw;
  background-color: #FFF;
  z-index: 2;
  resize: none;"
    >
    </textarea>
  );
}
