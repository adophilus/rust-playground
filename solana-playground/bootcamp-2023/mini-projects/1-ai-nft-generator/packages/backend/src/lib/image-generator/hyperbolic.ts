import { env } from "@/env";
import OpenAI from "openai";
import {
  ImageGeneratorServiceError,
  type ImageGeneratorService,
} from "./types";
import { ok, err } from "true-myth/result";

class HyperbolicImageGeneratorService implements ImageGeneratorService {
  private declare client: OpenAI;

  constructor() {
    this.client = new OpenAI({
      apiKey: env.HYPERBOLIC_API_KEY,
      baseURL: "https://api.hyperbolic.xyz/v1/",
    });
  }

  async generateImage(prompt: string) {
    type GenerateImageApiRepsonse = {
      images: { image: string }[];
    };

    let res: Response;
    try {
      res = await fetch("https://api.hyperbolic.xyz/v1/image/generation", {
        method: "POST",
        headers: {
          Authorization: `Bearer ${env.HYPERBOLIC_API_KEY}`,
          "Content-Type": "application/json",
        },
        body: JSON.stringify({
          model_name: "SDXL1.0-base",
          prompt,
          height: 1024,
          width: 1024,
          backend: "auto",
        }),
      });
    } catch (error) {
      console.warn(error);
      return err(ImageGeneratorServiceError.UploadFailed);
    }

    const json: GenerateImageApiRepsonse = await res.json();

    const image = new File(
      [Buffer.from(json.images[0].image, "base64")],
      "image.png",
    );

    return ok(image);
  }
}

export { HyperbolicImageGeneratorService as HyperbolicImageGenerationService };
