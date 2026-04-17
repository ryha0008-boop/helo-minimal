# helomin

Minimal AI runtime environment manager. Single file (`src/main.rs`), no modules.

## Release process

1. Update `[Unreleased]` → `[x.y.z] — YYYY-MM-DD` in CHANGELOG.md
2. Bump `version` in Cargo.toml
3. `cargo build --release`
4. `cp target/release/helomin.exe /c/Users/H/bin/helomin.exe`
5. Commit, tag, push (`git tag vx.y.z && git push origin main && git push origin vx.y.z`)
6. GitHub Actions builds all four platform binaries automatically

Asset naming: `helomin-x86_64-windows.exe`, `helomin-x86_64-linux`, `helomin-aarch64-macos`, `helomin-x86_64-macos`

## Development rules

- Every user-facing change gets a CHANGELOG.md entry under `[Unreleased]` in the same commit
- Keep it minimal — no interactive mode, no API key management, no provider abstraction
- Single `src/main.rs` only. No modules, no extra files

## Architecture

**Blueprints** — stored in `config.helomin.toml` (separate from helo's `config.toml` to avoid field conflicts). Fields: `name`, `runtime`, optional `permissions`.

**Config path:**
- Windows: `%APPDATA%\helo\config\config.helomin.toml`
- Linux: `~/.config/helo/config.helomin.toml`
- macOS: `~/Library/Application Support/helo/config.helomin.toml`

## Commands

```
helomin add <name> --runtime <runtime> [--permissions bypass]
helomin run <name>
helomin list
helomin remove <name>
```

## Runtime isolation

| Runtime | Env var set | Notes |
|---------|-------------|-------|
| claude | `CLAUDE_CONFIG_DIR` | `--permissions bypass` seeds settings.json with bypassPermissions |
| pi | `PI_CODING_AGENT_DIR` | Windows: `cmd /c` |
| opencode | `OPENCODE_CONFIG` + `XDG_DATA_HOME` | both needed for full isolation |

## `--permissions bypass`

Claude only. On first `helomin run`, writes `settings.json` to the env dir:
```json
{
  "permissions": {
    "defaultMode": "bypassPermissions"
  }
}
```
`defaultMode` must be nested under `permissions` — root-level is silently ignored by Claude Code.
