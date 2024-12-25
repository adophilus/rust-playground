export type ImageGeneratorService = {
  generateImage: (prompt: string) => Promise<File>;
};
