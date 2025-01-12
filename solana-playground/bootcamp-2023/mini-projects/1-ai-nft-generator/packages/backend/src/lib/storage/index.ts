import { env } from "@/env";
import type { StorageService } from "./types";
import { LocalhostStorageService } from "./localhost";
import { ArweaveStorageService } from "./arweave";

export const storage = new ArweaveStorageService()
// export const storage: StorageService =
//   env.NODE_ENV === "development"
//     ? new LocalhostStorageService()
//     : new ArweaveStorageService();

export type { StorageService };
