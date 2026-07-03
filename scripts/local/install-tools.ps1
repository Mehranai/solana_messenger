$ErrorActionPreference = "Stop"

$root = (Resolve-Path (Join-Path $PSScriptRoot "..\..")).Path
$toolsDir = Join-Path $root ".tools"

if (-not $env:HOME) {
    $env:HOME = $env:USERPROFILE
}

cargo install `
    --root $toolsDir `
    --force `
    anchor-cli `
    --version 0.30.1

Write-Host "Installed Anchor CLI 0.30.1 at $toolsDir\bin\anchor.exe"
