export function ProgressBar({ current, total }: { current: number; total: number }) {
  const width = total <= 0 ? 0 : Math.min(100, Math.round((current / total) * 100));
  return (
    <div className="h-1.5 w-full overflow-hidden rounded-full bg-white/10">
      <div
        className="h-full rounded-full bg-cream transition-all duration-300 ease-openbx"
        style={{ width: `${width}%` }}
      />
    </div>
  );
}
