import { Input } from "@/components/input";
import { StatefulButton } from "@/components/button";
import { Section } from "@/components/section";
import { SparklesIcon } from "lucide-react";
import { createFileRoute } from "@tanstack/react-router";
import { useForm } from "@tanstack/react-form";
import { z } from "zod";

export const Route = createFileRoute("/")({
  component: IndexPage,
});

const promptSchema = z.object({
  prompt: z
    .string({ message: "Prompt cannot be empty" })
    .nonempty({ message: "Prompt cannot be empty" }),
});

function IndexPage() {
  const form = useForm({
    onSubmit: async ({ value }) => {
      console.log(value);
    },
    validators: {
      onChange: promptSchema,
    },
  });

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
            <form.Field
              name="prompt"
              children={(field) => (
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
            />
            <StatefulButton type="submit" isLoading={form.state.isSubmitting}>
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
