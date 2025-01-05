import { createEnv } from "@t3-oss/env-core";
import { z } from "zod";

export const env = createEnv({
  server: {
    PORT: z.coerce.number().min(0).max(65535),
    NODE_ENV: z.enum(["development", "production"]),
    HYPERBOLIC_API_KEY: z.string().min(1),
    DATABASE_URL: z.string().url(),
    UPLOADTHING_TOKEN: z.string(),
  },
  runtimeEnv: {
    NODE_ENV: process.env.NODE_ENV,
    PORT: process.env.PORT,
    HYPERBOLIC_API_KEY: process.env.HYPERBOLIC_API_KEY,
    DATABASE_URL: process.env.DATABASE_URL,
    UPLOADTHING_TOKEN: process.env.UPLOADTHING_TOKEN,
  },
});
