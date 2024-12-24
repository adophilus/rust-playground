import { cn } from "@/lib/utils";
import { Loader2Icon } from "lucide-react";
import { type ComponentProps, forwardRef, type ReactNode } from "react";

export const Button = forwardRef<HTMLButtonElement, ComponentProps<"button">>(
  ({ className, ...props }, ref) => (
    <button
      {...props}
      className={cn(
        "bg-[#0D1B2A] text-white rounded-md px-2.5 py-1.5 focus:outline-2 focus:outline-offset-2 active:outline-2 active:outline-offset-2 hover:cursor-pointer disabled:bg-gray-500",
        className,
      )}
      ref={ref}
    />
  ),
);

export type StatefulButtonProps = ComponentProps<typeof Button> & {
  isLoading?: boolean;
  loader?: ReactNode;
};

export const StatefulButton = forwardRef<
  HTMLButtonElement,
  StatefulButtonProps
>(({ className, isLoading, children, ...props }, ref) => (
  <Button
    {...props}
    className={cn(
      "grid [&>*]:[grid-area:1/1] justify-items-center items-center",
      className,
    )}
    ref={ref}
    disabled={isLoading}
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
