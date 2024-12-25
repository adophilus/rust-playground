import type { ImageGenerationService } from "./types";

class LocalhostImageGenerationService implements ImageGenerationService {
  async generateImage(prompt: string) {
    return new File([prompt], "image.png");
  }
}

export { LocalhostImageGenerationService };
