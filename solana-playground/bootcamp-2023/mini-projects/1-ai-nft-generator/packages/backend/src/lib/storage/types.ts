export interface StorageService {
  uploadFile: (file: File) => Promise<string>;
}
