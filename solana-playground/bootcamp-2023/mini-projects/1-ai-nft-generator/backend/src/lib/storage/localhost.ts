import type { StorageService } from "./types";

class LocalhostStorageService implements StorageService {
  async uploadFile(file: File): Promise<string> {
    return "https://utfs.io/f/R9PBlhgRHZlqZiAASUVFkIY3cHl21jUGWFOEtbvziyfnMqJa";
  }
}

export { LocalhostStorageService };
