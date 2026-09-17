# winget-gui

A fast replacement for Windows' **Settings → Apps → Installed apps** ("Add or
remove programs"), built on [nano-ui](https://github.com/goolord/nano-ui) and
the [WinGet](https://github.com/microsoft/winget-cli) COM API, with bulk and
automated uninstallation.

https://github.com/user-attachments/assets/273b6a4b-a5d6-4e4c-a449-d92da5a5d568


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
| `bridge/` | Rust static library exposing a small C ABI over WinGet (`include/winget_bridge.h`). Bindings are generated from `winmd/Microsoft.Management.Deployment.winmd` by `bridge/gen`. |
| `src/WinGet/Bridge.hs` | Haskell FFI to the bridge: listing, uninstall/upgrade with progress and cancellation. |
| `src/WinGet/Queue.hs` | Bulk job runner shared by the GUI and CLI. |
| `src/WinGet/Select.hs` | Selection rules and rule files. |
| `app/Gui/` | The nano-ui application: state, view, and headless self-test. |
| `app/Cli.hs` | `winget-gui-cli`, the headless front end. |
| `build/Build.hs` | The [Shake](https://shakebuild.com) build: bridge, executables, vendored SDL3, `dist\`, release zip. |

## Building

Requirements: GHC 9.14 and cabal, Rust with `rustup`, `pkg-config`, and LLVM's
binutils (`llvm-readobj`, `llvm-strip`) on `PATH`. nano-ui is fetched from
GitHub at the commit pinned in `cabal.project`; SDL3 and SDL3_ttf are fetched
and checksummed by the build itself.

```powershell
cabal run build                 # bridge and both executables into dist\
cabal run build -- run          # ...and start the GUI
cabal run build -- selftest     # ...and run the scripted self-test
cabal run build -- package      # ...and zip it into release\
cabal run build -- clean
```

`dist\` holds four files -- the two stripped executables, `SDL3.dll` and
`SDL3_ttf.dll` -- and nothing else is needed: copy it anywhere with App
Installer (WinGet) present. Once `cabal run build` has produced the bridge and
fetched SDL, a plain `cabal build` works too, but it links against whatever
SDL3 `pkg-config` finds on `PATH`; alternating between the two makes cabal
reconfigure and rebuild `nano-ui-sdl` each time.

Everything Haskell and Rust is linked into the executables. The bridge is a
static library built for the `x86_64-pc-windows-gnullvm` target, whose ABI
matches the toolchain GHC links with, so there is no `winget_bridge.dll` and no
Visual C++ runtime to install. SDL3 and SDL3_ttf come from upstream's MinGW
builds, which carry FreeType and HarfBuzz inside `SDL3_ttf.dll` rather than as
the dozen further DLLs MSYS2's builds pull in.

Every build checks what `dist\` imports and fails on anything that is neither
staged there nor part of Windows, so a release cannot quietly depend on a DLL
that only exists on the machine that built it. The SDL pins in
`build/Build.hs` are tied to the ABI baked into `sdl3-bindgen-sys`, which
asserts struct sizes against the headers it compiles against; bump them
together.

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
