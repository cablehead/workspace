import Counter from "../islands/Counter.tsx";

export default function Home() {
  return (
    <main>
    <div>
      <img
        src="/logo.svg"
        height="100px"
        alt="the fresh logo: a sliced lemon dripping with juice"
      />
      </div>
      <p>
        Wha? I need to refresh? Oh, phew
      </p>
      <div style="display: flex; gap: 10px;">
        <a href="/countdown">countdown</a>
        <a href="/stream">stream</a>
        <a href="/github/cablehead">cablehead</a>
        <a href="/greet/me">greet me</a>
        <a href="/search">search</a>
        <a href="/about">about</a>
      </div>
      <Counter start={3} />
    </main>
  );
}
