import { z } from "zod";

export const generateNft = z.object({
  prompt: z.string().nonempty(),
});
