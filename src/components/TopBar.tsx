import { windowApi } from "../services/api";

export function TopBar({ subtitle }: { subtitle: string }) {
  return (
    <header className="drag-region flex h-14 items-center justify-between border-b border-white/5 px-6">
      <p className="text-[12px] uppercase tracking-[0.18em] text-[color:var(--faint)]">{subtitle}</p>
      <div className="no-drag flex items-center gap-1">
        <WindowButton label="—" onClick={() => void windowApi.minimize()} />
        <WindowButton label="□" onClick={() => void windowApi.toggleMaximize()} />
        <WindowButton label="×" onClick={() => void windowApi.close()} />
      </div>
    </header>
  );
}

function WindowButton({ label, onClick }: { label: string; onClick: () => void }) {
  return (
    <button
      className="grid h-8 w-8 place-items-center rounded-md text-[color:var(--muted)] hover:bg-white/8 hover:text-ink"
      onClick={onClick}
    >
      {label}
    </button>
  );
}
