import { GlassButton } from "../components/GlassButton";
import { ScoreRing } from "../components/ScoreRing";
import { SystemInfoCard } from "../components/SystemInfoCard";
import { useAppStore } from "../store/useAppStore";
import { plural } from "../utils/format";

export function HomeScreen() {
  const scan = useAppStore((state) => state.scan);
  const system = useAppStore((state) => state.system);
  const backups = useAppStore((state) => state.backups);
  const requestOptimize = useAppStore((state) => state.requestOptimize);
  const setScreen = useAppStore((state) => state.setScreen);
  const refreshScan = useAppStore((state) => state.refreshScan);
  const busy = useAppStore((state) => state.busy);

  return (
    <div className="mx-auto flex min-h-full max-w-[720px] flex-col items-center justify-center py-8">
      <p className="text-[11px] uppercase tracking-[0.22em] text-[color:var(--faint)]">( 01 ) — PC Status</p>
      <div className="mt-8">
        <ScoreRing score={scan?.score ?? null} />
      </div>
      <p className="mt-8 text-sm text-[color:var(--muted)]">
        {scan
          ? `Abbiamo controllato ${plural(scan.applicable, "impostazione Windows", "impostazioni Windows")}. Non è un aumento di FPS.`
          : "Premi OPTIMIZE dopo il controllo del PC."}
      </p>
      {scan ? (
        <div className="mt-3 flex gap-6 text-sm">
          <span className="text-ready">✓ {scan.optimized} già ottimizzate</span>
          <span className="text-warn">⚠ {scan.pending} da ottimizzare</span>
        </div>
      ) : null}
      <GlassButton
        className="mt-10 w-full max-w-md"
        size="lg"
        disabled={busy || !scan || scan.pending === 0}
        onClick={() => requestOptimize()}
      >
        OPTIMIZE MY PC
      </GlassButton>
      <div className="mt-4 flex gap-3">
        <GlassButton variant="ghost" onClick={() => void refreshScan()}>
          Controlla di nuovo
        </GlassButton>
        {backups.length > 0 ? (
          <GlassButton variant="ghost" onClick={() => setScreen("restore")}>
            RESTORE
          </GlassButton>
        ) : null}
      </div>
      {system ? (
        <div className="mt-12 grid w-full grid-cols-2 gap-3">
          <SystemInfoCard label="CPU" value={system.cpu} />
          <SystemInfoCard label="GPU" value={system.gpu} />
          <SystemInfoCard label="RAM" value={`${system.ramGb} GB`} />
          <SystemInfoCard label="Windows" value={system.windows} />
        </div>
      ) : null}
    </div>
  );
}
