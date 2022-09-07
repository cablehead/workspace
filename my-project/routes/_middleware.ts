import { MiddlewareHandlerContext } from "$fresh/server.ts";

export async function handler(
  req: Request,
  ctx: MiddlewareHandlerContext,
) {
  // console.log(req);
  const resp = await ctx.next();
  resp.headers.set("server", "fresh server");
  return resp;
}
