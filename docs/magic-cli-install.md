# Install Local `magik` CLI

This repo now builds a local CLI binary named `magik`.

## Quick install

From the repo root:

```bash
./scripts/install-magik-local.sh --build
```

That will:

- build with `./build-fast.sh` (if needed)
- install `magik` to `~/.local/bin/magik`

Verify:

```bash
magik --version
```

## Install modes

Copy mode (default):

```bash
./scripts/install-magik-local.sh --build --copy
```

Symlink mode (auto-picks up new local repo builds):

```bash
./scripts/install-magik-local.sh --build --link
```

Custom install directory:

```bash
./scripts/install-magik-local.sh --build --install-dir "$HOME/bin"
```

## PATH setup

If `magik` is not found after install, add your install dir to `PATH`.

For the default location:

```bash
export PATH="$HOME/.local/bin:$PATH"
```

Put that in your shell profile (`~/.bashrc`, `~/.zshrc`, etc.), then open a new shell.

## Updating after pulling changes

```bash
./scripts/install-magik-local.sh --build
```
