import type { DatabaseTables } from "./lib/db";
import type { Selectable } from "kysely";

export type Nft = Selectable<DatabaseTables["nfts"]>;
