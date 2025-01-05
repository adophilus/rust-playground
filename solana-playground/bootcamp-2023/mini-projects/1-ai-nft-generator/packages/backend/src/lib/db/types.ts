import type { ColumnType } from "kysely";

type NftsTable = {
  id: string;
  image_url: string;
  prompt: string;
  created_at: ColumnType<number, never, never>;
  updated_at: ColumnType<number | null, never, number>;
};

export type DatabaseTables = {
  nfts: NftsTable;
};
