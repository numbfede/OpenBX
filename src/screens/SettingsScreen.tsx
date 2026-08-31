import type { ReactNode } from "react";
import { GlassButton } from "../components/GlassButton";
import { GlassCard } from "../components/GlassCard";
import { useAppStore } from "../store/useAppStore";
import type { AppSettings } from "../services/types";

export function SettingsScreen() {
  const settings = useAppStore((state) => state.settings);
  const updateSettings = useAppStore((state) => state.updateSettings);
  const exportLogs = useAppStore((state) => state.exportLogs);

  return (
    <div className="mx-auto max-w-2xl space-y-6 py-8">
      <div>
        <p className="text-[11px] uppercase tracking-[0.22em] text-[color:var(--faint)]">( 06 ) — Settings</p>
        <h2 className="mt-3 text-4xl font-medium tracking-tight">Impostazioni</h2>
      </div>
      <Group title="General">
        <Toggle label="Start with Windows" checked={settings.startWithWindows} onChange={(startWithWindows) => void updateSettings({ startWithWindows })} />
        <Toggle label="Minimize to tray" checked={settings.minimizeToTray} onChange={(minimizeToTray) => void updateSettings({ minimizeToTray })} />
        <Toggle label="Notifications" checked={settings.notifications} onChange={(notifications) => void updateSettings({ notifications })} />
        <Row label="Theme">
          <div className="flex gap-2">
            {(["dark", "light"] as AppSettings["theme"][]).map((theme) => (
              <GlassButton
                key={theme}
                variant={settings.theme === theme ? "primary" : "secondary"}
                onClick={() => void updateSettings({ theme })}
              >
                {theme}
              </GlassButton>
            ))}
          </div>
        </Row>
      </Group>
      <Group title="Optimization">
        <Toggle label="Create backup automatically" checked={settings.createBackupAutomatically} onChange={(createBackupAutomatically) => void updateSettings({ createBackupAutomatically })} />
        <Toggle label="Ask before applying changes" checked={settings.askBeforeApplying} onChange={(askBeforeApplying) => void updateSettings({ askBeforeApplying })} />
        <Toggle label="Safe Mode" checked={settings.safeMode} onChange={(safeMode) => void updateSettings({ safeMode })} />
      </Group>
      <Group title="Advanced">
        <Toggle label="Developer mode" checked={settings.developerMode} onChange={(developerMode) => void updateSettings({ developerMode })} />
        <Toggle label="Show technical details" checked={settings.showTechnicalDetails} onChange={(showTechnicalDetails) => void updateSettings({ showTechnicalDetails })} />
        <GlassButton variant="secondary" onClick={() => void exportLogs()}>
          Export logs
        </GlassButton>
      </Group>
    </div>
  );
}

function Group({ title, children }: { title: string; children: ReactNode }) {
  return (
    <GlassCard>
      <h3 className="text-sm uppercase tracking-[0.16em] text-[color:var(--faint)]">{title}</h3>
      <div className="mt-4 space-y-4">{children}</div>
    </GlassCard>
  );
}

function Toggle({
  label,
  checked,
  onChange,
}: {
  label: string;
  checked: boolean;
  onChange: (value: boolean) => void;
}) {
  return (
    <button className="flex w-full items-center justify-between text-left" onClick={() => onChange(!checked)}>
      <span>{label}</span>
      <span className={`h-6 w-11 rounded-full p-0.5 transition ${checked ? "bg-cream" : "bg-white/15"}`}>
        <span className={`block h-5 w-5 rounded-full bg-canvas transition ${checked ? "translate-x-5" : ""}`} />
      </span>
    </button>
  );
}

function Row({ label, children }: { label: string; children: ReactNode }) {
  return (
    <div className="flex items-center justify-between gap-4">
      <span>{label}</span>
      {children}
    </div>
  );
}
