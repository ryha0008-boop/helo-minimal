# helo-min

Minimal AI runtime environment manager. Two commands.

Like Python venv, but for AI runtimes — each agent gets its own isolated config directory so multiple agents can run on the same machine without conflicting.

## How it works

Each AI runtime checks an environment variable at startup and uses that directory for all config, sessions, and memory instead of the global default.

| Runtime | Env var set | Default (without helo-min) |
|---------|-------------|---------------------------|
| claude | `CLAUDE_CONFIG_DIR` | `~/.claude/` |
| pi | `PI_CODING_AGENT_DIR` | `~/.pi/` |
| opencode | `XDG_DATA_HOME` + `OPENCODE_CONFIG` | `~/.local/share/opencode/` |

`helo-min run` sets the right variable and launches the binary. That's it.

## Install

Download the binary for your platform from [Releases](https://github.com/ryha0008-boop/helo-minimal/releases/latest):

| Platform | File |
|----------|------|
| Windows x86_64 | `helo-min-x86_64-windows.exe` |
| Linux x86_64 | `helo-min-x86_64-linux` |
| macOS ARM | `helo-min-aarch64-macos` |
| macOS Intel | `helo-min-x86_64-macos` |

### Windows

```powershell
# Rename and move to a folder in your PATH, e.g. C:\Users\<you>\bin\
mv helo-min-x86_64-windows.exe helo-min.exe
# Move to PATH folder, then verify:
helo-min --version
```

### Linux / macOS

```bash
chmod +x helo-min-x86_64-linux   # or the correct platform binary
sudo mv helo-min-x86_64-linux /usr/local/bin/helo-min
helo-min --version
```

### Build from source

Requires [Rust](https://rustup.rs/).

```bash
git clone https://github.com/ryha0008-boop/helo-minimal.git
cd helo-minimal
cargo build --release
# Binary at target/release/helo-min (or helo-min.exe on Windows)
```

## Usage

### 1. Register a blueprint

```bash
helo-min add <name> --runtime <runtime>
```

Runtimes: `claude`, `pi`, `opencode`

```bash
helo-min add myagent --runtime claude
helo-min add reviewer --runtime opencode
```

Blueprints are stored globally in your config directory. Define once, use in any project.

### 2. Run in a project

```bash
cd your-project
helo-min run <name>
```

On first run, creates an isolated env directory (`.claude-env-myagent/`, `.pi-env-myagent/`, etc.) in the current directory and launches the runtime pointed at it. Subsequent runs reuse the same directory.

```bash
cd my-project
helo-min run myagent    # launches claude with CLAUDE_CONFIG_DIR=.claude-env-myagent/
```

### That's it

For API keys, provider config, model selection, and everything else — configure them directly inside the env dir or follow the runtime's own documentation. helo-min only handles the isolation.

## Config location

Blueprints are stored in:

| Platform | Path |
|----------|------|
| Windows | `%APPDATA%\helo\config\config.toml` |
| Linux | `~/.config/helo/config.toml` |
| macOS | `~/Library/Application Support/helo/config.toml` |

## Compared to helo

[helo](https://github.com/ryha0008-boop/helo-win) is the full version — adds API key management, provider presets, interactive mode, self-update, shell completions, and more. helo-min is the core mechanism only, for users who want full manual control.
