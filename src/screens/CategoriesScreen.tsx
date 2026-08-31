import { OptimizationCard } from "../components/OptimizationCard";
import { StatusBadge } from "../components/StatusBadge";
import { useAppStore } from "../store/useAppStore";
import { CATEGORY_META } from "../services/types";

export function CategoriesScreen() {
  const scan = useAppStore((state) => state.scan);
  const settings = useAppStore((state) => state.settings);
  const openTweak = useAppStore((state) => state.openTweak);

  return (
    <div className="mx-auto max-w-3xl space-y-6 py-8">
      <div>
        <p className="text-[11px] uppercase tracking-[0.22em] text-[color:var(--faint)]">( 02 ) — Categorie</p>
        <h2 className="mt-3 text-4xl font-medium tracking-tight">Cosa vuoi migliorare</h2>
        <p className="mt-3 max-w-xl text-[15px] text-[color:var(--muted)]">
          Non una lista di tweak. Cinque obiettivi chiari, con lo stato reale del tuo PC.
        </p>
      </div>
      <div className="space-y-3">
        {scan?.categories.map((category) => (
          <OptimizationCard
            key={category.id}
            category={category}
            onClick={() => {
              const first = scan.tweaks.find((tweak) => tweak.category === category.id && tweak.applicable);
              if (first) openTweak(first);
            }}
          />
        ))}
      </div>
      {scan ? (
        <div className="space-y-3 pt-4">
          {scan.tweaks
            .filter((tweak) => tweak.applicable)
            .map((tweak) => (
              <button
                key={tweak.id}
                className="glass flex w-full items-center justify-between rounded-card px-5 py-4 text-left"
                onClick={() => openTweak(tweak)}
              >
                <div>
                  <p className="text-[11px] uppercase tracking-[0.16em] text-[color:var(--faint)]">
                    {CATEGORY_META[tweak.category].title}
                  </p>
                  <p className="mt-1 text-[15px]">{tweak.title}</p>
                  <p className="mt-1 text-sm text-[color:var(--muted)]">{tweak.description}</p>
                </div>
                <StatusBadge
                  tone={tweak.optimized ? "ready" : "warn"}
                  label={tweak.optimized ? "OK" : "Dettagli"}
                />
              </button>
            ))}
        </div>
      ) : (
        <p className="text-sm text-[color:var(--muted)]">Controlla il PC dalla Home per vedere le categorie.</p>
      )}
      {settings.showTechnicalDetails ? (
        <p className="text-xs text-[color:var(--faint)]">I dettagli tecnici restano nascosti finché non apri Dettagli.</p>
      ) : null}
    </div>
  );
}
