# `nudo.nvim` / Nushell System Management Toolkit

A collection of Nushell scripts providing:

* Persistent toggles
* GPU mode switching
* Autostart management
* USE flag editing for Gentoo
* Package management abstraction (Arch, Gentoo, Winget)
* fzf-powered directory/history/file navigation
* Bluetooth helpers
* Safe file editing with lock system
* wm/autostart integration
* Pywal theme automation

This project turns Nushell into a **system administration control panel** with a single command: `nudo`.

---

## Table of Contents

1. Overview
2. Installation
3. Directory Layout
4. Architecture
5. Environment / Toggle System
6. Package Manager Wrapper
7. Autostart System
8. Gentoo Utilities
9. GPU Mode Switching
10. fzf Utilities
11. Bluetooth Client
12. Window Manager Startup
13. Pywal Integration
14. Logging / Debugging
15. Usage Examples
16. Roadmap
17. License
18. Maintainers & Credits

---

---

# 1. Overview

This project is a structured collection of Nushell scripts intended to provide a unified system management interface across multiple Linux environments (Arch, Gentoo, BSD where applicable). It implements a general command:

```
nudo <command> [args]
```

`nudo` acts similarly to:

* AUR helper (paru/yay)
* Gentoo USE flag and env manager
* Bluetooth quick connector
* GPU mode selector
* Autostart process supervisor

All implemented using plain Nushell without external daemon services.

The philosophy is:

> Configuration should be declarative, persistent, and discoverable.

Everything is stored inside `$nu.data-dir` and `$nu.default-config-dir` with plain text or JSON formats.

---

# 2. Installation

### Prerequisites

* Nushell ≥ 0.93
* Linux recommended (limited Windows support for `pkg_manager`)
* `fzf` recommended
* `fd` recommended
* `bat` recommended
* Optional: `pywal`, `starship`, `bluetoothctl`, `notify-send`

### Clone or copy the files

Recommended tree:

```
~/.config/nushell/
    config.nu
    config.vscode.nu
    functions/
```

Make sure `config.nu` includes:

```nu
const functions_path = ($nu.config-path | path dirname | path join "functions")
use $functions_path *
```

---

# 3. Directory Layout

```
config.nu
config.vscode.nu

functions/
│
├─ autostart.nu
├─ bluecon.nu
├─ editsu.nu
├─ fzf.nu
├─ genuse.nu
├─ gpu-mode.nu
├─ mod.nu
├─ nudo_file.nu
├─ pkg_manager.nu
├─ settings.nu
├─ start_wm.nu
└─ utils.nu
```

### Role Summary

| File             | Purpose                              |
| ---------------- | ------------------------------------ |
| config.nu        | Terminal startup configuration       |
| config.vscode.nu | VSCode terminal configuration        |
| utils.nu         | Core reusable functions              |
| nudo_file.nu     | Main entrypoint (`nudo`)             |
| pkg_manager.nu   | Package abstraction layer            |
| genuse.nu        | Gentoo USE flag controller           |
| gpu-mode.nu      | NVIDIA performance modes             |
| autostart.nu     | Per-user autostart manager           |
| fzf.nu           | Terminal UI and keybinding utilities |
| editsu.nu        | Safe, atomic file editor             |
| settings.nu      | Toggles & environment store          |
| bluecon.nu       | Interactive bluetooth connector      |
| start_wm.nu      | Window Manager autostart loader      |
| mod.nu           | Global re-export file                |

---

# 4. Architecture

The project uses several core design ideas:

### 4.1 Command Dispatch System

Commands are routed by `nudo_file.nu`:

```
nudo install
nudo set
nudo get
nudo remove
nudo connect
```

Each is dispatched to a specific module.

### 4.2 Persistent Data Storage

Data is stored in these locations:

| Purpose         | File                            |
| --------------- | ------------------------------- |
| Toggles         | `$nu.data-dir/toggles`          |
| Autostart       | `$nu.data-dir/astart-repo`      |
| Custom env vars | `$nu.default-config-dir/env.nu` |
| GPU settings    | same toggle file                |

Everything stored is either plain text(for custom env vars) or JSON.

### 4.3 Error Handling System

`utils.nu` implements:

