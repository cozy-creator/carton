use std::{
    env,
    path::{Path, PathBuf},
};

use anyhow::{bail, Ok, Result};
use glob::glob;

use crate::manifest;

pub fn expand_glob(base_path: &Path, pattern: &str) -> Result<Vec<PathBuf>> {
    let pattern_path = base_path.join(pattern).display().to_string();
    let paths = glob(&pattern_path)?
        .map(|e| e.unwrap())
        .collect::<Vec<PathBuf>>();

    Ok(paths)
}

pub fn is_glob_pattern(pattern: &str) -> bool {
    pattern.contains(&['*', '?', '[', ']'][..])
}

pub fn get_root_path() -> Result<PathBuf> {
    let mut current = env::current_dir()?;

    loop {
        let manifest_path = current.join(manifest::CARTON_MANIFEST_FILE_NAME);

        if manifest_path.is_file() {
            break Ok(current.canonicalize()?);
        }

        if !current.pop() {
            bail!(
                "Unable to find Carton manifest in {} or it's parent",
                env::current_dir()?.to_string_lossy()
            )
        }
    }
}
