import type { ButtonHTMLAttributes } from "react";

type Variant = "primary" | "secondary" | "ghost" | "danger";

interface GlassButtonProps extends ButtonHTMLAttributes<HTMLButtonElement> {
  variant?: Variant;
  size?: "md" | "lg";
}

const variants: Record<Variant, string> = {
  primary:
    "bg-cream text-canvas hover:opacity-90 shadow-[0_0_0_1px_rgb(245_240_232_/_0.15),0_12px_30px_rgb(0_0_0_/_0.25)]",
  secondary: "glass text-ink hover:bg-white/10",
  ghost: "bg-transparent text-[color:var(--muted)] hover:text-ink hover:bg-white/5",
  danger: "bg-danger/15 text-danger hover:bg-danger/25",
};

export function GlassButton({
  variant = "primary",
  size = "md",
  className = "",
  children,
  ...props
}: GlassButtonProps) {
  return (
    <button
      className={`no-drag inline-flex items-center justify-center rounded-btn font-medium tracking-wide transition duration-200 ease-openbx disabled:cursor-not-allowed disabled:opacity-40 ${
        size === "lg" ? "h-14 px-7 text-[15px]" : "h-11 px-5 text-sm"
      } ${variants[variant]} ${className}`}
      {...props}
    >
      {children}
    </button>
  );
}
