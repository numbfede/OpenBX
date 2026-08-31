import type { GameEntry } from "../services/types";
import { GlassButton } from "./GlassButton";
import { GlassCard } from "./GlassCard";
import { StatusBadge } from "./StatusBadge";

export function GameCard({
  game,
  onOptimize,
}: {
  game: GameEntry;
  onOptimize: () => void;
}) {
  return (
    <GlassCard className="flex items-center justify-between gap-4">
      <div>
        <h3 className="text-lg font-medium">{game.name}</h3>
        <p className="mt-1 text-xs uppercase tracking-[0.16em] text-[color:var(--faint)]">{game.source}</p>
        <div className="mt-3">
          <StatusBadge
            tone={game.optimized ? "ready" : "warn"}
            label={game.optimized ? "Optimized" : "Da ottimizzare"}
          />
        </div>
      </div>
      <GlassButton disabled={!game.applicable || game.optimized} onClick={onOptimize}>
        OPTIMIZE
      </GlassButton>
    </GlassCard>
  );
}
