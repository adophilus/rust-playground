import { ok } from "true-myth/result";
import type { StorageService } from "./types";

class LocalhostStorageService implements StorageService {
  async uploadFile(_: File) {
    return ok(
      "https://utfs.io/f/R9PBlhgRHZlq0l7PDNEfrVOL3mqlhYHEcaSBbn8MFGeAXstv",
    );
  }
}

export { LocalhostStorageService };
