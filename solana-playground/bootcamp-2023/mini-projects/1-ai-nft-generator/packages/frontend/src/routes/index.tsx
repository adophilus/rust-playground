import { Input } from "@nft-ai-generator/frontend/components/input";
import { StatefulButton } from "@nft-ai-generator/frontend/components/button";
import { Section } from "@nft-ai-generator/frontend/components/section";
import { SparklesIcon } from "lucide-react";
import { createFileRoute, useNavigate } from "@tanstack/react-router";
import { useForm } from "@tanstack/react-form";
import { z } from "zod";
import { useMutation } from "@tanstack/react-query";
import { backendClient } from "@nft-ai-generator/frontend/lib/http";
import type { InferRequestType } from "hono";
import { toast } from "sonner";
import { useRef } from "react";

export const Route = createFileRoute("/")({
  component: IndexPage,
});

const promptSchema = z.object({
  prompt: z
    .string({ message: "Prompt cannot be empty" })
    .nonempty({ message: "Prompt cannot be empty" }),
});

function IndexPage() {
  const toastId = useRef<string | number>();
  const navigate = useNavigate();

  const { mutateAsync, status } = useMutation({
    mutationFn: async (
      payload: InferRequestType<typeof backendClient.api.nfts.$post>["json"],
    ) => {
      toastId.current = toast.loading("Generating...");

      const res = await backendClient.api.nfts.$post({
        json: payload,
      });

      return res.json();
    },
    onSuccess: (data) => {
      toast.dismiss(toastId.current);
      navigate({
        to: "/nfts/$id",
        params: {
          id: data.id,
        },
      });
    },
    onError: (error) => {
      toast.dismiss(toastId.current);
      toast.error(error.message);
    },
  });

  const form = useForm({
    onSubmit: ({ value }) => mutateAsync(value),
    validators: {
      onChange: promptSchema,
    },
  });

  const isSubmitting = status === "pending";

  return (
    <div className="h-screen flex flex-col text-[#0D1B2A]">
      <div className="grow flex items-center justify-center">
        <Section>
          <form
            className="flex gap-2 items-start"
            onSubmit={(e) => {
              e.preventDefault();
              e.stopPropagation();
              form.handleSubmit();
            }}
          >
            <form.Field name="prompt">
              {(field) => (
                <div className="w-full">
                  <Input
                    type="text"
                    placeholder="Enter a prompt to generate an image"
                    className="w-full"
                    name={field.name}
                    value={field.state.value}
                    onBlur={field.handleBlur}
                    onChange={(e) => field.handleChange(e.target.value)}
                  />
                  <div>
                    <span className="text-red-500 text-xs">
                      {field.state.meta.errors.join(",")}
                    </span>
                  </div>
                </div>
              )}
            </form.Field>
            <StatefulButton type="submit" isLoading={isSubmitting}>
              <span className="inline-flex items-center gap-1">
                <SparklesIcon className="size-5" />
                Generate
              </span>
            </StatefulButton>
          </form>
        </Section>
      </div>
    </div>
  );
}
