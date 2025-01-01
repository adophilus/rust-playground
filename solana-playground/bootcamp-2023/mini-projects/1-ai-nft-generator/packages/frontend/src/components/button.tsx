import { cn } from "@/lib/utils";
import { Loader2Icon } from "lucide-react";
import { type ComponentProps, forwardRef, type ReactNode } from "react";

type ButtonProps = ComponentProps<"button"> & {
  variant?: "primary" | "secondary";
};

export const Button = forwardRef<HTMLButtonElement, ButtonProps>(
  ({ className, variant = "primary", ...props }, ref) => (
    <button
      {...props}
      className={cn(
        "rounded-md px-2.5 py-1.5 not-disabled:focus:outline-2 not-disabled:focus:outline-offset-2 not-disabled:active:outline-2 active:outline-offset-2 hover:cursor-pointer disabled:bg-gray-400 transition-all duration-300 not-disabled:hover:scale-102",
        variant === "primary" &&
          "bg-[#0D1B2A] text-white hover:bg-[color-mix(in_srgb,#0D1B2A_90%,#FFFFFF_10%)]",
        variant === "secondary" &&
          "bg-white text-[#0D1B2A] hover:bg-[color-mix(in_srgb,#FFFFFF_90%,#0D1B2A_10%)]",
        className,
      )}
      ref={ref}
    />
  ),
);

export type StatefulButtonProps = ButtonProps & {
  isLoading?: boolean;
  loader?: ReactNode;
};

export const StatefulButton = forwardRef<
  HTMLButtonElement,
  StatefulButtonProps
>(({ className, isLoading, children, disabled, ...props }, ref) => (
  <Button
    {...props}
    className={cn(
      "grid [&>*]:[grid-area:1/1] justify-items-center items-center",
      className,
    )}
    ref={ref}
    disabled={disabled || isLoading}
  >
    <Loader2Icon
      className={cn("size-6 animate-spin", isLoading ? "visible" : "invisible")}
    />
    <span
      className={cn(
        "flex grow items-center",
        isLoading ? "invisible" : "visible",
      )}
    >
      {children}
    </span>
  </Button>
));
