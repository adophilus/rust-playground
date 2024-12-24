import { Hono } from "hono";
import { logger } from "hono/logger";
import { router as nftRouter } from "./routes/nft";

export const app = new Hono().use(logger()).route("/nft", nftRouter);

export type App = typeof app;
