import { Hono } from "hono";
import { logger } from "hono/logger";
import { router as nftRouter } from "./routes/nft";
import { serveStatic } from "@hono/node-server/serve-static";

export const app = new Hono()
  .use(logger())
  .route("/nft", nftRouter)
  .use(serveStatic({ root: "./public" }))
  .use(serveStatic({ path: "./public/index.html" }));

export type App = typeof app;
