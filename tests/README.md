# Tests

Automated tests live next to the Rust modules:

```bash
cargo test --manifest-path src-tauri/Cargo.toml
```

They cover:

- score math (ratio of real settings, never an invented number)
- NTFS last-access encoding
- Game Mode bundles (Streaming is less aggressive than Competitive)
- backup serialization
- Steam ACF / VDF parsers
- non-applicable tweaks excluded from the score

Frontend typecheck and production bundle:

```bash
npm run build
```

## Manual smoke checklist

1. Open the app. The Home score is empty until SCAN.
2. SCAN reads real CPU / GPU / RAM / Windows.
3. OPTIMIZE MY PC creates a backup, applies only pending SAFE tweaks, then verifies.
4. Home score changes only if `verify()` passed.
5. RESTORE returns the previous values.
6. A laptop on battery does not offer the High Performance plan.
7. No network settings exist anywhere in the UI.
8. Developer mode reveals registry paths; the default UI does not.
