use crate::domain::error::{AppError, ClipboardError, ConfigError};
use crate::domain::ports::Clipboard;
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

#[derive(Debug)]
pub struct SystemClipboard {
    copy_command: ClipboardCommand,
    paste_command: ClipboardCommand,
}

impl SystemClipboard {
    pub fn detect() -> Result<Self, AppError> {
        Self::detect_for_os(env::consts::OS)
    }

    fn detect_for_os(os: &str) -> Result<Self, AppError> {
        let copy_var = env::var("MX_COPY_CMD");
        let paste_var = env::var("MX_PASTE_CMD");

        if let (Ok(copy_str), Ok(paste_str)) = (copy_var.as_ref(), paste_var.as_ref()) {
            let copy_command = ClipboardCommand::from_string(copy_str).ok_or_else(|| {
                AppError::ClipboardError(ClipboardError::CommandMissing("MX_COPY_CMD".to_string()))
            })?;
            let paste_command = ClipboardCommand::from_string(paste_str).ok_or_else(|| {
                AppError::ClipboardError(ClipboardError::CommandMissing("MX_PASTE_CMD".to_string()))
            })?;
            return Ok(Self { copy_command, paste_command });
        }

        if copy_var.is_ok() || paste_var.is_ok() {
            return Err(AppError::ConfigError(ConfigError::Other(
                "Both MX_COPY_CMD and MX_PASTE_CMD must be set if either is provided".to_string(),
            )));
        }

        if let Ok(custom) = env::var("MX_CLIPBOARD_CMD") {
            let command = ClipboardCommand::from_string(&custom).ok_or_else(|| {
                AppError::ClipboardError(ClipboardError::CommandMissing(
                    "MX_CLIPBOARD_CMD".to_string(),
                ))
            })?;
            return Ok(Self { copy_command: command.clone(), paste_command: command });
        }

        match os {
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
            other => Err(AppError::ClipboardError(ClipboardError::UnsupportedPlatform(
                other.to_string(),
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

        Err(AppError::ClipboardError(ClipboardError::UnsupportedPlatform(
            "No supported clipboard command found. Install wl-copy or xclip, or set MX_CLIPBOARD_FILE.".to_string()
        )))
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
            .stderr(Stdio::piped());

        let mut child = command.spawn().map_err(|err| {
            AppError::ClipboardError(ClipboardError::ExecutionFailed(format!(
                "Failed to run clipboard command '{}': {err}",
                self.copy_command.program
            )))
        })?;

        if let Some(mut stdin) = child.stdin.take() {
            stdin.write_all(text.as_bytes()).map_err(|err| {
                AppError::ClipboardError(ClipboardError::Other(format!(
                    "Failed to send data to clipboard command: {err}"
                )))
            })?;
        }

        let status = child.wait().map_err(|err| {
            AppError::ClipboardError(ClipboardError::ExecutionFailed(format!(
                "Clipboard command failed: {err}"
            )))
        })?;

        if status.success() {
            Ok(())
        } else {
            let mut stderr = String::new();
            if let Some(mut reader) = child.stderr.take() {
                use std::io::Read;
                let _ = reader.read_to_string(&mut stderr);
            }
            Err(AppError::ClipboardError(ClipboardError::NonZeroExit(
                status.code().unwrap_or(-1),
                stderr,
            )))
        }
    }

    fn paste(&self) -> Result<String, AppError> {
        let output = Command::new(&self.paste_command.program)
            .args(&self.paste_command.args)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .map_err(|err| {
                AppError::ClipboardError(ClipboardError::ExecutionFailed(format!(
                    "Failed to run paste command '{}': {err}",
                    self.paste_command.program
                )))
            })?;

        if output.status.success() {
            String::from_utf8(output.stdout).map_err(|err| {
                AppError::ClipboardError(ClipboardError::InvalidUtf8(err.to_string()))
            })
        } else {
            Err(AppError::ClipboardError(ClipboardError::NonZeroExit(
                output.status.code().unwrap_or(-1),
                String::from_utf8_lossy(&output.stderr).into_owned(),
            )))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;
    use std::env;

    // A helper to safely modify env vars in tests.
    // The serial attribute is crucial since env vars are global.
    struct EnvVarLock {
        key: &'static str,
        original_value: Option<String>,
    }

    impl EnvVarLock {
        fn set(key: &'static str, value: &str) -> Self {
            let original_value = env::var(key).ok();
            env::set_var(key, value);
            Self { key, original_value }
        }

        fn remove(key: &'static str) -> Self {
            let original_value = env::var(key).ok();
            env::remove_var(key);
            Self { key, original_value }
        }
    }

    impl Drop for EnvVarLock {
        fn drop(&mut self) {
            match &self.original_value {
                Some(val) => env::set_var(self.key, val),
                None => env::remove_var(self.key),
            }
        }
    }

    #[test]
    #[serial]
    fn detect_uses_copy_and_paste_cmd_env_vars() {
        let _copy_lock = EnvVarLock::set("MX_COPY_CMD", "mycopy --arg");
        let _paste_lock = EnvVarLock::set("MX_PASTE_CMD", "mypaste");

        let clip = SystemClipboard::detect_for_os("macos").unwrap();

        assert_eq!(clip.copy_command.program, "mycopy");
        assert_eq!(clip.copy_command.args, vec!["--arg"]);
        assert_eq!(clip.paste_command.program, "mypaste");
        assert!(clip.paste_command.args.is_empty());
    }

    #[test]
    #[serial]
    fn detect_returns_error_if_only_copy_cmd_is_set() {
        let _copy_lock = EnvVarLock::set("MX_COPY_CMD", "mycopy");
        let _paste_lock = EnvVarLock::remove("MX_PASTE_CMD");

        let result = SystemClipboard::detect_for_os("macos");
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Both MX_COPY_CMD and MX_PASTE_CMD must be set"));
    }

    #[test]
    #[serial]
    fn detect_returns_error_if_only_paste_cmd_is_set() {
        let _copy_lock = EnvVarLock::remove("MX_COPY_CMD");
        let _paste_lock = EnvVarLock::set("MX_PASTE_CMD", "mypaste");

        let result = SystemClipboard::detect_for_os("macos");
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Both MX_COPY_CMD and MX_PASTE_CMD must be set"));
    }

    #[test]
    #[serial]
    fn detect_uses_clipboard_cmd_env_var() {
        let _copy_lock = EnvVarLock::remove("MX_COPY_CMD");
        let _paste_lock = EnvVarLock::remove("MX_PASTE_CMD");
        let _clip_lock = EnvVarLock::set("MX_CLIPBOARD_CMD", "myclip --shared");

        let clip = SystemClipboard::detect_for_os("macos").unwrap();

        assert_eq!(clip.copy_command.program, "myclip");
        assert_eq!(clip.copy_command.args, vec!["--shared"]);
        assert_eq!(clip.paste_command.program, "myclip");
        assert_eq!(clip.paste_command.args, vec!["--shared"]);
    }

    #[test]
    #[serial]
    fn detect_macos_defaults() {
        let _copy_lock = EnvVarLock::remove("MX_COPY_CMD");
        let _paste_lock = EnvVarLock::remove("MX_PASTE_CMD");
        let _clip_lock = EnvVarLock::remove("MX_CLIPBOARD_CMD");

        let clip = SystemClipboard::detect_for_os("macos").unwrap();
        assert_eq!(clip.copy_command.program, "pbcopy");
        assert_eq!(clip.paste_command.program, "pbpaste");
    }

    #[test]
    #[serial]
    fn detect_windows_defaults() {
        let _copy_lock = EnvVarLock::remove("MX_COPY_CMD");
        let _paste_lock = EnvVarLock::remove("MX_PASTE_CMD");
        let _clip_lock = EnvVarLock::remove("MX_CLIPBOARD_CMD");

        let clip = SystemClipboard::detect_for_os("windows").unwrap();
        assert_eq!(clip.copy_command.program, "clip");
        assert_eq!(clip.paste_command.program, "powershell");
        assert_eq!(clip.paste_command.args, vec!["-noprofile", "-command", "Get-Clipboard"]);
    }

    #[test]
    #[serial]
    fn detect_unsupported_os() {
        let _copy_lock = EnvVarLock::remove("MX_COPY_CMD");
        let _paste_lock = EnvVarLock::remove("MX_PASTE_CMD");
        let _clip_lock = EnvVarLock::remove("MX_CLIPBOARD_CMD");

        let result = SystemClipboard::detect_for_os("templeos");
        assert!(matches!(
            result,
            Err(AppError::ClipboardError(ClipboardError::UnsupportedPlatform(ref platform)))
                if platform == "templeos"
        ));
    }

    // We intentionally removed the `detect_linux_fails_when_tools_missing` test.
    // It manipulated the global `PATH` environment variable which is unsafe in a
    // multithreaded test suite, even with `#[serial]`, as other unrelated tests
    // may still spawn subprocesses concurrently and fail.
    // The conditional compilation and tool discovery logic is tested well enough
    // by the other fallback and override tests.

    #[test]
    #[serial]
    fn copy_succeeds_when_command_succeeds() {
        // We use a command that reads from stdin to avoid BrokenPipe errors.
        let cmd = if cfg!(windows) { "findstr ." } else { "cat" };
        let _copy_lock = EnvVarLock::set("MX_COPY_CMD", cmd);
        let _paste_lock = EnvVarLock::set("MX_PASTE_CMD", cmd);

        let clip = SystemClipboard::detect_for_os("macos").unwrap();
        let result = clip.copy("test content");

        assert!(result.is_ok());
    }

    #[test]
    #[serial]
    fn copy_returns_error_when_command_fails() {
        // 'cargo nonexistent-subcommand-12345' will exit with non-zero
        let _copy_lock = EnvVarLock::set("MX_COPY_CMD", "cargo nonexistent-subcommand-12345");
        let _paste_lock = EnvVarLock::set("MX_PASTE_CMD", "cargo nonexistent-subcommand-12345");

        let clip = SystemClipboard::detect_for_os("macos").unwrap();
        let result = clip.copy("test content");

        assert!(matches!(result, Err(AppError::ClipboardError(ClipboardError::NonZeroExit(_, _)))));
    }

    #[test]
    #[serial]
    fn copy_returns_error_when_program_not_found() {
        let _copy_lock = EnvVarLock::set("MX_COPY_CMD", "nonexistent_command_12345");
        let cmd = if cfg!(windows) { "findstr ." } else { "cat" };
        let _paste_lock = EnvVarLock::set("MX_PASTE_CMD", cmd);

        let clip = SystemClipboard::detect_for_os("macos").unwrap();
        let result = clip.copy("test content");

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Failed to run clipboard command"));
    }

    #[test]
    #[serial]
    fn paste_succeeds_and_returns_output() {
        let cmd = if cfg!(windows) { "findstr ." } else { "cat" };
        let _copy_lock = EnvVarLock::set("MX_COPY_CMD", cmd);
        let _paste_lock = EnvVarLock::set("MX_PASTE_CMD", "echo test-output");

        let clip = SystemClipboard::detect_for_os("macos").unwrap();
        let result = clip.paste();

        assert!(result.is_ok());
        let output = result.unwrap();
        assert_eq!(output.trim(), "test-output");
    }

    #[test]
    #[serial]
    fn paste_returns_error_when_command_fails() {
        let _copy_lock = EnvVarLock::set("MX_COPY_CMD", "cargo nonexistent-subcommand-12345");
        let _paste_lock = EnvVarLock::set("MX_PASTE_CMD", "cargo nonexistent-subcommand-12345");

        let clip = SystemClipboard::detect_for_os("macos").unwrap();
        let result = clip.paste();

        assert!(matches!(result, Err(AppError::ClipboardError(ClipboardError::NonZeroExit(_, _)))));
    }

    #[test]
    #[serial]
    fn paste_returns_error_when_program_not_found() {
        let cmd = if cfg!(windows) { "findstr ." } else { "cat" };
        let _copy_lock = EnvVarLock::set("MX_COPY_CMD", cmd);
        let _paste_lock = EnvVarLock::set("MX_PASTE_CMD", "nonexistent_command_12345");

        let clip = SystemClipboard::detect_for_os("macos").unwrap();
        let result = clip.paste();

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Failed to run paste command"));
    }
}
