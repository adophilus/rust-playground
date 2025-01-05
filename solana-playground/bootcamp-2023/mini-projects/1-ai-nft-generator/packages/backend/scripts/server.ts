import { env } from "@/env";
import { handle } from "hono/vercel";
import { app } from "@/app";
import { serve } from "@hono/node-server";

const handler = handle(app);

export const GET = handler;
export const DELETE = handler;
export const OPTIONS = handler;
export const PATCH = handler;
export const PUT = handler;
export const POST = handler;

if (env.NODE_ENV === "development")
  serve(
    {
      fetch: app.fetch,
      port: env.PORT,
    },
    ({ port, address }) =>
      console.log(`Server is running on http://${address}:${port}`),
  );
