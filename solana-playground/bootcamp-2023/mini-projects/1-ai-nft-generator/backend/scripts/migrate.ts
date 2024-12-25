import * as path from "node:path";
import { promises as fs } from "node:fs";
import { run } from "kysely-migration-cli";
import { db } from "@/lib/db";
import { FileMigrationProvider, Migrator } from "kysely";

const MIGRATION_FOLDER = path.join(process.cwd(), "./migrations");

const migrator = new Migrator({
  db,
  provider: new FileMigrationProvider({
    fs,
    path,
    migrationFolder: MIGRATION_FOLDER,
  }),
});

run(db as any, migrator as any, MIGRATION_FOLDER);
