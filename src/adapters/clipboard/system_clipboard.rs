use crate::domain::error::AppError;
use crate::ports::Clipboard;
use std::env;
use std::io::Write;
use std::process::{Command, Stdio};

#[derive(Debug, Clone)]
struct ClipboardCommand {
    program: String,
    args: Vec<String>,
}

impl ClipboardCommand {
    fn new(program: impl Into<String>) -> Self {
        Self { program: program.into(), args: Vec::new() }
    }

    fn from_string(cmd_str: &str) -> Option<Self> {
        let mut parts = cmd_str.split_whitespace();
        let program = parts.next()?;
        let args = parts.map(|part| part.to_string()).collect();
        Some(Self { program: program.to_string(), args })
    }

    fn with_args<I, S>(mut self, args: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.args = args.into_iter().map(Into::into).collect();
        self
    }
}

pub struct SystemClipboard {
    copy_command: ClipboardCommand,
    paste_command: ClipboardCommand,
}

impl SystemClipboard {
    pub fn detect() -> Result<Self, AppError> {
        let copy_var = env::var("MX_COPY_CMD");
        let paste_var = env::var("MX_PASTE_CMD");

        if let (Ok(copy_str), Ok(paste_str)) = (copy_var.as_ref(), paste_var.as_ref()) {
            let copy_command = ClipboardCommand::from_string(copy_str)
                .ok_or_else(|| AppError::clipboard_error("MX_COPY_CMD is empty"))?;
            let paste_command = ClipboardCommand::from_string(paste_str)
                .ok_or_else(|| AppError::clipboard_error("MX_PASTE_CMD is empty"))?;
            return Ok(Self { copy_command, paste_command });
        }

        if copy_var.is_ok() || paste_var.is_ok() {
            return Err(AppError::clipboard_error(
                "Both MX_COPY_CMD and MX_PASTE_CMD must be set if either is provided",
            ));
        }

        if let Ok(custom) = env::var("MX_CLIPBOARD_CMD") {
            let command = ClipboardCommand::from_string(&custom)
                .ok_or_else(|| AppError::clipboard_error("MX_CLIPBOARD_CMD is empty"))?;
            return Ok(Self { copy_command: command.clone(), paste_command: command });
        }

        match env::consts::OS {
            "macos" => Ok(Self {
                copy_command: ClipboardCommand::new("pbcopy"),
                paste_command: ClipboardCommand::new("pbpaste"),
            }),
            "linux" => Self::detect_linux(),
            "windows" => Ok(Self {
                copy_command: ClipboardCommand::new("clip"),
                paste_command: ClipboardCommand::new("powershell").with_args([
                    "-noprofile",
                    "-command",
                    "Get-Clipboard",
                ]),
            }),
            other => Err(AppError::clipboard_error(format!(
                "Unsupported platform '{other}' for clipboard operations"
            ))),
        }
    }

    fn detect_linux() -> Result<Self, AppError> {
        if Self::command_available("wl-copy", ["--version"]) {
            return Ok(Self {
                copy_command: ClipboardCommand::new("wl-copy"),
                paste_command: ClipboardCommand::new("wl-paste"),
            });
        }

        if Self::command_available("xclip", ["-version"]) {
            return Ok(Self {
                copy_command: ClipboardCommand::new("xclip").with_args(["-selection", "clipboard"]),
                paste_command: ClipboardCommand::new("xclip").with_args([
                    "-selection",
                    "clipboard",
                    "-o",
                ]),
            });
        }

        Err(AppError::clipboard_error(
            "No supported clipboard command found. Install wl-copy or xclip, or set MX_CLIPBOARD_FILE.",
        ))
    }

    fn command_available<'a, I>(program: &str, args: I) -> bool
    where
        I: IntoIterator<Item = &'a str>,
    {
        Command::new(program)
            .args(args)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .is_ok()
    }
}

impl Clipboard for SystemClipboard {
    fn copy(&self, text: &str) -> Result<(), AppError> {
        let mut command = Command::new(&self.copy_command.program);
        command
            .args(&self.copy_command.args)
            .stdin(Stdio::piped())
            .stdout(Stdio::null())
            .stderr(Stdio::null());

        let mut child = command.spawn().map_err(|err| {
            AppError::clipboard_error(format!(
                "Failed to run clipboard command '{}': {err}",
                self.copy_command.program
            ))
        })?;

        if let Some(mut stdin) = child.stdin.take() {
            stdin.write_all(text.as_bytes()).map_err(|err| {
                AppError::clipboard_error(format!(
                    "Failed to send data to clipboard command: {err}"
                ))
            })?;
        }

        let status = child
            .wait()
            .map_err(|err| AppError::clipboard_error(format!("Clipboard command failed: {err}")))?;

        if status.success() {
            Ok(())
        } else {
            Err(AppError::clipboard_error(format!("Clipboard command exited with status {status}")))
        }
    }

    fn paste(&self) -> Result<String, AppError> {
        let output = Command::new(&self.paste_command.program)
            .args(&self.paste_command.args)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .map_err(|err| {
                AppError::clipboard_error(format!(
                    "Failed to run paste command '{}': {err}",
                    self.paste_command.program
                ))
            })?;

        if output.status.success() {
            String::from_utf8(output.stdout).map_err(|err| {
                AppError::clipboard_error(format!("Clipboard content is not valid UTF-8: {err}"))
            })
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            Err(AppError::clipboard_error(format!(
                "Paste command exited with status {}. Stderr: {}",
                output.status,
                stderr.trim()
            )))
        }
    }
}
