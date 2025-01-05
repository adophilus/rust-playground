import { hc } from "hono/client";
import type { App } from "@nft-ai-generator/backend";
import { env } from "@nft-ai-generator/frontend/env";

export const backendClient = hc<App>(env.BACKEND_SERVER_URL);
