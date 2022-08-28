/** @jsx h */
import { h } from "preact";
import ZeStream from "../islands/ZeStream.tsx";

export default function Home() {
  return (
    <div>
      <h2>A stream</h2>
      <div><a href="/">home</a></div>
      <ZeStream />
    </div>
  );
}
