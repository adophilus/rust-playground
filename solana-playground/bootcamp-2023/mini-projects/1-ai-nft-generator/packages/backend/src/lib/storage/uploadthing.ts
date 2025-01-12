import { env } from "@/env";
import { UTApi } from "uploadthing/server";
import type { StorageService } from "./types";
import { ok } from "true-myth/result";

class UploadThingStorageService implements StorageService {
  private declare utapi: UTApi;
  constructor() {
    this.utapi = new UTApi({
      token: env.UPLOADTHING_TOKEN,
    });
  }

  async uploadFile(file: File) {
    const res = await this.utapi.uploadFiles(file);
    if (res.error) throw new Error(res.error.message);
    return ok(res.data.url);
  }
}

export { UploadThingStorageService };
