param(
    [switch]$Test
)

$ErrorActionPreference = "Stop"

& (Join-Path $PSScriptRoot "scripts\local\build.ps1") -Test:$Test

