import { createEnv } from "@t3-oss/env-core";
import { z } from "zod";

export const env = createEnv({
  server: {
    PORT: z.coerce.number().min(0).max(65535),
    HYPERBOLIC_API_KEY: z.string().min(1),
    DATABASE_URL: z.string().url(),
    DATABASE_AUTH_TOKEN: z.string().url(),
  },
  runtimeEnv: {
    PORT: process.env.PORT,
    HYPERBOLIC_API_KEY: process.env.HYPERBOLIC_API_KEY,
    DATABASE_URL: process.env.DATABASE_URL,
    DATABASE_AUTH_TOKEN: process.env.DATABASE_AUTH_TOKEN,
  },
});
