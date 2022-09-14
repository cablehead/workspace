import { createRef } from "preact";
import { useEffect, useRef } from "preact/hooks";

export function Editor(props) {
  const textarea = useRef();

  useEffect(() => {
    textarea.current.focus();
    textarea.current.select();
  }, []);

  return (
    <textarea ref={textarea} style="height:100%; width:100%; resize: none;" {...props} />
  );
}
