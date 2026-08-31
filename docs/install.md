# Install and update OpenBX

The only supported way to **use** OpenBX is the Windows installer from GitHub Releases. Do not run `target\debug\openbx.exe` or `cargo run` as your daily app.

Releases: [https://github.com/numbfede/OpenBX/releases](https://github.com/numbfede/OpenBX/releases)

## Install (end users)

1. Open **[Releases](https://github.com/numbfede/OpenBX/releases/latest)**.
2. Download `OpenBX_<version>_x64-setup.exe` (example: `OpenBX_0.1.1_x64-setup.exe`).
3. Run the setup file.
4. If Windows SmartScreen appears, choose **More info** → **Run anyway** (the build is unsigned while the project is young).
5. Finish the installer. It installs for the current Windows user.
6. Start **OpenBX** from the Start menu — not from a `debug` folder.
7. On first launch, press **SCAN**, then **OPTIMIZE MY PC** if the score says the PC can be improved.
8. If the app asks for Windows permission (**Autorizza**), that UAC prompt is expected on the **installed** app. Confirm it. Do not do this inside `npm run tauri dev`.

Requirements: Windows 10 2004 (build 19041) or later, or Windows 11. WebView2 is already on Windows 11.

## Update (end users)

The same GitHub repository is used for every version. There is not a second repo.

1. Open OpenBX. If a newer release exists, a banner says so.
2. Click **Aggiorna**. The browser opens the latest GitHub Release.
3. Download the new `OpenBX_<version>_x64-setup.exe`.
4. Run it. It replaces the previous install. Your backups in `%LOCALAPPDATA%\OpenBX\` are kept.
5. Start OpenBX again.

You can also watch the repo: on GitHub click **Watch** → **Custom** → **Releases**.

The app only asks GitHub “what is the latest version?”. It does not send hardware or personal data. Turn the check off in **Settings → Check for updates on GitHub**.

If you installed **0.1.0** before this checker existed, update once by hand from the Releases page. Later versions will notify you in the app.

## What not to do

| Do not | Why |
| --- | --- |
| Run `src-tauri\target\debug\openbx.exe` | That build needs a local Vite server. After UAC you get `ERR_CONNECTION_REFUSED`. |
| Click **Autorizza** during `npm run tauri dev` | Windows blocks elevated apps from talking to localhost. |
| Treat the MSVC line `Creazione della libreria ...dll.lib` as a failure | Harmless linker note. |

## Developers

```bash
git clone https://github.com/numbfede/OpenBX.git
cd OpenBX
npm install
npm run tauri dev      # UI development only — press Continua, not Autorizza
npm run tauri build    # real app + installer
```

Update a local clone:

```bash
git pull
npm install
npm run tauri build
```

The installer is written to `src-tauri/target/release/bundle/nsis/`.
