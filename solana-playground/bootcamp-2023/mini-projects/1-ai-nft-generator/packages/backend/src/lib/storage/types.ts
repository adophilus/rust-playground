import { Result } from "true-myth";

export type StorageServiceError = "UPLOAD_FAILED";

export interface StorageService {
  uploadFile: (file: File) => Promise<Result<string, StorageServiceError>>;
}
