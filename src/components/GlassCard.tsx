import type { HTMLAttributes } from "react";

interface GlassCardProps extends HTMLAttributes<HTMLDivElement> {
  padding?: "sm" | "md" | "lg";
}

const paddingClass = {
  sm: "p-4",
  md: "p-6",
  lg: "p-8",
};

export function GlassCard({ padding = "md", className = "", children, ...props }: GlassCardProps) {
  return (
    <div className={`glass rounded-card ${paddingClass[padding]} ${className}`} {...props}>
      {children}
    </div>
  );
}
