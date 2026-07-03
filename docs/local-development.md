# Local Development On Windows

This project is pinned for the local Solana toolchain you reported:

- `solana-cli 1.18.20`
- `anchor-cli 0.30.1`
- `anchor-lang 0.30.1`

PowerShell usually does not define `HOME`, but Solana's SBF builder expects it. First install the project-local Anchor CLI from the repository root:

```powershell
.\install-tools.ps1
```

Explanation: This installs Anchor CLI `0.30.1` into `.tools\bin\anchor.exe`, so this project can use the Solana `1.18.20`-compatible Anchor version without replacing your global Anchor install.

Then use the helper script instead of running `anchor build` directly:

```powershell
.\build.ps1
```

Explanation: The script sets `HOME` to `USERPROFILE`, points Solana to localhost, uses your default Solana wallet, and runs `anchor build`.

To build and run tests against an already-running local validator:

```powershell
.\build.ps1 -Test
```

Explanation: This airdrops local SOL to your configured wallet and then runs `anchor test --skip-local-validator`, so start `solana-test-validator` in another terminal first.

If deployment fails with insufficient funds, manually airdrop to the reported account:

```powershell
solana airdrop 10 <ACCOUNT_ADDRESS> --url localhost
```

Explanation: Local SOL only exists on your local validator. It has no mainnet value and is used to pay rent/fees while deploying and testing.

If you really want to type `anchor build` in the current terminal, load the project tools into your PowerShell session first:

```powershell
. .\scripts\local\use-tools.ps1
anchor build
```

Explanation: The leading dot sources the script, prepending `.tools\bin` to `PATH` for the current terminal so `anchor` resolves to `.tools\bin\anchor.exe`.

If you prefer to replace the global Anchor CLI instead, run:

```powershell
cargo install --force anchor-cli --version 0.30.1
```

Explanation: Anchor CLI `1.1.2` hardcodes `cargo build-sbf --tools-version v1.52`; Anchor CLI `0.30.1` uses the older build path expected by Solana `1.18.20`.

After changing the Anchor JS version, refresh local JavaScript dependencies:

```powershell
npm install
```

Explanation: This updates `node_modules` and refreshes `package-lock.json` so the TypeScript tests use `@coral-xyz/anchor` `0.30.1`.

The Anchor test script uses npm:

```toml
[scripts]
test = "npm run test:ts"
```

Explanation: `anchor test` executes this command after deploying the program. Yarn is not required.

If the build reports an access error under `C:\Users\mehra\.cache\solana\v1.52`, close any running build/validator terminals and remove that stale cache directory:

```powershell
Remove-Item -LiteralPath "$env:USERPROFILE\.cache\solana\v1.52" -Recurse -Force
```

Explanation: That directory was created by the newer Anchor build path. It is safe to remove because Solana recreates platform-tools cache folders when needed.
