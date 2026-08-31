import { GlassCard } from "../components/GlassCard";
import { useAppStore } from "../store/useAppStore";
import type { GameModePreset } from "../services/types";

const presets: { id: GameModePreset; title: string; body: string; kicker: string }[] = [
  {
    id: "competitive",
    title: "Competitive",
    body: "Massima priorità a performance e meno overlay. Non forziamo lo scheduling GPU accelerato: su alcuni giochi aumenta lo stutter.",
    kicker: "( 01 )",
  },
  {
    id: "balanced",
    title: "Balanced",
    body: "Performance + utilizzo normale del PC.",
    kicker: "( 02 )",
  },
  {
    id: "streaming",
    title: "Streaming",
    body: "Gaming + OBS/streaming, senza chiudere il resto.",
    kicker: "( 03 )",
  },
  {
    id: "default",
    title: "Default",
    body: "Ripristina le impostazioni Windows normali.",
    kicker: "( 04 )",
  },
];

export function GameModeScreen() {
  const applyGameMode = useAppStore((state) => state.applyGameMode);
  const busy = useAppStore((state) => state.busy);

  return (
    <div className="mx-auto max-w-3xl py-8">
      <p className="text-[11px] uppercase tracking-[0.22em] text-[color:var(--faint)]">( 03 ) — Game Mode</p>
      <h2 className="mt-3 text-4xl font-medium tracking-tight">GAME MODE</h2>
      <p className="mt-3 max-w-xl text-[15px] text-[color:var(--muted)]">
        Quattro scelte. Competitive spegne Game Bar e non attiva HAGS. Non è un pacchetto FPS.
      </p>
      <div className="mt-8 grid gap-3 sm:grid-cols-2">
        {presets.map((preset) => (
          <button
            key={preset.id}
            disabled={busy}
            className="text-left disabled:opacity-50"
            onClick={() => void applyGameMode(preset.id)}
          >
            <GlassCard className="h-full transition duration-200 hover:bg-white/5">
              <p className="text-[11px] uppercase tracking-[0.18em] text-[color:var(--faint)]">{preset.kicker}</p>
              <h3 className="mt-4 text-2xl font-medium">{preset.title}</h3>
              <p className="mt-3 text-sm leading-relaxed text-[color:var(--muted)]">{preset.body}</p>
            </GlassCard>
          </button>
        ))}
      </div>
    </div>
  );
}
