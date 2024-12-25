import type { Kysely } from "kysely";

export async function up(db: Kysely<any>): Promise<void> {
  await db.schema
    .createTable("nfts")
    .addColumn("id", "text", (col) => col.primaryKey().notNull())
    .addColumn("image_url", "text", (col) => col.notNull())
    .addColumn("prompt", "text", (col) => col.notNull())
    .addColumn("created_at", "integer", (col) => col.notNull())
    .addColumn("updated_at", "integer")
    .execute();
}

export async function down(db: Kysely<any>): Promise<void> {
  await db.schema.dropTable("nfts").execute();
}
