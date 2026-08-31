import { GlassButton } from "./GlassButton";
import { GlassCard } from "./GlassCard";
import { Modal } from "./Modal";

export function ConfirmationDialog({
  title,
  body,
  confirmLabel,
  danger = false,
  onConfirm,
  onCancel,
}: {
  title: string;
  body: string;
  confirmLabel: string;
  danger?: boolean;
  onConfirm: () => void;
  onCancel: () => void;
}) {
  return (
    <Modal onClose={onCancel}>
      <GlassCard padding="lg">
        <p className="text-[11px] uppercase tracking-[0.2em] text-[color:var(--faint)]">( 00 ) — Conferma</p>
        <h2 className="mt-4 text-3xl font-medium tracking-tight">{title}</h2>
        <p className="mt-3 text-[15px] leading-relaxed text-[color:var(--muted)]">{body}</p>
        <div className="mt-8 flex gap-3">
          <GlassButton className="flex-1" variant={danger ? "danger" : "primary"} onClick={onConfirm}>
            {confirmLabel}
          </GlassButton>
          <GlassButton className="flex-1" variant="secondary" onClick={onCancel}>
            Annulla
          </GlassButton>
        </div>
      </GlassCard>
    </Modal>
  );
}
