Below is a **GitHub-ready README** version of your project.
Same material, but formatted with badges, flair, automatic TOC, and visually clean sections for public hosting.

---

# `nudo` — Nushell System Management Toolkit

> A complete Nushell automation suite providing GPU mode control, autostart supervision, Gentoo USE management, pywal integration, Bluetooth quick connect, and cross-platform package handling — all from one command: `nudo`.

<p align="left">
  <a href="https://www.nushell.sh/"><img src="https://img.shields.io/badge/Nushell-0.93%2B-blue?logo=nuget" /></a>
  <a href="https://opensource.org/licenses/MIT"><img src="https://img.shields.io/badge/License-MIT-green.svg" /></a>
  <img src="https://img.shields.io/badge/Linux-Supported-success?logo=linux" />
  <img src="https://img.shields.io/badge/Gentoo-Optimized-purple?logo=gentoo" />
  <img src="https://img.shields.io/badge/Arch-Works-lightgrey?logo=arch-linux" />
</p>

---

```
                           _       
   _ __   _   _   ___   __| |  ___ 
  | '__| | | | | / __| / _` | / _ \
  | |    | |_| | \__ \| (_| ||  __/
  |_|     \__,_| |___/ \__,_| \___|
```

The purpose of this toolkit is very simple:

* **One command** — `nudo`
* **Everything system management**

No more remembering which package manager you’re on, how to edit USE flags, what WM to start, or what GPU power level to switch to. Just manage everything consistently using Nushell functions.

---

## ⭐ Features

* **Unified control tool**: `nudo <command>`
* Gentoo USE flag helper (no file editing required)
* Nvidia GPU performance profile manager
* Autostart process scheduler (with lock safety)
* fzf-powered file, directory, and history navigation
* Safe atomic file editing with privileged fallback
* Bluetooth device selection + connection
* Pywal startup automation
* Window manager startup without `.xinitrc`
* Cross-distro package installer abstraction

---

## 📦 Table of Contents

* [Installation](#installation)
* [Project Structure](#project-structure)
* [Usage](#usage)

  * [Package management](#package-management)
  * [Toggles](#toggles)
  * [Autostart](#autostart)
  * [USE flags (Gentoo)](#use-flags-gentoo)
  * [GPU mode](#gpu-mode)
  * [fzf tools](#fzf-tools)
  * [Bluetooth](#bluetooth)
  * [Window manager](#window-manager)
* [Requirements](#requirements)
* [Debugging](#debugging)
* [Roadmap](#roadmap)
* [License](#license)

---

## 🛠 Installation

Clone or copy into your Nushell config directory:

```
~/.config/nushell/
    config.nu
    config.vscode.nu
    functions/
```

Make sure `functions/mod.nu` gets included in your `config.nu`:

```nu
const functions_path = ($nu.config-path | path dirname | path join "functions")
use $functions_path *
```

**Restart Nushell.**
`nudo` should now be available.

---

## 🏗 Project Structure

```
config.nu
config.vscode.nu

functions/
├─ utils.nu
├─ nudo_file.nu
├─ pkg_manager.nu
├─ genuse.nu
├─ gpu-mode.nu
├─ autostart.nu
├─ fzf.nu
├─ bluecon.nu
├─ editsu.nu
├─ settings.nu
├─ start_wm.nu
└─ mod.nu
```

### Major Modules

| Module           | Purpose                                   |
| ---------------- | ----------------------------------------- |
| `nudo_file.nu`   | Command dispatch system                   |
| `pkg_manager.nu` | Pacman/paru/yay/emerge/winget abstraction |
| `genuse.nu`      | Gentoo USE, keyword, env control          |
| `gpu-mode.nu`    | Manage `nvidia-smi` performance levels    |
| `autostart.nu`   | Persistent autostart registry             |
| `fzf.nu`         | Keyboard shortcuts for fzf actions        |
| `bluecon.nu`     | Interactive bluetooth device selection    |
| `editsu.nu`      | Safe file editor with locking             |
| `settings.nu`    | Toggle + environment variable persistence |

---

## 🚀 Usage

All commands start with:

```
nudo <command> [...]
```

### Package Management

#### Install

```
nudo install neovim
```

Auto-detects:

* `paru`
* `yay`
* `pacman`
* `emerge`
* `winget`

#### Update

```
nudo update
```

#### Search

```
nudo search firefox
```

---

### Toggles

Store persistent state values:

```
nudo set toggle colors true
nudo set toggle wallpath ~/Wallpapers/current.jpg
nudo get toggle
nudo remove toggle colors
```

Stored in:

```
$nu.data-dir/toggles
```

---

### Autostart

Add autostart commands:

```
nudo set autostart "fastfetch --config examples/10"
nudo get autostart
nudo remove autostart 0
```

On every start:

* Commands are checked
* Commands run **once** using lock files in `/tmp/nudo/astart`

---

### USE Flags (Gentoo)

#### Add USE flags

```
nudo set use dev-libs/openssl asm ssl
```

#### Remove

```
nudo remove use dev-libs/openssl
```

#### View all USE flags

```
nudo get use
```

Files written to:

* `/etc/portage/package.use`
* `/etc/portage/package.accept_keywords`(wip)
* `/etc/portage/package.env`(wip)

---

### GPU Mode

Switch Nvidia GPU performance profile:

```
nudo set mode powersave
nudo set mode balanced
nudo set mode gaming
```

Modes read from toggles:

```
powersave
balanced
gaming
```

Example toggle:

```
nudo set toggle gaming "1500,4500"
```

---

### fzf Tools

Three keybindings (Ctrl-h, Ctrl-t, Ctrl-e):

* History search
* Directory telescope
* Quick edit

Direct usage:

```
fzf-history(Ctrl-h)
fzf-directory(Ctrl-t)
fzf-edit(Ctrl-e)
```

---

### Bluetooth

Connect to device via interactive fuzzy match:

```
nudo connect
```

Or search term:

```
nudo connect speaker
```

---

### Window Manager

Set default WM:

```
nudo set toggle wm startx
nudo set toggle wm hyprland
```

Launch:

```
wm
```

---

## ✔ Requirements

### Core

* Nushell
* Linux (most modules)
* `sudo` or `doas` or `run0`

### Optional (recommended)

* `fzf`
* `fd`
* `bat`
* `paru` / `yay` / `pacman`
* `bluetoothctl`
* `notify-send` (Pre-installed in most distros, except maybe lfs)
* `pywal`
* `starship`

---

## 🧩 Debugging

Enable verbose logging:

```
nudo set toggle DEBUG 1
```

Disable:

```
nudo set toggle DEBUG 0
```

All debug messages print with:

```
[DEBUG]:
```

---
## 📄 License

MIT License — see `LICENSE`.
