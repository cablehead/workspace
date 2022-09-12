/** @jsx h */
import { h } from "preact";
import ZeStream from "../islands/ZeStream.tsx";

console.log("source:", Deno.env.get("API_HOST"));

export default function Home() {
  return (
    <main>
      <h2>A stream</h2>
      <div>
        <a href="/">home</a>
      </div>
      <ZeStream source={Deno.env.get("API_HOST")} />
    </main>
  );
}
