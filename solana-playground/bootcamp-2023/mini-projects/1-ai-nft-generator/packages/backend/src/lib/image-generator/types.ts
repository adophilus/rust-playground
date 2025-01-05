import { Result } from "true-myth";

export enum ImageGeneratorServiceError {
  UploadFailed,
}

export type ImageGeneratorService = {
  generateImage: (
    prompt: string,
  ) => Promise<Result<File, ImageGeneratorServiceError>>;
};
