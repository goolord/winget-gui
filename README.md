# winget-gui

A fast replacement for Windows' **Settings → Apps → Installed apps** ("Add or
remove programs"), built on [nano-ui](https://github.com/goolord/nano-ui) and
the [WinGet](https://github.com/microsoft/winget-cli) COM API, with bulk and
automated uninstallation.

<img width="1562" height="1002" alt="image" src="https://github.com/user-attachments/assets/4ebdc144-bd0b-419e-a92f-b2d5d78332eb" />

- **Fast**: the installed list appears as soon as WinGet's local catalog is
  read, and upgrade information is filled in afterwards in the background.
  Records are built on all cores. The list is virtualized, so only the rows on
  screen are built each frame, and the UI only redraws when something changes.

  Measured on a Ryzen 9 5900XT with 1,337 installed apps, process start
  included:

  | | Time |
  | --- | --- |
  | `winget-gui-cli list` (installed apps, sizes, dates) | 1.7 s |
  | `winget-gui-cli list --updates` (plus available upgrades) | 5.8 s |
  | `winget list` | 6.2 s |
  | windows settings | ~16.5 s | 
- **Bulk operations**: check any number of apps (click, Shift+click ranges,
  Ctrl+A, "Select shown") and uninstall or upgrade them in one go, silently or
  interactively, one at a time or several in parallel, with a live queue you
  can stop, skip, cancel, and retry.
- **Automation**: select apps by rule (exact id, name, publisher, or substring),
  save and load rule files, and run the same files headlessly with
  `winget-gui-cli`.
- **Disk view**: filter by the drive an app is installed on, with the number
  of apps and the space they take on each drive.
- **More detail than Settings**: WinGet id, source, scope, installer type,
  architecture, install location, product codes, package family names,
  whether a silent uninstaller is registered, and available upgrades.

## Layout

| Path | What it is |
| --- | --- |
| `bridge/` | Rust `cdylib` exposing a small C ABI over WinGet (`include/winget_bridge.h`). Bindings are generated from `winmd/Microsoft.Management.Deployment.winmd` by `bridge/gen`. |
| `src/WinGet/Bridge.hs` | Haskell FFI to the bridge: listing, uninstall/upgrade with progress and cancellation. |
| `src/WinGet/Queue.hs` | Bulk job runner shared by the GUI and CLI. |
| `src/WinGet/Select.hs` | Selection rules and rule files. |
| `app/Gui/` | The nano-ui application: state, view, and headless self-test. |
| `app/Cli.hs` | `winget-gui-cli`, the headless front end. |

## Building

Requirements: GHC 9.14 and cabal, Rust (MSVC toolchain), and SDL3 + SDL3_ttf
with `pkg-config` (for example from MSYS2 UCRT64). nano-ui is fetched from
GitHub at the commit pinned in `cabal.project`.

```powershell
pwsh scripts\build.ps1        # builds the bridge and both executables into dist\
pwsh scripts\build.ps1 -Run   # ...and starts the GUI
```

`dist\` is self-contained: the stripped executables, `winget_bridge.dll`,
SDL3 and SDL3_ttf, and the MSYS2 DLLs they load (found by following imports
with `llvm-readobj` or `objdump`). Copy the folder anywhere with App Installer
(WinGet) present.

## Using the CLI

```powershell
winget-gui-cli list --updates
winget-gui-cli uninstall --match toolbar --dry-run
winget-gui-cli uninstall --from bloat.txt --yes
winget-gui-cli upgrade --all --accept-agreements --yes
```

A rule file has one rule per line; `#` starts a comment:

```text
id:Mozilla.Firefox
name:Google Chrome
publisher:Contoso Ltd.
disk:D:
match:toolbar
```

`disk:` takes a drive (`D:`), `Network`, or `Unknown`. The disk comes from the
app's install location, falling back to its uninstaller's path; the GUI's disk
filter shows how many apps and how much space each drive holds. An app matches
a file when it matches any rule in it.

The exit code is the number of operations that failed (0 when all succeeded).

## How it talks to WinGet

The bridge first tries WinGet's out-of-process COM server. Current WinGet
releases only serve packaged apps from it, so an ordinary desktop process is
refused (`0x80073D54`). The bridge then uses WinGet's in-process module,
`WindowsPackageManager.dll`, from the installed App Installer package. Windows
will not load DLLs from `WindowsApps` into an unpackaged process, so the module
is copied to `%LOCALAPPDATA%\winget-gui\inproc\<App Installer version>\` and
loaded from there. Uninstallers therefore run as children of winget-gui, and
installers that need administrator rights show their own UAC prompt.

Sources whose agreements you have not accepted are skipped when checking for
upgrades, and package license agreements are only accepted when you tick the
option (or pass `--accept-agreements`).

## Testing

```powershell
winget-gui --selftest <dir>          # scripted clicks through every dialog; saves screenshots
winget-gui --screenshot main.bmp     # render the main window headlessly
$env:WINGET_GUI_TIMING = 1           # bridge prints listing phase timings to stderr
```

The self-test never starts an operation: it only opens and cancels dialogs.
