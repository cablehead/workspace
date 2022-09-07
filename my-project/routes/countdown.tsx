/** @jsx h */
import { h } from "preact";
import Countdown from "../islands/Countdown.tsx";

export default function Page() {
  const date = new Date();
  date.setHours(date.getHours() + 1);
  return (
    <p>
      <div>
        <a href="/">home</a>
      </div>
      The big event is happening <Countdown target={date.toISOString()} />.
    </p>
  );
}
