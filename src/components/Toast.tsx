import { useAppStore } from "../store/useAppStore";

const tones = {
  ready: "border-ready/20 text-ready",
  warn: "border-warn/20 text-warn",
  danger: "border-danger/20 text-danger",
};

export function ToastViewport() {
  const toasts = useAppStore((state) => state.toasts);
  const dismissToast = useAppStore((state) => state.dismissToast);

  return (
    <div className="pointer-events-none absolute bottom-6 right-6 z-50 flex w-80 flex-col gap-2">
      {toasts.map((toast) => (
        <button
          key={toast.id}
          className={`pointer-events-auto glass rounded-btn border p-4 text-left ${tones[toast.tone]}`}
          onClick={() => dismissToast(toast.id)}
        >
          <div className="text-sm font-medium text-ink">{toast.title}</div>
          {toast.body ? <div className="mt-1 text-xs text-[color:var(--muted)]">{toast.body}</div> : null}
        </button>
      ))}
    </div>
  );
}
