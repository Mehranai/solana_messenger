param(
    [switch]$Test
)

$ErrorActionPreference = "Stop"

if (-not $env:HOME) {
    $env:HOME = $env:USERPROFILE
}

if (-not $env:ANCHOR_WALLET) {
    $env:ANCHOR_WALLET = Join-Path $env:USERPROFILE ".config\solana\id.json"
}

$root = (Resolve-Path (Join-Path $PSScriptRoot "..\..")).Path
$localAnchor = Join-Path $root ".tools\bin\anchor.exe"

if (-not (Test-Path $localAnchor)) {
    throw "Project-local Anchor CLI is not installed. Run .\install-tools.ps1 from the repository root first."
}

solana config set --url localhost --keypair $env:ANCHOR_WALLET | Out-Host

& $localAnchor build

if ($Test) {
    if (-not (Test-Path (Join-Path $root "node_modules\.bin\ts-mocha.cmd")) -and -not (Test-Path (Join-Path $root "node_modules\.bin\ts-mocha"))) {
        throw "JavaScript test dependencies are missing. Run npm install from the repository root, then retry .\build.ps1 -Test."
    }

    $walletPubkey = solana-keygen pubkey $env:ANCHOR_WALLET
    $previousErrorActionPreference = $ErrorActionPreference
    $previousNativePreference = $null
    $hasNativePreference = Test-Path Variable:\PSNativeCommandUseErrorActionPreference

    try {
        $ErrorActionPreference = "Continue"
        if ($hasNativePreference) {
            $previousNativePreference = $PSNativeCommandUseErrorActionPreference
            $PSNativeCommandUseErrorActionPreference = $false
        }

        $airdropOutput = & solana --url localhost airdrop 10 $walletPubkey 2>&1
        $airdropExitCode = $LASTEXITCODE
    }
    finally {
        $ErrorActionPreference = $previousErrorActionPreference
        if ($hasNativePreference) {
            $PSNativeCommandUseErrorActionPreference = $previousNativePreference
        }
    }

    if ($airdropExitCode -eq 0) {
        $airdropOutput | Out-Host
    } else {
        $airdropOutput | Out-Host
        Write-Warning "Airdrop failed or was rate-limited. Continuing because the wallet may already have enough local SOL."
    }

    & $localAnchor test --skip-local-validator
}
