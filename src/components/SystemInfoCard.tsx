import { GlassCard } from "./GlassCard";

export function SystemInfoCard({
  label,
  value,
}: {
  label: string;
  value: string;
}) {
  return (
    <GlassCard padding="sm" className="min-w-0">
      <p className="text-[11px] uppercase tracking-[0.18em] text-[color:var(--faint)]">{label}</p>
      <p className="mt-2 truncate text-sm text-ink">{value}</p>
    </GlassCard>
  );
}
