import { useComputed } from "@preact/signals";

export function Item(props) {
  const isSelected = useComputed(() => props.selected.value == props.index);
  return (
    <div
      class="message-item"
      style={{
        borderBottom: "1px solid #eee",
        overflow: "hidden",
        lineHeight: "2.5em",
        height: "2.5em",
        backgroundColor: isSelected.value && "#eee" || "#fff",
      }}
      onClick={() => {
        props.selected.value = props.index;
      }}
      {...props}
    />
  );
}
