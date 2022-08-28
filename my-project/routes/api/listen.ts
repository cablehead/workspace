/** @jsx h  */
import { Handlers, RouteConfig } from "$fresh/server.ts";

 export const handler: Handlers = {
 GET(_req, ctx) {

  console.log("hi");
  console.log("hi");

  var timer;

  const stream = new ReadableStream({
    start: (controller) => {
  console.log("start");
      controller.enqueue(": Welcome to Deno Deploy Chat!\n\n");

    timer = setInterval(() => {
      controller.enqueue("event: oh\ndata: {}\n\n");
      console.log("1");
    }, 1000);

    },
    cancel() {
    	clearInterval(timer);
      console.log("cancel");
    },
  });

  return new Response(stream.pipeThrough(new TextEncoderStream()), {
    headers: { "content-type": "text/event-stream" },
  });
  },
};
