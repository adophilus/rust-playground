import { env } from "@/env";
import type { StorageService } from "./types";
import { LocalhostStorageService } from "./localhost";
import { UploadThingStorageService } from "./uploadthing";

export const storage: StorageService =
  env.NODE_ENV === "development"
    ? new LocalhostStorageService()
    : new UploadThingStorageService();

export type { StorageService };
