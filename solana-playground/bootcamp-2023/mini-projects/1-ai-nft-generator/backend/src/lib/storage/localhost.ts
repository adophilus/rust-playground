import type { StorageService } from "./types";

class LocalhostStorageService implements StorageService {
  async uploadFile(file: File): Promise<string> {
    return "https://utfs.io/f/R9PBlhgRHZlq0l7PDNEfrVOL3mqlhYHEcaSBbn8MFGeAXstv";
  }
}

export { LocalhostStorageService };
