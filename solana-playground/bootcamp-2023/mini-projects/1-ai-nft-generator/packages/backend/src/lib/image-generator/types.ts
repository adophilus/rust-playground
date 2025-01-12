import { Result } from "true-myth";

export type ImageGeneratorServiceError = "UPLOAD_FAILED";

export type ImageGeneratorService = {
  generateImage: (
    prompt: string,
  ) => Promise<Result<File, ImageGeneratorServiceError>>;
};
