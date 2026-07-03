$ErrorActionPreference = "Stop"

$root = (Resolve-Path (Join-Path $PSScriptRoot "..\..")).Path
$toolsBin = Join-Path $root ".tools\bin"
$localAnchor = Join-Path $toolsBin "anchor.exe"

if (-not (Test-Path $localAnchor)) {
    throw "Project-local Anchor CLI is not installed. Run .\install-tools.ps1 from the repository root first."
}

if (-not $env:HOME) {
    $env:HOME = $env:USERPROFILE
}

if (-not (($env:PATH -split ';') -contains $toolsBin)) {
    $env:PATH = "$toolsBin;$env:PATH"
}

Write-Host "Using project tools from $toolsBin"
Write-Host "anchor version: $(& $localAnchor --version)"

