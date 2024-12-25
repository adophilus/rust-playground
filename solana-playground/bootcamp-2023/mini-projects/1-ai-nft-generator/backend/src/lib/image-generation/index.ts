import { env } from "@/env";
import { LocalhostImageGenerationService } from "./localhost";
import { HyperbolicImageGenerationService } from "./hyperbolic";
import type { ImageGenerationService } from "./types";

export const imageGeneration: ImageGenerationService =
  env.NODE_ENV === "development"
    ? new LocalhostImageGenerationService()
    : new HyperbolicImageGenerationService();
