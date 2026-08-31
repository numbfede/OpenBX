type Tone = "ready" | "warn" | "muted";

const tones: Record<Tone, string> = {
  ready: "text-ready bg-ready/10",
  warn: "text-warn bg-warn/10",
  muted: "text-[color:var(--muted)] bg-white/5",
};

export function StatusBadge({ label, tone }: { label: string; tone: Tone }) {
  return (
    <span className={`inline-flex items-center rounded-full px-3 py-1 text-[11px] uppercase tracking-[0.16em] ${tones[tone]}`}>
      {label}
    </span>
  );
}
