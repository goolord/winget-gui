# winget-gui

A fast replacement for Windows' **Settings → Apps → Installed apps** ("Add or remove programs"), built on [nano-ui](https://github.com/goolord/nano-ui) and the [WinGet](https://github.com/microsoft/winget-cli) COM API, with bulk and automated uninstallation.

https://github.com/user-attachments/assets/bdc2e82c-3d45-41df-ab98-6792e3835bcb

- **Fast**: Multi-threaded catalog reads, background upgrade checks, and virtualized list rendering.
- **Bulk operations**: Select multiple apps to uninstall or upgrade in parallel or sequentially, with a live controllable queue.
- **Automation**: Headless batch operations via `winget-gui-cli` using rule files or CLI filters.
- **Disk view**: Filter apps by drive, inspect occupied space per drive, and sort by size.
- **Rich metadata**: View installer type, scopes, architecture, product codes, silent uninstall availability, and pending upgrades.

## Layout

| Path | What it is |
| --- | --- |
| `bridge/` | Rust static library exposing a C ABI over WinGet via `winmd` bindings. |
| `src/WinGet/` | Haskell core: bridge FFI, bulk job runner, and selection rule parser. |
| `app/` | Front ends: nano-ui GUI (`Gui/`) and headless CLI (`Cli.hs`). |
| `build/Build.hs` | Shake build script for compiling, vendoring SDL3, and packaging. |

## Building

**Requirements:** GHC 9.14 + Cabal, Rust (`rustup`), `pkg-config`, LLVM binutils (`llvm-readobj`, `llvm-strip`), and `upx` on `PATH`.

```powershell
cabal run build          # Build bridge and staged binaries to dist\
cabal run build -- run   # Build and launch GUI
cabal run build -- clean
```

## Rules

Rules let you select apps automatically in both the GUI and CLI. An app matches a ruleset if it satisfies any rule (OR logic).
```
id:Mozilla.Firefox       # Matches exact WinGet package ID
name:Google Chrome       # Matches package display name
publisher:Contoso Ltd.   # Matches package publisher
disk:D:                  # Matches install or uninstaller drive (e.g. D:, Network, Unknown)
match:toolbar            # Substring match across ID, name, and publisher
```
