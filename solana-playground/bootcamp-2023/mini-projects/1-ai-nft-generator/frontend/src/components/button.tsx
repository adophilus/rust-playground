import { cn } from "@/lib/utils";
import { type ComponentProps, forwardRef } from "react";

export const Button = forwardRef<HTMLButtonElement, ComponentProps<"button">>(
  ({ className, ...props }, ref) => (
    <button
      {...props}
      className={cn("bg-[#0D1B2A] text-white rounded-md px-2.5 py-1.5 focus:outline-2 focus:outline-offset-2 active:outline-2 active:outline-offset-2 hover:cursor-pointer", className)}
      ref={ref}
    />
  ),
);
