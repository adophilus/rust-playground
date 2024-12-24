import { int, sqliteTable, text, blob } from "drizzle-orm/sqlite-core";

export const nftsTable = sqliteTable("nfts", {
  id: text().primaryKey().notNull(),
  imageUrl: text().notNull(),
  prompt: text().notNull(),
  createdAt: int().notNull(),
  updatedAt: int(),
});
