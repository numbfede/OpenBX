import { useState } from "react";
import { GlassButton } from "./GlassButton";
import { GlassCard } from "./GlassCard";
import { Modal } from "./Modal";

export function ErrorPanel({
  title,
  body,
  details,
  onRetry,
  onSkip,
  onClose,
}: {
  title: string;
  body: string;
  details?: string;
  onRetry?: () => void;
  onSkip?: () => void;
  onClose: () => void;
}) {
  const [open, setOpen] = useState(false);

  return (
    <Modal onClose={onClose}>
      <GlassCard padding="lg">
        <p className="text-[11px] uppercase tracking-[0.2em] text-danger">Attenzione</p>
        <h2 className="mt-3 text-3xl font-medium">{title}</h2>
        <p className="mt-3 text-[15px] leading-relaxed text-[color:var(--muted)]">{body}</p>
        <div className="mt-8 flex flex-wrap gap-3">
          {onRetry ? (
            <GlassButton onClick={onRetry}>TRY AGAIN</GlassButton>
          ) : null}
          {onSkip ? (
            <GlassButton variant="secondary" onClick={onSkip}>
              SKIP
            </GlassButton>
          ) : null}
          <GlassButton variant="ghost" onClick={() => setOpen((value) => !value)}>
            VIEW DETAILS
          </GlassButton>
        </div>
        {open && details ? (
          <pre className="mt-5 overflow-x-auto rounded-btn bg-black/30 p-4 text-xs text-[color:var(--muted)]">
            {details}
          </pre>
        ) : null}
        <GlassButton className="mt-4 w-full" variant="secondary" onClick={onClose}>
          Chiudi
        </GlassButton>
      </GlassCard>
    </Modal>
  );
}
