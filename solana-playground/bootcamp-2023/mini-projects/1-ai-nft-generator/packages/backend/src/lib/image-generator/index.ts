import { env } from "@/env";
import { LocalhostImageGenerationService } from "./localhost";
import { HyperbolicImageGenerationService } from "./hyperbolic";
import type { ImageGeneratorService } from "./types";

export const imageGenerator: ImageGeneratorService =
  env.NODE_ENV === "development"
    ? new LocalhostImageGenerationService()
    : new HyperbolicImageGenerationService();

export type { ImageGeneratorService };
