import { api } from "../services/api";
import { useAppStore } from "../store/useAppStore";
import { GlassButton } from "./GlassButton";

export function UpdateBanner() {
  const update = useAppStore((state) => state.updateInfo);
  const dismissed = useAppStore((state) => state.updateDismissed);
  const dismissUpdate = useAppStore((state) => state.dismissUpdate);

  if (!update?.available || dismissed || !update.latestVersion) return null;

  return (
    <div className="flex items-center justify-between gap-3 border-b border-white/5 bg-warn/10 px-6 py-2.5">
      <p className="text-sm text-ink">
        Stai usando la versione {update.currentVersion}. È disponibile{" "}
        <span className="font-medium">{update.latestVersion}</span>.
      </p>
      <div className="flex shrink-0 gap-2">
        <GlassButton
          variant="primary"
          onClick={() => void api.openUrl(update.releaseUrl || "https://github.com/numbfede/OpenBX/releases/latest")}
        >
          Aggiorna
        </GlassButton>
        <GlassButton variant="ghost" onClick={dismissUpdate}>
          Dopo
        </GlassButton>
      </div>
    </div>
  );
}
