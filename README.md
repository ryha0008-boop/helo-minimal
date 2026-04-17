# helomin

Minimal AI runtime environment manager. Two commands.

Like Python venv, but for AI runtimes — each agent gets its own isolated config directory so multiple agents can run on the same machine without conflicting.

## How it works

Each AI runtime checks an environment variable at startup and uses that directory for all config, sessions, and memory instead of the global default.

| Runtime | Env var set | Default (without helomin) |
|---------|-------------|---------------------------|
| claude | `CLAUDE_CONFIG_DIR` | `~/.claude/` |
| pi | `PI_CODING_AGENT_DIR` | `~/.pi/` |
| opencode | `XDG_DATA_HOME` + `OPENCODE_CONFIG` | `~/.local/share/opencode/` |

`helomin run` sets the right variable and launches the binary. That's it.

## Install

Download the binary for your platform from [Releases](https://github.com/ryha0008-boop/helominimal/releases/latest):

| Platform | File |
|----------|------|
| Windows x86_64 | `helomin-x86_64-windows.exe` |
| Linux x86_64 | `helomin-x86_64-linux` |
| macOS ARM | `helomin-aarch64-macos` |
| macOS Intel | `helomin-x86_64-macos` |

### Windows

```bash
# Rename and move to a folder in your PATH, e.g. C:\Users\<you>\bin\
mv helomin-x86_64-windows.exe helomin.exe
```

> **Note:** Use Git Bash on Windows, not PowerShell. PowerShell may fail to resolve `helomin` as a command even when it's on PATH.

### Linux / macOS

```bash
chmod +x helomin-x86_64-linux   # or the correct platform binary
sudo mv helomin-x86_64-linux /usr/local/bin/helomin
helomin --version
```

### Build from source

Requires [Rust](https://rustup.rs/).

```bash
git clone https://github.com/ryha0008-boop/helominimal.git
cd helominimal
cargo build --release
# Binary at target/release/helomin (or helomin.exe on Windows)
```

## Usage

### 1. Register a blueprint

```bash
helomin add <name> --runtime <runtime>
```

Runtimes: `claude`, `pi`, `opencode`

```bash
helomin add myagent --runtime claude
helomin add reviewer --runtime opencode
```

**Claude only:** add `--permissions bypass` to seed `settings.json` with `bypassPermissions` on first run — skips Claude Code's interactive permission prompts:

```bash
helomin add myagent --runtime claude --permissions bypass
```

Blueprints are stored globally in your config directory. Define once, use in any project.

### 2. Run in a project

```bash
cd your-project
helomin run <name>
```

On first run, creates an isolated env directory (`.claude-env-myagent/`, `.pi-env-myagent/`, etc.) in the current directory and launches the runtime pointed at it. Subsequent runs reuse the same directory.

```bash
cd my-project
helomin run myagent    # launches claude with CLAUDE_CONFIG_DIR=.claude-env-myagent/
```

### That's it

For API keys, provider config, model selection, and everything else — configure them directly inside the env dir or follow the runtime's own documentation. helomin only handles the isolation.

## Config location

Blueprints are stored in:

| Platform | Path |
|----------|------|
| Windows | `%APPDATA%\helo\config\config.helomin.toml` |
| Linux | `~/.config/helo/config.helomin.toml` |
| macOS | `~/Library/Application Support/helo/config.helomin.toml` |

## Compared to helo

[helo](https://github.com/ryha0008-boop/helo-win) is the full version — adds API key management, provider presets, interactive mode, self-update, shell completions, and more. helomin is the core mechanism only, for users who want full manual control.
