#!/usr/bin/env -S deno run -A --watch=static/,routes/

import dev from "$fresh/dev.ts";

import { config } from "https://deno.land/x/dotenv/mod.ts";

config({ export: true });

await dev(import.meta.url, "./main.ts");
