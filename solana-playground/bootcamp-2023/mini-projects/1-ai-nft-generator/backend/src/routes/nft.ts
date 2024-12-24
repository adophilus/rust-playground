import { Hono } from "hono";
import { zValidator } from "@hono/zod-validator";
import { HyperbolicService } from "@/lib/hyperbolic";
import * as Schema from "./schema";
import { UploadThingService } from "@/lib/uploadthing";
import { db, nftsTable } from "@/lib/db";

export const router = new Hono().post(
  "/",
  zValidator("json", Schema.generateNft),
  async (c) => {
    const payload = c.req.valid("json");

    const image = await HyperbolicService.generateImage(payload.prompt);
    const imageUrl = await UploadThingService.uploadFile(image);

    const [nft] = await db
      .insert(nftsTable)
      .values({
        id: crypto.randomUUID(),
        imageUrl,
        prompt: payload.prompt,
        createdAt: Date.now(),
      })
      .returning();

    return c.json(nft, 200);
  },
);
