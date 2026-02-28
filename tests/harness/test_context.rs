use assert_cmd::Command;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use tempfile::TempDir;

pub struct TestContext {
    root: TempDir,
    work_dir: PathBuf,
    env_vars: RefCell<HashMap<String, String>>,
}

#[allow(dead_code)]
impl TestContext {
    pub fn new() -> Self {
        let root = TempDir::new().expect("Failed to create temp directory");
        let work_dir = root.path().join("work");
        fs::create_dir_all(&work_dir).expect("Failed to create test work dir");

        let ctx = Self { root, work_dir, env_vars: RefCell::new(HashMap::new()) };
        fs::create_dir_all(ctx.commands_root()).expect("Failed to create commands root");
        ctx
    }

    pub fn home(&self) -> &Path {
        self.root.path()
    }

    pub fn work_dir(&self) -> &Path {
        &self.work_dir
    }

    pub fn commands_root(&self) -> PathBuf {
        self.home().join(".config").join("mx").join("commands")
    }

    pub fn cli(&self) -> Command {
        self.cli_in(self.work_dir())
    }

    pub fn cli_in<P: AsRef<Path>>(&self, dir: P) -> Command {
        let mut cmd = Command::cargo_bin("mx").expect("Failed to locate mx binary");
        cmd.current_dir(dir.as_ref()).env("HOME", self.home());

        for (key, value) in self.env_vars.borrow().iter() {
            cmd.env(key, value);
        }

        cmd
    }

    pub fn write_snippet(&self, relative: &str, contents: &str) -> PathBuf {
        let path = self.commands_root().join(relative);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).expect("Failed to create snippet parent");
        }
        fs::write(&path, contents).expect("Failed to write snippet");
        path
    }

    pub fn clipboard_file(&self, name: &str) -> PathBuf {
        let file = self.work_dir().join(name);
        self.set_env("MX_CLIPBOARD_FILE", file.to_string_lossy());
        file
    }

    pub fn set_env<S: AsRef<str>>(&self, key: &str, value: S) {
        self.env_vars.borrow_mut().insert(key.to_string(), value.as_ref().to_string());
    }
}
