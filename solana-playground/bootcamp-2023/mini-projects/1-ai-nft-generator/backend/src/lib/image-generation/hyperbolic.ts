import { env } from "@/env";
import OpenAI from "openai";
import type { ImageGenerationService } from "./types";

class HyperbolicImageGenerationService implements ImageGenerationService {
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

    const res = await fetch("https://api.hyperbolic.xyz/v1/image/generation", {
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

    const json: GenerateImageApiRepsonse = await res.json();

    const image = new File(
      [Buffer.from(json.images[0].image, "base64")],
      "image.png",
    );

    return image;
  }
}

export { HyperbolicImageGenerationService };
