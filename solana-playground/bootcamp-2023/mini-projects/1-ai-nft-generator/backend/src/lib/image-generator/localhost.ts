import type { ImageGeneratorService } from "./types";

class LocalhostImageGenerationService implements ImageGeneratorService {
  async generateImage(prompt: string) {
    return new File([prompt], "image.png");
  }
}

export { LocalhostImageGenerationService };
