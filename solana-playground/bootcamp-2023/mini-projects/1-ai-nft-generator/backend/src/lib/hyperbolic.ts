import { env } from "@/env";
import OpenAI from "openai";

export namespace HyperbolicService {
  const client = new OpenAI({
    apiKey: env.HYPERBOLIC_API_KEY,
    baseURL: "https://api.hyperbolic.xyz/v1/",
  });

  type GenerateImageApiRepsonse = {
    images: { image: string }[];
  };

  export const generateImage = async (prompt: string): Promise<File> => {
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
  };
}
