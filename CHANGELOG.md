# Changelog

All notable changes to helo-minimal are documented here.
Format: [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).

---

## [Unreleased]

## [0.1.3] — 2026-04-17

### Changed
- Binary renamed from `helo-min` to `helomin` — PowerShell treats hyphens as operators

## [0.1.2] — 2026-04-17

### Changed
- Binary renamed from `helo` to `helo-min` to avoid conflicts with helo full version

## [0.1.1] — 2026-04-17

### Fixed
- opencode: isolation now uses `XDG_DATA_HOME` (redirects database) + `OPENCODE_CONFIG` (config file). `OPENCODE_CONFIG` alone only redirected config, not session data.
- pi on Windows: launch via `cmd /c` to resolve `.cmd` shim

## [0.1.0] — 2026-04-16

Initial release.

### Added
- `helo add <name> --runtime <runtime>` — register a blueprint
- `helo run <name>` — launch a blueprint in the current directory with runtime isolation
