import type { ImageGeneratorService } from "./types";
import { ok } from "true-myth/result";

class LocalhostImageGenerationService implements ImageGeneratorService {
  async generateImage(prompt: string) {
    return ok(new File([prompt], "image.png"));
  }
}

export { LocalhostImageGenerationService };
