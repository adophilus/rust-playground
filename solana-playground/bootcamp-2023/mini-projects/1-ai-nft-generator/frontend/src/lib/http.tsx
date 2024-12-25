import { hc } from "hono/client";
import type { App } from "@nft-ai-generator/backend";

export const backendClient = hc<App>("/");
