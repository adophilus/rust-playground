import { env } from "@/env";
import type { DatabaseTables } from "./types";
import { Kysely } from "kysely";
import { LibsqlDialect } from "@libsql/kysely-libsql";

export const db = new Kysely<DatabaseTables>({
  dialect: new LibsqlDialect({
    url: env.DATABASE_URL,
    authToken: env.DATABASE_AUTH_TOKEN,
  }),
});

export type { DatabaseTables };
