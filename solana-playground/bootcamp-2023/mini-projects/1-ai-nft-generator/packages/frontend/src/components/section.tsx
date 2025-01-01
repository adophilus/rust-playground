import { cn } from "@/lib/utils";
import { ComponentProps, forwardRef } from "react";

export const Section = forwardRef<HTMLDivElement, ComponentProps<"div">>(({ className, ...props }, ref) => <section
  {...props}
  className={cn('w-full max-w-4xl px-4', className)}
  ref={ref}
/>)
