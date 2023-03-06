use std::{env, path::PathBuf};

use anyhow::{bail, Ok, Result};

use crate::manifest;

pub fn get_current_path() -> Result<PathBuf> {
    Ok(env::current_dir()?)
}

pub fn get_root_path(path: Option<PathBuf>) -> Result<PathBuf> {
    let path = match path {
        Some(path) => path,
        _ => get_current_path()?,
    };

    let mut current = path.to_path_buf();

    loop {
        let manifest_path = current.join(manifest::CARTON_MANIFEST_FILE_NAME);

        if manifest_path.is_file() {
            break Ok(current.canonicalize()?);
        }

        if !current.pop() {
            bail!(
                "Unable to find Carton manifest in {} or it's parent",
                path.to_string_lossy()
            )
        }
    }
}
