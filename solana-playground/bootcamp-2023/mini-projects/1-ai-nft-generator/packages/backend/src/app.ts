import { Hono } from "hono";
import { logger } from "hono/logger";
import { router as nftRouter } from "./routes/nft";
import { serveStatic } from "@hono/node-server/serve-static";
import { cors } from "hono/cors";

const apiRouter = new Hono().route("/", nftRouter);

export const app = new Hono()
  .use(cors())
  .use(logger())
  .route("/api", apiRouter)
  .use(serveStatic({ root: "./public" }))
  .use(serveStatic({ path: "./public/index.html" }));

export type App = typeof app;
