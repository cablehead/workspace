/** @jsx h */
import { h } from "preact";

export function Item(props) {
  return (
    <div
      style={{
        borderBottom: "1px solid #eee",
        overflow: "hidden",
        width: "40ch",
        lineHeight: "3em",
        height: "3em",
        paddingLeft: "1em",
        backgroundColor: props.selected && "#eee" || "#fff",
      }}
      {...props}
    />
  );
}
