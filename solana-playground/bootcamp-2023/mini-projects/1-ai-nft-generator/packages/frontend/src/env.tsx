import { z } from "zod";

export const env = z
  .object({
    REOWN_PROJECT_ID: z.string(),
  })
  .parse({
    REOWN_PROJECT_ID: import.meta.env.VITE_REOWN_PROJECT_ID,
  });
