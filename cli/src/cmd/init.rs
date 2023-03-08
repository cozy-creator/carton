use anyhow::{bail, Ok, Result};
use clap::Parser;
use std::{fs::File, io::Write};

use carton_core::{manifest, path};

use crate::template;

#[derive(Parser)]
/// Initialize Carton.toml in an existing directory
pub struct Init {
    #[clap(long)]
    force: bool,
}

impl Init {
    pub fn execute(self) -> Result<()> {
        let file_path = path::get_current_path()?.join(manifest::CARTON_MANIFEST_FILE_NAME);

        if file_path.is_file() && !self.force {
            bail!("Carton has already been initialized in this directory")
        }

        let mut file = File::create(&file_path)?;
        file.write_all(template::CARTON_MANIFEST_TEMPLATE.as_bytes())?;

        Ok(())
    }
}
