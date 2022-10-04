export default function Page() {
  const date = new Date();
  date.setHours(date.getHours() + 1);
  return (
    <div>
      <div>
        <a href="/">home</a>
      </div>

      <div style="
      position: relative;
      font-size: 6em;
      width: 6.7em; height: 8.3em;

      background-image:
      repeating-linear-gradient(to top, #ccc 0 0.01em, transparent 1px 1.19em),
      repeating-linear-gradient(90deg, #ccc 0 0.01em, transparent 1px 1.19em);
      background-color: blue;
      ">

      <div style="
      position: absolute;
      margin-top: 1.2em;
      margin-left: 0.6em;
      width: 4.8em; height: 6em; background-color: green;">
      </div>

      <div style="
      position: absolute;
      margin-top: 1.5em;
      margin-left: 0.6em;
      width: 2.39em; height: 0.125em; background-color: white;">
      </div>

      <div style="
      position: absolute;
      margin-top: 1.5em;
      margin-left: 3em;
      width: 2.39em; height: 0.125em; background-color: white;">
      </div>

      <div style="
      position: absolute;
      margin-top: 3.3em;
      margin-left: 3em;
      width: 2.39em; height: 0.125em; background-color: white;">
      </div>

      <div style="
      position: absolute;
      margin-top: 2.7em;
      margin-left: 0.6em;
      width: 2.39em; height: 0.125em; background-color: white;">
      </div>

      <div style="
      position: absolute;
      margin-top: 3.9em;
      margin-left: 0.6em;
      width: 2.39em; height: 0.125em; background-color: white;">
      </div>

      <div style="
      position: absolute;
      margin-top: 5.1em;
      margin-left: 0.6em;
      width: 2.39em; height: 0.125em; background-color: white;">
      </div>

      <div style="
      position: absolute;
      margin-top: 5.1em;
      margin-left: 3em;
      width: 2.39em; height: 0.125em; background-color: white;">
      </div>

      <div style="
      position: absolute;
      margin-top: 5.3em;
      margin-left: 0.6em;
      width: 2.39em; height: 0.325em; background-color: white;">
      </div>

      <div style="
      position: absolute;
      margin-top: 5.7em;
      margin-left: 0.6em;
      width: 2.39em; height: 0.325em; background-color: white;">
      </div>

      <div style="
      position: absolute;
      margin-top: 6.1em;
      margin-left: 0.6em;
      width: 2.39em; height: 0.725em; background-color: white;">
      </div>

      <div style="
      position: absolute;
      margin-top: 6.9em;
      margin-left: 0.6em;
      width: 2.39em; height: 0.725em; background-color: white;">
      </div>

      <div style="
      position: absolute;
      margin-top: 5.3em;
      margin-left: 3em;
      width: 2.39em; height: 0.325em; background-color: white;">
      </div>

      <div style="
      position: absolute;
      margin-top: 5.7em;
      margin-left: 3em;
      width: 2.39em; height: 0.325em; background-color: white;">
      </div>

      <div style="
      position: absolute;
      margin-top: 6.1em;
      margin-left: 3em;
      width: 2.39em; height: 0.725em; background-color: white;">
      </div>

      <div style="
      position: absolute;
      margin-top: 6.9em;
      margin-left: 3em;
      width: 2.39em; height: 0.725em; background-color: white;">
      </div>

      <div name="pegboard" style="
      display: none;
      position: absolute;
      margin-top: 3.5em;
      margin-left: 3em;
      width: 2.39em; height: 1.5em; background-color: white;">
      </div>

      </div>
    </div>
  );
}
