# Configuration

## Environment overrides

| Variable            | Purpose                                                                                 |
|---------------------|-----------------------------------------------------------------------------------------|
| `MX_COMMANDS_ROOT`  | Override the snippet commands directory (defaults to `~/.config/mx/commands`; legacy `<root>/commands` layouts are also accepted). |
| `MX_CLIPBOARD_FILE` | Use a file for clipboard operations (both read and write) instead of system clipboard.  |
| `MX_CLIPBOARD_CMD`  | Provide a custom clipboard command if the auto-detected one is unavailable.             |
