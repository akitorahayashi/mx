# mx

`mx` is a Rust CLI that unifies two daily workflows:

1. **Snippet command** – type `mx command <snippet>` (alias `mx c`) to stream any markdown snippet under
   `~/.config/mx/commands/` into your clipboard.
2. **Context Orchestration** – type `mx touch <key>` (alias `mx t`) to create context files in your project with clipboard content, and `mx cat <key>` (alias `mx ct`) to view their contents.

## Storage layout

```text
~/.config/mx/
  commands/
    w/wc.md
    sdd/sdd-0-rq.md
    ... (any nested directory structure)
```

- **Snippet lookup** scans `commands/` recursively for `.md` files. Both `mx c wc` and `mx c w/wc` resolve to
  `commands/w/wc.md`.

## Documentation

- [CLI usage](docs/cli-usage.md): Learn how to use the CLI, manage context aliases, and work with dynamic paths.
- [Configuration](docs/configuration.md): Environment variables and other ways to configure `mx`.
- [Development guide](docs/development-guide.md): Instructions for setting up the development environment, running tests, and contributing.
