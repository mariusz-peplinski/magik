# Install Local `magic` CLI

This repo now builds a local CLI binary named `magic`.

## Quick install

From the repo root:

```bash
./scripts/install-magic-local.sh --build
```

That will:

- build with `./build-fast.sh` (if needed)
- install `magic` to `~/.local/bin/magic`

Verify:

```bash
magic --version
```

## Install modes

Copy mode (default):

```bash
./scripts/install-magic-local.sh --build --copy
```

Symlink mode (auto-picks up new local repo builds):

```bash
./scripts/install-magic-local.sh --build --link
```

Custom install directory:

```bash
./scripts/install-magic-local.sh --build --install-dir "$HOME/bin"
```

## PATH setup

If `magic` is not found after install, add your install dir to `PATH`.

For the default location:

```bash
export PATH="$HOME/.local/bin:$PATH"
```

Put that in your shell profile (`~/.bashrc`, `~/.zshrc`, etc.), then open a new shell.

## Updating after pulling changes

```bash
./scripts/install-magic-local.sh --build
```

