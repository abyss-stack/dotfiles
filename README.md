# Abyss Dotfiles

A flexible dotfiles manager.

## Features
* **Config-driven**: Define your dotfiles layout in a single JSON config.
* **Multiple strategies**: To create symlinks or copy the whole package.
* **Package-oriented**: Organize dotfiles as reusable packages.
* **SSOT**: The provided config acts as the Single Source of Truth (SSOT).
* **JSON Output**: Output uses a strict JSON contract and is easy to parse.

<!-- # INSTALLATION_START -->
## Installation

Install the pre-compiled static binary:

```sh
sudo curl -L -o /usr/local/bin/abyss-dotfiles https://github.com/abyss-stack/dotfiles/releases/download/v2026.08.02/abyss-dotfiles \
  && sudo chmod +x /usr/local/bin/abyss-dotfiles
```

Verify the installation:
```sh
abyss-dotfiles --version
```
<!-- # INSTALLATION_END -->
