use std::fs::{create_dir_all, File};
use std::io::Result;
use std::path::{Path, PathBuf};
use std::{env, fs, io};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct RuntimeLock {
    pub pid: Option<u32>,
}

impl RuntimeLock {
    pub fn empty() -> Self {
        Self { pid: None }
    }

    pub fn default_path() -> PathBuf {
        Self::get_dura_cache_home().join("runtime.db")
    }

    /// Location of all config & database files. By default this is ~/.cache/dura but can be
    /// overridden by setting DURA_CACHE_HOME environment variable.
    fn get_dura_cache_home() -> PathBuf {
        // The environment variable lets us run tests independently, but I'm sure someone will come
        // up with another reason to use it.
        if let Ok(env_var) = env::var("DURA_CACHE_HOME") {
            if !env_var.is_empty() {
                return env_var.into();
            }
        }

        dirs::cache_dir()
            .expect("Could not find your cache directory. The default is ~/.cache/dura but it can also \
                be controlled by setting the DURA_CACHE_HOME environment variable.")
            .join("dura")
    }

    /// Load Config from default path
    pub fn load() -> Self {
        Self::load_file(Self::default_path().as_path()).unwrap_or_else(|_| Self::empty())
    }

    pub fn load_file(path: &Path) -> Result<Self> {
        let reader = io::BufReader::new(File::open(path)?);
        let res = serde_json::from_reader(reader)?;
        Ok(res)
    }

    /// Save config to disk in ~/.cache/dura/runtime.db
    pub fn save(&self) {
        self.save_to_path(Self::default_path().as_path())
    }

    pub fn create_dir(path: &Path) {
        if let Some(dir) = path.parent() {
            create_dir_all(dir)
                .expect(format!("Failed to create directory at `{}`", dir.display()).as_str())
        }
    }

    /// Attempts to create parent dirs, serialize `self` as JSON and write to disk.
    pub fn save_to_path(&self, path: &Path) {
        Self::create_dir(path);

        let json = serde_json::to_string(self).unwrap();
        fs::write(path, json).unwrap()
    }
}
