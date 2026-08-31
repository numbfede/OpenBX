import { useEffect } from "react";
import { GameCard } from "../components/GameCard";
import { GlassButton } from "../components/GlassButton";
import { useAppStore } from "../store/useAppStore";

export function GamesScreen() {
  const games = useAppStore((state) => state.games);
  const refreshGames = useAppStore((state) => state.refreshGames);
  const optimizeGame = useAppStore((state) => state.optimizeGame);

  useEffect(() => {
    void refreshGames();
  }, [refreshGames]);

  return (
    <div className="mx-auto max-w-3xl py-8">
      <p className="text-[11px] uppercase tracking-[0.22em] text-[color:var(--faint)]">( 04 ) — Giochi</p>
      <div className="mt-3 flex items-end justify-between gap-4">
        <div>
          <h2 className="text-4xl font-medium tracking-tight">I tuoi giochi</h2>
          <p className="mt-3 max-w-xl text-[15px] text-[color:var(--muted)]">
            Riconosciamo i giochi installati quando possibile. Applichiamo solo preferenze GPU documentate.
          </p>
        </div>
        <GlassButton variant="secondary" onClick={() => void refreshGames()}>
          Cerca giochi
        </GlassButton>
      </div>
      <div className="mt-8 space-y-3">
        {games.length === 0 ? (
          <p className="text-sm text-[color:var(--muted)]">
            Nessun gioco trovato in locale. Steam, Epic e il menu Start vengono controllati offline.
          </p>
        ) : (
          games.map((game) => (
            <GameCard key={game.id} game={game} onOptimize={() => void optimizeGame(game.id)} />
          ))
        )}
      </div>
    </div>
  );
}
