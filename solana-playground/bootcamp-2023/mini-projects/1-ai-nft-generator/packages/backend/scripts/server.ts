import { env } from "@/env";
import { app } from "@/app";
import { serve } from "@hono/node-server";

serve(
  {
    fetch: app.fetch,
    port: env.PORT,
  },
  ({ port, address }) =>
    console.log(`Server is running on http://${address}:${port}`),
);
