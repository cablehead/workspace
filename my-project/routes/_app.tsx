/** @jsx h */
import { h } from "preact";
import { asset, Head } from "$fresh/runtime.ts";
import { AppProps } from "$fresh/src/server/types.ts";

export default function App({ Component }: AppProps) {
  return (
    <html data-custom="data">
      <Head>
        <title>cross.stream</title>

	    <meta charset="utf-8" />
	    <meta name="viewport" content="width=device-width, initial-scale=1.0, maximum-scale=1.0, user-scalable=0" />
	    <meta name="apple-mobile-web-app-capable" content="yes" />
	    <link href="https://unpkg.com/sanitize.css" rel="stylesheet" />
	    <link href="https://unpkg.com/sanitize.css/forms.css" rel="stylesheet" />
	    <link href="https://unpkg.com/sanitize.css/assets.css" rel="stylesheet" />
	    <link href="https://unpkg.com/sanitize.css/reduce-motion.css" rel="stylesheet" />
	    <link href="https://unpkg.com/sanitize.css/system-ui.css" rel="stylesheet" />
	    <link href="https://unpkg.com/sanitize.css/ui-monospace.css" rel="stylesheet" />
	    <link href="https://unpkg.com/sanitize.css/typography.css" rel="stylesheet" />
        <link rel="stylesheet" href={asset("style.css")} />

      </Head>
      <body class="bodyClass">
        <Component />
      </body>
    </html>
  );
}
