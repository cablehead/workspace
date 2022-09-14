export function Item(props) {
  return (
    <div
      class="message-item"
      style={{
        borderBottom: "1px solid #eee",
        overflow: "hidden",
        lineHeight: "2.5em",
        height: "2.5em",
        backgroundColor: (props.selected == props.index) && "#eee" || "#fff",
      }}
      onClick={() => props.setSelected(props.index)}
      {...props}
    />
  );
}
