import { GlassButton } from "../components/GlassButton";
import { GlassCard } from "../components/GlassCard";
import { useAppStore } from "../store/useAppStore";
import { formatRestoreDate, plural } from "../utils/format";

export function RestoreScreen() {
  const backups = useAppStore((state) => state.backups);
  const requestRestore = useAppStore((state) => state.requestRestore);

  return (
    <div className="mx-auto max-w-3xl py-8">
      <p className="text-[11px] uppercase tracking-[0.22em] text-[color:var(--faint)]">( 05 ) — Restore</p>
      <h2 className="mt-3 text-4xl font-medium tracking-tight">RESTORE</h2>
      <p className="mt-3 max-w-xl text-[15px] text-[color:var(--muted)]">
        Ripristina le modifiche effettuate da Optimizer.
      </p>
      <div className="mt-8 space-y-3">
        {backups.length === 0 ? (
          <GlassCard>
            <p className="text-sm text-[color:var(--muted)]">
              Nessun punto di ripristino. OpenBX crea un backup automatico prima di ogni modifica.
            </p>
          </GlassCard>
        ) : (
          backups.map((backup) => (
            <GlassCard key={backup.id} className="flex items-center justify-between gap-4">
              <div>
                <p className="text-[11px] uppercase tracking-[0.16em] text-[color:var(--faint)]">Restore Point</p>
                <h3 className="mt-2 text-xl font-medium">{formatRestoreDate(backup.createdAt)}</h3>
                <p className="mt-1 text-sm text-[color:var(--muted)]">
                  {plural(backup.changeCount, "modifica", "modifiche")} · {backup.label}
                </p>
              </div>
              <GlassButton onClick={() => requestRestore(backup.id)}>RESTORE</GlassButton>
            </GlassCard>
          ))
        )}
      </div>
    </div>
  );
}
