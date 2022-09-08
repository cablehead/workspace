/** @jsx h */
import { h } from "preact";

export function Item(props) {
  return (
    <div
      style={{
        borderBottom: "1px solid #eee",
        overflow: "hidden",
        lineHeight: "3em",
        height: "3em",
        backgroundColor: (props.selected == props.index) && "#eee" || "#fff",
      }}
      onClick={()=> props.setSelected(props.index)}
      {...props}
    />
  );
}
