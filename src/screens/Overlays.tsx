import { ConfirmationDialog } from "../components/ConfirmationDialog";
import { ErrorPanel } from "../components/ErrorPanel";
import { GlassButton } from "../components/GlassButton";
import { GlassCard } from "../components/GlassCard";
import { Modal } from "../components/Modal";
import { ProgressBar } from "../components/ProgressBar";
import { ScoreRing } from "../components/ScoreRing";
import { TweakDetailSheet } from "../components/TweakDetailSheet";
import { useAppStore } from "../store/useAppStore";
import { plural } from "../utils/format";

const STEP_COPY: Record<string, string> = {
  scanning: "Scanning...",
  backup: "Creating backup...",
  windows: "Optimizing Windows...",
  gaming: "Optimizing gaming...",
  startup: "Optimizing startup...",
  verifying: "Verifying changes...",
  done: "DONE",
  failed: "Something went wrong",
};

export function Overlays() {
  const overlay = useAppStore((state) => state.overlay);
  const settings = useAppStore((state) => state.settings);
  const selectedTweak = useAppStore((state) => state.selectedTweak);
  const progress = useAppStore((state) => state.optimizeProgress);
  const lastResult = useAppStore((state) => state.lastResult);
  const lastError = useAppStore((state) => state.lastError);
  const system = useAppStore((state) => state.system);
  const setOverlay = useAppStore((state) => state.setOverlay);
  const runOptimize = useAppStore((state) => state.runOptimize);
  const runRestore = useAppStore((state) => state.runRestore);
  const refreshScan = useAppStore((state) => state.refreshScan);
  const updateSettings = useAppStore((state) => state.updateSettings);
  const relaunchElevated = useAppStore((state) => state.relaunchElevated);

  if (overlay === "elevation") {
    return (
      <Modal>
        <GlassCard padding="lg">
          <p className="text-[11px] uppercase tracking-[0.2em] text-[color:var(--faint)]">Permesso Windows</p>
          <h2 className="mt-4 text-3xl font-medium">Windows deve autorizzare le modifiche</h2>
          <p className="mt-3 text-[15px] leading-relaxed text-[color:var(--muted)]">
            {system?.isDev
              ? "Stai usando la modalità sviluppo. Non premere Autorizza: Windows chiuderebbe la connessione a localhost. Continua, oppure riapri il terminale come amministratore."
              : "Per ottimizzare il PC serve il permesso di Windows. Non chiediamo password a OpenBX: è Windows che conferma."}
          </p>
          <div className="mt-8 flex flex-col gap-3">
            {system?.isDev ? null : (
              <GlassButton size="lg" onClick={() => void relaunchElevated()}>
                Autorizza
              </GlassButton>
            )}
            <GlassButton
              variant={system?.isDev ? "primary" : "ghost"}
              size={system?.isDev ? "lg" : "md"}
              onClick={() => {
                void updateSettings({ firstRunCompleted: system?.isElevated ? true : settings.firstRunCompleted });
                setOverlay("firstrun");
              }}
            >
              Continua
            </GlassButton>
          </div>
        </GlassCard>
      </Modal>
    );
  }

  if (overlay === "firstrun") {
    return (
      <Modal>
        <GlassCard padding="lg" className="text-center">
          <p className="text-[11px] uppercase tracking-[0.2em] text-[color:var(--faint)]">Primo avvio</p>
          <h2 className="mt-4 text-4xl font-medium tracking-tight">Controlliamo il tuo PC.</h2>
          <p className="mt-3 text-[15px] text-[color:var(--muted)]">
            Nessuna modifica. Solo una lettura dello stato attuale.
          </p>
          <GlassButton
            className="mt-8 w-full"
            size="lg"
            onClick={() => {
              void (async () => {
                await updateSettings({ firstRunCompleted: true });
                await refreshScan();
                setOverlay(null);
              })();
            }}
          >
            SCAN
          </GlassButton>
        </GlassCard>
      </Modal>
    );
  }

  if (overlay === "optimize") {
    return (
      <Modal>
        <GlassCard padding="lg" className="text-center">
          <p className="text-[11px] uppercase tracking-[0.2em] text-[color:var(--faint)]">Optimize</p>
          <h2 className="mt-6 text-3xl font-medium">{STEP_COPY[progress?.step ?? "scanning"]}</h2>
          <p className="mt-3 text-sm text-[color:var(--muted)]">{progress?.message}</p>
          <div className="mt-8">
            <ProgressBar current={progress?.current ?? 0} total={progress?.total ?? 1} />
          </div>
        </GlassCard>
      </Modal>
    );
  }

  if (overlay === "result" && lastResult) {
    const ready = lastResult.scoreAfter >= lastResult.scoreBefore;
    return (
      <Modal>
        <GlassCard padding="lg" className="text-center">
          <p className="text-[11px] uppercase tracking-[0.2em] text-[color:var(--faint)]">Risultato</p>
          <h2 className="mt-4 text-4xl font-medium tracking-tight">
            {ready ? "YOUR PC IS READY" : "Operazione completata"}
          </h2>
          <p className="mt-3 text-[15px] text-[color:var(--muted)]">
            {lastResult.applied > 0
              ? `Abbiamo ottimizzato ${plural(lastResult.applied, "impostazione", "impostazioni")}.`
              : "Nessuna modifica necessaria: il PC era già a posto."}
          </p>
          <div className="mt-8 flex items-center justify-center gap-10">
            <div>
              <p className="text-[11px] uppercase tracking-[0.16em] text-[color:var(--faint)]">Prima</p>
              <p className="mt-2 text-4xl">{lastResult.scoreBefore}</p>
            </div>
            <div>
              <p className="text-[11px] uppercase tracking-[0.16em] text-[color:var(--faint)]">Dopo</p>
              <p className="mt-2 text-4xl text-ready">{lastResult.scoreAfter}</p>
            </div>
          </div>
          <div className="mt-8 text-left">
            <p className="text-[11px] uppercase tracking-[0.16em] text-[color:var(--faint)]">WHAT WE CHANGED</p>
            <p className="mt-2 text-sm text-[color:var(--muted)]">
              {plural(lastResult.changes.length, "optimization", "optimizations")}
            </p>
            <GlassButton className="mt-3" variant="ghost" onClick={() => setOverlay("beforeafter")}>
              View details
            </GlassButton>
          </div>
          <GlassButton className="mt-6 w-full" size="lg" onClick={() => setOverlay(null)}>
            DONE
          </GlassButton>
        </GlassCard>
      </Modal>
    );
  }

  if (overlay === "beforeafter" && lastResult) {
    return (
      <Modal onClose={() => setOverlay("result")}>
        <GlassCard padding="lg">
          <div className="grid grid-cols-2 gap-6">
            <div className="text-center">
              <p className="text-[11px] uppercase tracking-[0.16em] text-[color:var(--faint)]">BEFORE</p>
              <div className="mt-4 flex justify-center">
                <ScoreRing score={lastResult.scoreBefore} size={180} />
              </div>
            </div>
            <div className="text-center">
              <p className="text-[11px] uppercase tracking-[0.16em] text-[color:var(--faint)]">AFTER</p>
              <div className="mt-4 flex justify-center">
                <ScoreRing score={lastResult.scoreAfter} size={180} />
              </div>
            </div>
          </div>
          <div className="mt-6 max-h-48 space-y-2 overflow-y-auto text-left">
            {lastResult.changes.map((change) => (
              <div key={change.id} className="flex items-center justify-between text-sm">
                <span>{change.title}</span>
                <span className={change.success ? "text-ready" : "text-danger"}>
                  {change.success ? "OK" : "Saltata"}
                </span>
              </div>
            ))}
          </div>
          <GlassButton className="mt-6 w-full" variant="secondary" onClick={() => setOverlay(null)}>
            DONE
          </GlassButton>
        </GlassCard>
      </Modal>
    );
  }

  if (overlay === "tweak-detail" && selectedTweak) {
    return (
      <TweakDetailSheet
        tweak={selectedTweak}
        developerMode={settings.developerMode || settings.showTechnicalDetails}
        onClose={() => setOverlay(null)}
      />
    );
  }

  if (overlay === "confirm-optimize") {
    return (
      <ConfirmationDialog
        title="Ottimizzare il PC?"
        body="Creiamo prima un backup. Poi applichiamo solo le modifiche sicure e compatibili con questo computer."
        confirmLabel="OPTIMIZE MY PC"
        onConfirm={() => void runOptimize()}
        onCancel={() => setOverlay(null)}
      />
    );
  }

  if (overlay === "confirm-restore") {
    return (
      <ConfirmationDialog
        title="RESTORE MY PC"
        body="Il PC tornerà alle impostazioni precedenti all’ottimizzazione."
        confirmLabel="RESTORE MY PC"
        danger
        onConfirm={() => void runRestore()}
        onCancel={() => setOverlay(null)}
      />
    );
  }

  if (overlay === "confirm-safemode") {
    return (
      <ConfirmationDialog
        title="Disattivare Safe Mode?"
        body="Safe Mode applica solo modifiche reversibili e a basso rischio. Disattivarlo è per utenti esperti."
        confirmLabel="Disattiva Safe Mode"
        danger
        onConfirm={() => {
          void updateSettings({ safeMode: false }, { allowDisableSafeMode: true });
        }}
        onCancel={() => setOverlay(null)}
      />
    );
  }

  if (overlay === "error" && lastError) {
    return (
      <ErrorPanel
        title={lastError.title}
        body={lastError.body}
        details={lastError.details}
        onRetry={() => {
          setOverlay(null);
          void refreshScan();
        }}
        onSkip={() => setOverlay(null)}
        onClose={() => setOverlay(null)}
      />
    );
  }

  return null;
}
