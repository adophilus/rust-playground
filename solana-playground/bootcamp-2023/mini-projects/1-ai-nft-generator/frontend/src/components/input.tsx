import { cn } from "@/lib/utils";
import { type ComponentProps, forwardRef } from "react";

export const Input = forwardRef<HTMLInputElement, ComponentProps<"input">>(
  ({ className, ...props }, ref) => (
    <input
      {...props}
      className={cn(
        "focus:outline-2 focus:outline-offset-2 rounded-md border border-gray-300 py-1.5 px-2.5",
        className,
      )}
      ref={ref}
    />
  ),
);