* `dependency_check`
* `detect_os`
* `any_one_of`
* `args_required`
* `debug_print`
* `run`

Every command has defensive checks.

---

# 5. Toggle System

Toggles are stored in:

```
$nu.data-dir/toggles   (JSON format)
```

A toggle is a simple `{ toggle, value }` object.

### Typical toggles

```
colors
wm
wallpath
DEBUG
powersave
balanced
gaming
```

### Commands

```
nudo set toggle colors true
nudo get toggle
nudo remove toggle colors
```

---

# 6. Package Manager Wrapper

Supported systems:

* Arch (paru, yay, pacman)
* Gentoo (emerge)
* Windows (winget)

### Commands

```
nudo install <pkg>
nudo remove <pkg>
nudo update
nudo search <term>
nudo clean
```

### Design

`pkg_manager.nu`:

* Detects package manager automatically
* Privilege elevation (sudo/doas/run0)
* Cross-platform code

---

# 7. Autostart System

Autostart commands are stored in:

```
$nu.data-dir/astart-repo   (JSON list)
```

### Features

* Lock files prevent duplicate process start
* Spawned using background jobs
* Automatically executed on shell launch

### Commands

```
nudo set autostart "fastfetch --config examples/10"
nudo get autostart
nudo remove autostart 0
```

---

# 8. Gentoo Utilities (`genuse.nu`)

Full USE flag management without editing `/etc/portage` by hand.

### Commands

```
nudo set use dev-libs/openssl ssl asm
nudo get use
nudo remove use dev-libs/openssl
```

Also supports:

```
keyword
env
```

---

# 9. GPU Mode Switching (`gpu-mode.nu`)

Requires:

* `nvidia-smi`
* Toggles configured (powersave, balanced, gaming)

### Usage

```
nudo set mode powersave
nudo set mode balanced
nudo set mode gaming
```

Modes apply persistent:

```
nvidia-smi -pm 1
nvidia-smi -lgc X,Y
```

---

# 10. FZF Utilities (`fzf.nu`)

Provides terminal UI bindings:

* Ctrl+h → history
* Ctrl+t → directory telescope
* Ctrl+e → quick edit

### Example

```
fzf-edit ~/projects
fzf-directory /etc
```

---

# 11. Bluetooth (`bluecon.nu`)

Scan, filter, and connect to BT devices.

### Usage

```
nudo connect
nudo connect speaker
```

---

# 12. Window Manager Startup

```
wm
```

* Reads toggle `"wm"`
* Starts or logs WM command
* Respects `$XDG_CURRENT_DESKTOP`

Examples:

```
nudo set toggle wm startx
nudo set toggle wm hyprland
```

---

# 13. Pywal Integration

`config.nu` automatically applies pywal colors when:

* toggle `"colors"` is true
* not running on TTY
* `wal` exists in PATH

Uses:

* setsid to suppress terminal output
* notify-send for progress popup

---

# 14. Debug Logging

Activate debug mode:

```
nudo set toggle DEBUG 1
```

Disable:

```
nudo set toggle DEBUG 0
```

Prints `[DEBUG]` output for all functions that call `debug_print`.

---

# 15. Usage Examples

### Install a package

```
nudo install neovim
```

### Search for a package

```
nudo search firefox
```

### Set Pywal auto mode

```
nudo set toggle colors true
nudo set toggle wallpath ~/Pictures/Wallpapers/current.jpg
```

### Set GPU balanced clocks

```
nudo set toggle balanced "1500,4500"
nudo set mode balanced
```

### Add autostart app

```
nudo set autostart "fastfetch --config examples/10"
```

### List USE flags

```
nudo get use
```

---

# 16. Roadmap

* Systemd user service for autostart
* Logging system in `$XDG_STATE_HOME`
* Pipe-based IPC for `nudo`
* Plug-in installer (`nudo self update`)
* Web UI (local, optional)
* Generate man page

---

# 17. License

MIT License — see `LICENSE`.

---

# 18. Maintainers & Credits

Author: **Aschere Verhen**
Project language: **Nushell**
Compatible systems tested: Arch Linux, Gentoo Linux

> Contributions, issues, and feature requests are welcome.

---

If you want a **second version** of this README optimized for **GitHub flair**, including badges, ASCII banners, and auto-generated TOC links, just ask.
