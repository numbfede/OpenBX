import type { ScreenId } from "../services/types";

const items: { id: ScreenId; label: string }[] = [
  { id: "home", label: "Home" },
  { id: "categories", label: "Categorie" },
  { id: "gamemode", label: "Game Mode" },
  { id: "games", label: "Giochi" },
  { id: "restore", label: "Ripristina" },
  { id: "settings", label: "Impostazioni" },
];

export function Sidebar({
  current,
  onNavigate,
}: {
  current: ScreenId;
  onNavigate: (screen: ScreenId) => void;
}) {
  return (
    <aside className="flex w-[220px] shrink-0 flex-col border-r border-white/5 px-5 py-6">
      <div>
        <p className="text-[11px] uppercase tracking-[0.22em] text-[color:var(--faint)]">OpenBX</p>
        <h1 className="mt-2 text-xl font-medium tracking-tight">Optimizer</h1>
      </div>
      <nav className="mt-10 flex flex-1 flex-col gap-1">
        {items.map((item, index) => {
          const active = item.id === current;
          return (
            <button
              key={item.id}
              className={`rounded-btn px-3 py-2.5 text-left text-sm transition duration-200 ease-openbx ${
                active ? "bg-white/8 text-ink" : "text-[color:var(--muted)] hover:bg-white/5 hover:text-ink"
              }`}
              onClick={() => onNavigate(item.id)}
            >
              <span className="mr-3 text-[11px] text-[color:var(--faint)]">
                {String(index + 1).padStart(2, "0")}
              </span>
              {item.label}
            </button>
          );
        })}
      </nav>
      <p className="text-[11px] leading-relaxed text-[color:var(--faint)]">
        Non devi sapere come funziona Windows. Devi solo sapere cosa vuoi ottenere.
      </p>
    </aside>
  );
}
