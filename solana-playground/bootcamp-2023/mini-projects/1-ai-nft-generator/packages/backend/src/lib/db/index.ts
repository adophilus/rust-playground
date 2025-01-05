import { env } from "@/env";
import type { DatabaseTables } from "./types";
import { Kysely, PostgresDialect } from "kysely";
import { TablePrefixPlugin, IndexPrefixPlugin } from "kysely-plugin-prefix";
import pg from "pg";

const { Pool } = pg;

const dialect = new PostgresDialect({
  pool: new Pool({
    connectionString: env.DATABASE_URL,
  }),
});

export const db = new Kysely<DatabaseTables>({
  dialect,
  plugins: [
    new TablePrefixPlugin({ prefix: "nft_ai_generator_" }),
    new IndexPrefixPlugin({ prefix: "nft_ai_generator_" }),
  ],
});

export type { DatabaseTables };
