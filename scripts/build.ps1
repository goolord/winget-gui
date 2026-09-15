# Builds the Rust WinGet bridge and the Haskell executables, then stages a
# runnable folder in dist\ with every DLL the programs load.
#
#   pwsh scripts\build.ps1            # release build into dist\
#   pwsh scripts\build.ps1 -Run       # ...and launch the GUI
param(
    [switch]$Run
)

$ErrorActionPreference = "Stop"
$root = Resolve-Path "$PSScriptRoot\.."
Push-Location $root
try {
    Write-Host "Building bridge (cargo)..." -ForegroundColor Cyan
    cargo build --release --manifest-path bridge\Cargo.toml
    if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }

    # GHC links with lld in MinGW mode, which looks for <name>.lib, while
    # rustc names the MSVC import library <name>.dll.lib.
    $release = "bridge\target\release"
    Copy-Item "$release\winget_bridge.dll.lib" "$release\winget_bridge.lib" -Force

    Write-Host "Building GUI and CLI (cabal)..." -ForegroundColor Cyan
    $env:PATH = "$root\$release;$env:PATH"
    cabal build exe:winget-gui exe:winget-gui-cli
    if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }

    $dist = Join-Path $root "dist"
    New-Item -ItemType Directory -Force $dist | Out-Null
    foreach ($exe in "winget-gui", "winget-gui-cli") {
        Copy-Item (cabal list-bin $exe).Trim() $dist -Force
    }
    Copy-Item "$release\winget_bridge.dll" $dist -Force

    # SDL3 and SDL3_ttf live wherever pkg-config found them (for example MSYS2
    # UCRT64). Copy them, then follow imports recursively and copy every DLL
    # that comes from that same directory (freetype, harfbuzz, the MinGW
    # runtime, ...). System DLLs are left alone.
    $sdlBin = Join-Path (& pkg-config --variable=prefix sdl3).Trim() "bin"
    foreach ($dll in "SDL3.dll", "SDL3_ttf.dll") {
        Copy-Item (Join-Path $sdlBin $dll) $dist -Force
    }
    $readobj = Get-Command llvm-readobj -ErrorAction SilentlyContinue
    $objdump = Get-Command objdump -ErrorAction SilentlyContinue
    if (-not ($readobj -or $objdump)) {
        Write-Warning "Neither llvm-readobj nor objdump is on PATH; DLL dependencies of SDL3 were not staged."
    }
    else {
        $seen = @{}
        $pending = [System.Collections.Generic.Queue[string]]::new()
        Get-ChildItem "$dist\*" -Include *.dll, *.exe -File | ForEach-Object {
            $seen[$_.Name.ToLowerInvariant()] = $true
            $pending.Enqueue($_.FullName)
        }
        while ($pending.Count -gt 0) {
            $file = $pending.Dequeue()
            $lines = if ($readobj) { & $readobj.Source --coff-imports $file 2>$null } else { & $objdump.Source -p $file 2>$null }
            foreach ($line in $lines) {
                if ($line -match "^\s*(?:DLL )?Name:\s*(\S+\.dll)\s*$") {
                    $name = $Matches[1]
                    $key = $name.ToLowerInvariant()
                    if ($seen.ContainsKey($key)) { continue }
                    $seen[$key] = $true
                    $candidate = Join-Path $sdlBin $name
                    if (Test-Path $candidate) {
                        Copy-Item $candidate $dist -Force
                        $pending.Enqueue((Join-Path $dist $name))
                    }
                }
            }
        }
    }

    # GHC executables carry full symbol tables. Strip the staged copies only;
    # the ones under dist-newstyle keep symbols for debugging.
    $strip = Get-Command llvm-strip, strip -ErrorAction SilentlyContinue | Select-Object -First 1
    if ($strip) {
        foreach ($exe in "winget-gui.exe", "winget-gui-cli.exe") {
            & $strip.Source --strip-all (Join-Path $dist $exe)
        }
    }

    Write-Host "Staged $dist" -ForegroundColor Green
    if ($Run) { & (Join-Path $dist "winget-gui.exe") }
}
finally {
    Pop-Location
}
