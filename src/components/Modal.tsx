import type { ReactNode } from "react";

export function Modal({
  children,
  onClose,
}: {
  children: ReactNode;
  onClose?: () => void;
}) {
  return (
    <div className="absolute inset-0 z-40 grid place-items-center bg-black/50 p-6 backdrop-blur-md">
      <button className="absolute inset-0 cursor-default" aria-label="Chiudi" onClick={onClose} />
      <div className="relative z-10 w-full max-w-lg">{children}</div>
    </div>
  );
}
