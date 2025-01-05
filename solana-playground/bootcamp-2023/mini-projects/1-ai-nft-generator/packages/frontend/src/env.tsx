import { z } from "zod";

export const env = z
  .object({
    REOWN_PROJECT_ID: z.string(),
    BACKEND_SERVER_URL: z.string().url(),
  })
  .parse({
    REOWN_PROJECT_ID: import.meta.env.VITE_REOWN_PROJECT_ID,
    BACKEND_SERVER_URL: import.meta.env.VITE_BACKEND_SERVER_URL,
  });
