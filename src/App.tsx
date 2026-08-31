import { useEffect } from "react";
import { Sidebar } from "./components/Sidebar";
import { ToastViewport } from "./components/Toast";
import { TopBar } from "./components/TopBar";
import { CategoriesScreen } from "./screens/CategoriesScreen";
import { GameModeScreen } from "./screens/GameModeScreen";
import { GamesScreen } from "./screens/GamesScreen";
import { HomeScreen } from "./screens/HomeScreen";
import { Overlays } from "./screens/Overlays";
import { RestoreScreen } from "./screens/RestoreScreen";
import { SettingsScreen } from "./screens/SettingsScreen";
import { useAppStore } from "./store/useAppStore";

const titles = {
  home: "( 01 ) — Home",
  categories: "( 02 ) — Categorie",
  gamemode: "( 03 ) — Game Mode",
  games: "( 04 ) — Giochi",
  restore: "( 05 ) — Ripristina",
  settings: "( 06 ) — Impostazioni",
};

export default function App() {
  const screen = useAppStore((state) => state.screen);
  const setScreen = useAppStore((state) => state.setScreen);
  const bootstrap = useAppStore((state) => state.bootstrap);
  const ready = useAppStore((state) => state.ready);
  const system = useAppStore((state) => state.system);

  useEffect(() => {
    void bootstrap();
  }, [bootstrap]);

  return (
    <div className="flex h-full overflow-hidden bg-canvas text-ink">
      <Sidebar current={screen} onNavigate={setScreen} />
      <div className="flex min-w-0 flex-1 flex-col">
        <TopBar subtitle={system && !system.isElevated ? "Permessi limitati · alcune modifiche restano in attesa" : titles[screen]} />
        <main className="relative min-h-0 flex-1 overflow-y-auto px-8">
          {!ready ? (
            <div className="grid h-full place-items-center text-sm text-[color:var(--muted)]">
              Controlliamo il tuo PC...
            </div>
          ) : (
            <>
              {screen === "home" ? <HomeScreen /> : null}
              {screen === "categories" ? <CategoriesScreen /> : null}
              {screen === "gamemode" ? <GameModeScreen /> : null}
              {screen === "games" ? <GamesScreen /> : null}
              {screen === "restore" ? <RestoreScreen /> : null}
              {screen === "settings" ? <SettingsScreen /> : null}
            </>
          )}
          <Overlays />
          <ToastViewport />
        </main>
      </div>
    </div>
  );
}
