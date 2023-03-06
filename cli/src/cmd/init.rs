use anyhow::{bail, Ok, Result};
use clap::Parser;
use std::{fs, io::Write, path::Path};

use carton_core::{manifest, path};

use crate::template;

#[derive(Parser)]
/// Initialize Carton.toml in an existing directory
pub struct Init {}

impl Init {
    pub fn execute(self) -> Result<()> {
        let path = path::get_current_path()?;
        Self::init_carton(&path, true)?;

        Ok(())
    }

    pub fn init_carton(path: &Path, force: bool) -> Result<()> {
        let path = path.join(manifest::CARTON_MANIFEST_FILE_NAME);

        if path.is_file() && !force {
            bail!("Carton has already been initialized in this directory")
        }

        let mut file = fs::File::create(&path)?;
        file.write_all(template::CARTON_MANIFEST_TEMPLATE.as_bytes())?;

        Ok(())
    }
}
