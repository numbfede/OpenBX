import { api } from "../services/api";
import type { TweakState } from "../services/types";
import { GlassButton } from "./GlassButton";
import { GlassCard } from "./GlassCard";
import { Modal } from "./Modal";
import { StatusBadge } from "./StatusBadge";

export function TweakDetailSheet({
  tweak,
  developerMode,
  onClose,
}: {
  tweak: TweakState;
  developerMode: boolean;
  onClose: () => void;
}) {
  return (
    <Modal onClose={onClose}>
      <GlassCard padding="lg" className="max-h-[80vh] overflow-y-auto">
        <div className="flex items-start justify-between gap-4">
          <div>
            <p className="text-[11px] uppercase tracking-[0.2em] text-[color:var(--faint)]">Dettagli</p>
            <h2 className="mt-3 text-2xl font-medium">{tweak.title}</h2>
          </div>
          <StatusBadge
            tone={tweak.optimized ? "ready" : tweak.applicable ? "warn" : "muted"}
            label={tweak.optimized ? "Ottimizzata" : tweak.applicable ? "Disponibile" : "Non applicabile"}
          />
        </div>
        <Section title="WHAT?" body={tweak.what} />
        <Section title="WHY?" body={tweak.why} />
        <div className="mt-5 grid grid-cols-2 gap-3 text-sm">
          <Meta label="RISK" value={tweak.risk} />
          <Meta label="REVERSIBLE?" value={tweak.reversible ? "Yes" : "No"} />
        </div>
        <div className="mt-5">
          <p className="text-[11px] uppercase tracking-[0.18em] text-[color:var(--faint)]">SOURCE</p>
          <div className="mt-2 flex flex-col gap-2">
            {tweak.sources.map((source) => (
              <button
                key={source.url}
                className="text-left text-sm text-cream underline-offset-4 hover:underline"
                onClick={() => void api.openUrl(source.url)}
              >
                {source.title}
              </button>
            ))}
          </div>
        </div>
        {developerMode && tweak.technical ? (
          <div className="mt-6 space-y-2 rounded-btn bg-black/20 p-4 text-xs leading-relaxed text-[color:var(--muted)]">
            <p>Registry: {tweak.technical.registryPath ?? "—"}</p>
            <p>Old value: {tweak.technical.oldValue ?? "—"}</p>
            <p>New value: {tweak.technical.newValue ?? "—"}</p>
            <p>PowerShell: {tweak.technical.powershell ?? "—"}</p>
            <p>Windows API: {tweak.technical.windowsApi ?? "—"}</p>
            <p>Rollback: {tweak.technical.rollbackMethod}</p>
          </div>
        ) : null}
        <GlassButton className="mt-6 w-full" variant="secondary" onClick={onClose}>
          Chiudi
        </GlassButton>
      </GlassCard>
    </Modal>
  );
}

function Section({ title, body }: { title: string; body: string }) {
  return (
    <div className="mt-5">
      <p className="text-[11px] uppercase tracking-[0.18em] text-[color:var(--faint)]">{title}</p>
      <p className="mt-2 text-sm leading-relaxed text-[color:var(--muted)]">{body}</p>
    </div>
  );
}

function Meta({ label, value }: { label: string; value: string }) {
  return (
    <div className="glass rounded-btn p-3">
      <p className="text-[11px] uppercase tracking-[0.16em] text-[color:var(--faint)]">{label}</p>
      <p className="mt-1 capitalize">{value}</p>
    </div>
  );
}
