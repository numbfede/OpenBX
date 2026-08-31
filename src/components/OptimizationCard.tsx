import type { CategorySummary } from "../services/types";
import { CATEGORY_META } from "../services/types";
import { GlassCard } from "./GlassCard";

export function OptimizationCard({
  category,
  onClick,
}: {
  category: CategorySummary;
  onClick?: () => void;
}) {
  const meta = CATEGORY_META[category.id];
  const ratio = category.applicable === 0 ? 1 : category.optimized / category.applicable;

  return (
    <button className="w-full text-left" onClick={onClick}>
      <GlassCard className="transition duration-200 ease-openbx hover:bg-white/5">
        <div className="flex items-start justify-between gap-4">
          <div>
            <p className="text-[11px] uppercase tracking-[0.2em] text-[color:var(--faint)]">{meta.kicker}</p>
            <h3 className="mt-3 text-xl font-medium">{meta.title}</h3>
            <p className="mt-2 text-sm text-[color:var(--muted)]">{meta.description}</p>
          </div>
          <span className="text-sm text-[color:var(--muted)]">
            {category.optimized}/{category.applicable} optimized
          </span>
        </div>
        <div className="mt-6 h-1 overflow-hidden rounded-full bg-white/10">
          <div
            className="h-full rounded-full bg-cream/80"
            style={{ width: `${Math.round(ratio * 100)}%` }}
          />
        </div>
      </GlassCard>
    </button>
  );
}
