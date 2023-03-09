use anyhow::{bail, Ok, Result};
use clap::Parser;
use std::{env, fs::File, io::Write};

use carton_core::manifest;

use crate::template;

#[derive(Parser)]
/// Initialize Carton.toml in an existing directory
pub struct Init {
    #[clap(long)]
    force: bool,
}

impl Init {
    pub fn execute(self) -> Result<()> {
        let manifest_path = env::current_dir()?.join(manifest::CARTON_MANIFEST_FILE_NAME);

        if manifest_path.is_file() && !self.force {
            bail!("Carton has already been initialized in this directory")
        }

        let mut file = File::create(&manifest_path)?;
        file.write_all(template::CARTON_MANIFEST_TEMPLATE.as_bytes())?;

        Ok(())
    }
}
