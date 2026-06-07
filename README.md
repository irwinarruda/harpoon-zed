# Harpoon Zed

A small command-line helper for keeping a per-worktree harpoon list in Zed.

The tool stores one list per Zed worktree, lets you add the currently open file,
opens the list for editing, and jumps to numbered entries through Zed's CLI.

## Platform Support

This project is intended to work on macOS, Linux, and Windows, but it depends on
two commands being available from the environment where Zed runs tasks:

- `harpoon`, installed from this repo.
- `zed` or `zed.exe`, installed by Zed, unless `ZED_CLI_PATH` points to the Zed
  CLI executable.

Config and harpoon files are stored under the normal Zed config directory:

| Platform | Zed config directory |
| --- | --- |
| macOS | `~/.config/zed` |
| Linux | `$XDG_CONFIG_HOME/zed`, or `~/.config/zed` when `XDG_CONFIG_HOME` is unset |
| Windows | `%APPDATA%\Zed` |

## Install

Install Rust first if `cargo` is not available:

- macOS/Linux: <https://rustup.rs>
- Windows: <https://rustup.rs> or `winget install Rustlang.Rustup`

Then install this repo:

```sh
cargo install --path . --force
```

That installs the `harpoon` executable into Cargo's bin directory:

- macOS/Linux: `~/.cargo/bin`
- Windows: `%USERPROFILE%\.cargo\bin`

Make sure that directory is on `PATH`. You can check with:

```sh
harpoon
```

It should print a usage error like:

```text
usage: harpoon <open|add|go> [n]
```

You can also build and install with the Makefile on macOS/Linux:

```sh
make install
```

By default, this installs `harpoon` to `~/.bin`.

## Zed Configuration

Add one of these task sets to your global Zed `tasks.json`.

Open it from Zed with `zed: open tasks`, or edit the file directly:

- macOS/Linux: `~/.config/zed/tasks.json`
- Windows: `%APPDATA%\Zed\tasks.json`

### Normal Tasks

These tasks use Zed's normal task behavior. Zed may show the task terminal while
the command runs.

```json
[
  {
    "label": "harpoon: open",
    "command": "harpoon",
    "args": ["open"],
    "use_new_terminal": false,
    "allow_concurrent_runs": false
  },
  {
    "label": "harpoon: add",
    "command": "harpoon",
    "args": ["add"],
    "use_new_terminal": false,
    "allow_concurrent_runs": false
  },
  {
    "label": "harpoon: go 1",
    "command": "harpoon",
    "args": ["go", "1"],
    "use_new_terminal": false,
    "allow_concurrent_runs": false
  },
  {
    "label": "harpoon: go 2",
    "command": "harpoon",
    "args": ["go", "2"],
    "use_new_terminal": false,
    "allow_concurrent_runs": false
  },
  {
    "label": "harpoon: go 3",
    "command": "harpoon",
    "args": ["go", "3"],
    "use_new_terminal": false,
    "allow_concurrent_runs": false
  },
  {
    "label": "harpoon: go 4",
    "command": "harpoon",
    "args": ["go", "4"],
    "use_new_terminal": false,
    "allow_concurrent_runs": false
  }
]
```

### Silent Tasks

These tasks keep the task terminal out of the way during successful runs. This
is useful on Windows, where Zed may otherwise open a bottom task terminal for
each keybinding command.

```json
[
  {
    "label": "harpoon: open",
    "command": "harpoon",
    "args": ["open"],
    "use_new_terminal": false,
    "allow_concurrent_runs": false,
    "reveal": "never",
    "hide": "on_success",
    "show_summary": false,
    "show_command": false
  },
  {
    "label": "harpoon: add",
    "command": "harpoon",
    "args": ["add"],
    "use_new_terminal": false,
    "allow_concurrent_runs": false,
    "reveal": "never",
    "hide": "on_success",
    "show_summary": false,
    "show_command": false
  },
  {
    "label": "harpoon: go 1",
    "command": "harpoon",
    "args": ["go", "1"],
    "use_new_terminal": false,
    "allow_concurrent_runs": false,
    "reveal": "never",
    "hide": "on_success",
    "show_summary": false,
    "show_command": false
  },
  {
    "label": "harpoon: go 2",
    "command": "harpoon",
    "args": ["go", "2"],
    "use_new_terminal": false,
    "allow_concurrent_runs": false,
    "reveal": "never",
    "hide": "on_success",
    "show_summary": false,
    "show_command": false
  },
  {
    "label": "harpoon: go 3",
    "command": "harpoon",
    "args": ["go", "3"],
    "use_new_terminal": false,
    "allow_concurrent_runs": false,
    "reveal": "never",
    "hide": "on_success",
    "show_summary": false,
    "show_command": false
  },
  {
    "label": "harpoon: go 4",
    "command": "harpoon",
    "args": ["go", "4"],
    "use_new_terminal": false,
    "allow_concurrent_runs": false,
    "reveal": "never",
    "hide": "on_success",
    "show_summary": false,
    "show_command": false
  }
]
```

