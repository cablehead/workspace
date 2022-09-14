import { createRef } from "preact";
import { useEffect, useRef } from "preact/hooks";

export function Editor(props) {
  const textarea = useRef();

  useEffect(() => {
    textarea.current.focus();
    textarea.current.select();
  }, []);

  const handler = (event) => {
    console.log("Editor", event);
    switch (true) {
      case event.ctrlKey && event.key == "r":
        console.log("run", textarea.current.value);
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
      style="height:100%; width:100%; resize: none;"
      {...props}
    />
  );
}
