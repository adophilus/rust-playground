import { Result } from "true-myth";

export type ImageGeneratorServiceError = "GENERATION_FAILED";

export type ImageGeneratorService = {
  generateImage: (
    prompt: string,
  ) => Promise<Result<File, ImageGeneratorServiceError>>;
};