Use `"hide": "always"` instead of `"hide": "on_success"` if you want task tabs
hidden even when a command fails.

### Zed CLI Path

If `zed` or `zed.exe` is not on `PATH`, add `ZED_CLI_PATH` to each task.

Windows:

```json
"env": {
  "ZED_CLI_PATH": "C:\\Users\\you\\AppData\\Local\\Programs\\Zed\\bin\\Zed.exe"
}
```

macOS:

Zed installed in the default location works without extra configuration. For Zed
Preview or another app bundle, set:

```json
"env": {
  "ZED_CLI_PATH": "/Applications/Zed Preview.app/Contents/MacOS/cli"
}
```

## Keybindings

Add bindings to your global Zed `keymap.json`.

Open it from Zed with `zed: open keymap`, or edit the file directly:

- macOS/Linux: `~/.config/zed/keymap.json`
- Windows: `%APPDATA%\Zed\keymap.json`

```json
[
  {
    "context": "Workspace",
    "bindings": {
      "ctrl-alt-h": ["task::Spawn", { "task_name": "harpoon: open" }],
      "ctrl-alt-a": ["task::Spawn", { "task_name": "harpoon: add" }],
      "ctrl-alt-1": ["task::Spawn", { "task_name": "harpoon: go 1" }],
      "ctrl-alt-2": ["task::Spawn", { "task_name": "harpoon: go 2" }],
      "ctrl-alt-3": ["task::Spawn", { "task_name": "harpoon: go 3" }],
      "ctrl-alt-4": ["task::Spawn", { "task_name": "harpoon: go 4" }]
    }
  }
]
```

If you already have entries in `keymap.json`, merge the object above into the
existing top-level array instead of replacing the whole file.

## Usage

From Zed:

- `harpoon: add` adds the currently open file to the current worktree's list.
- `harpoon: open` opens the current worktree's harpoon list.
- `harpoon: go 1` opens the first saved file, `harpoon: go 2` opens the second,
  and so on.

The harpoon list is a plain text file. Reorder lines to change slot numbers.
Remove lines to delete entries.

From a shell, the same commands are available:

```sh
harpoon open
harpoon add
harpoon go 1
```

`harpoon add`, `harpoon open`, and `harpoon go` require `ZED_WORKTREE_ROOT`.
`harpoon add` also requires `ZED_RELATIVE_FILE`. Zed provides these variables
automatically when the commands run as tasks.

## Troubleshooting

If Zed says the task command was not found, make sure Cargo's bin directory, or
the Makefile install prefix, is on `PATH`.

If `harpoon go` fails with `failed to run Zed CLI`, make sure `zed` or `zed.exe`
is on `PATH`, or set `ZED_CLI_PATH` in the task environment.

If `harpoon add` says `ZED_RELATIVE_FILE is not set`, run it from inside Zed as a
task while a file is open.

If `harpoon go 1` says the slot is empty, add at least one file first or edit the
list opened by `harpoon open`.

## Development

```sh
cargo check --all-targets --all-features
cargo test --all-targets --all-features
cargo clippy --all-targets --all-features -- -D warnings
cargo fmt --all -- --check
```

The Makefile wraps the same commands:

```sh
make check
make test
make lint
make fmt-check
```

## Security Notes

The tool validates stored harpoon entries before writing or opening them.
Entries with newlines, absolute paths, parent-directory traversal, Windows drive
prefixes, or UNC-style paths are rejected.
