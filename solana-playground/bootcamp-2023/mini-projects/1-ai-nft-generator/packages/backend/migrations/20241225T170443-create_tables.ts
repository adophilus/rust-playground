import { sql, type Kysely } from "kysely";

export async function up(db: Kysely<any>): Promise<void> {
  await db.schema
    .createTable("nfts")
    .addColumn("id", "varchar", (col) => col.primaryKey().notNull())
    .addColumn("image_url", "varchar", (col) => col.notNull())
    .addColumn("prompt", "varchar", (col) => col.notNull())
    .addColumn("created_at", "timestamptz", (col) => col.notNull().defaultTo(sql`NOW()`))
    .addColumn("updated_at", "timestamptz")
    .execute();
}

export async function down(db: Kysely<any>): Promise<void> {
  await db.schema.dropTable("nfts").execute();
}
