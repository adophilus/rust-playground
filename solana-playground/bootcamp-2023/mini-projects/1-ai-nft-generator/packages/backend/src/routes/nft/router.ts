import { Hono } from "hono";
import { zValidator } from "@hono/zod-validator";
import { imageGenerator } from "@/lib/image-generator";
import * as Schema from "./schema";
import { storage } from "@/lib/storage";
import { db } from "@/lib/db";

const _router = new Hono()
  .post("/", zValidator("json", Schema.generateNft), async (c) => {
    const payload = c.req.valid("json");

    const image = await imageGenerator.generateImage(payload.prompt);
    const imageUrl = await storage.uploadFile(image);

    const nft = await db
      .insertInto("nfts")
      .values({
        id: crypto.randomUUID(),
        image_url: imageUrl,
        prompt: payload.prompt,
        created_at: Date.now(),
      })
      .returningAll()
      .executeTakeFirstOrThrow();

    return c.json(nft, 200);
  })
  .get("/:id", async (c) => {
    const id = c.req.param("id");

    const nft = await db
      .selectFrom("nfts")
      .selectAll()
      .where("nfts.id", "=", id)
      .executeTakeFirst();

    if (!nft) return c.json({ error: "NFT not found" }, 404);

    return c.json(nft, 200);
  });

export const router = new Hono().route("/nfts", _router);