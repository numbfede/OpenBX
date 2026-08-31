# Contributing to OpenBX

Thank you for helping. OpenBX is intentionally small. We would rather ship ten honest tweaks than two hundred myths.

The current UI language is Italian (with English CTAs). Documentation, issues, and pull requests should be written in **English** so anyone can contribute.

## Ground rules

A new optimization is welcome only if **all** of these are true:

1. You can explain exactly which Windows setting it changes.
2. There is a reasonable, documented benefit — not a YouTube/TikTok myth.
3. It is compatible with a clearly defined set of Windows versions and hardware.
4. It is reversible, and `rollback()` restores the previous value.
5. You cite an official source (Microsoft Learn, Microsoft Support, NVIDIA, AMD, Intel).

If a change cannot meet that bar, do not implement it.

## Never contribute

- Network changes (DHCP, IP, gateway, DNS, TCP, NIC)
- Fake FPS / latency claims
- Disabling Windows Security or Windows Update
- Service “massacre” lists
- HPET, global timer resolution, pagefile, core parking, BCDEdit
- Telemetry that is on by default

## Adding a tweak

1. Implement `detect` / `apply` / `verify` / `rollback` in [`src-tauri/src/tweaks/catalog.rs`](src-tauri/src/tweaks/catalog.rs).
2. Keep [`tweaks/catalog.yaml`](tweaks/catalog.yaml) and [`docs/tweaks.md`](docs/tweaks.md) in sync.
3. Hide the tweak when it is not applicable. Do not show AMD-only changes on Intel.
4. Keep user-facing copy non-technical. Registry paths belong in Developer mode.
5. Add a unit test for any value-decoding logic.
6. Run `cargo test --manifest-path src-tauri/Cargo.toml` and `npm run build`.

User-facing strings currently live in Italian inside the Rust catalog. If you add copy, include an English description in `docs/tweaks.md`.

## Pull requests

- Keep diffs focused. Do not mix a tweak with an unrelated UI rewrite.
- Describe **why** the change is safe, not only what you edited.
- Link the official documentation you relied on.
- Do not commit `node_modules`, `src-tauri/target`, or `.env` files.

## Development setup

Install and update instructions for testers: [docs/install.md](docs/install.md).

See the [README](README.md#build-from-source) for the compiler toolchain.

## Code of conduct

This project follows the [Contributor Covenant](CODE_OF_CONDUCT.md).
