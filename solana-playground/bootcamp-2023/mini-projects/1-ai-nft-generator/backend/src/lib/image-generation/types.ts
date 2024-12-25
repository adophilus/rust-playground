export type ImageGenerationService = {
  generateImage: (prompt: string) => Promise<File>;
};
