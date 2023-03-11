use std::{env, fs::File, io::Write};

use anyhow::{Ok, Result};
use carton_core::manifest;
use clap::Parser;
use sui_move::new::New as NewPackage;

use crate::template;

#[derive(Parser)]
/// Create a move package and initialize Carton.toml
pub struct Create {
    #[clap(flatten)]
    pub new: NewPackage,
}

impl Create {
    pub fn execute(self) -> Result<()> {
        let current_path = env::current_dir()?;
        let package_path = current_path.join(self.new.new.name.clone());

        self.new.execute(Some(package_path.to_path_buf()))?;

        if !current_path
            .join(manifest::CARTON_MANIFEST_FILE_NAME)
            .is_file()
        {
            let file_path = package_path.join(manifest::CARTON_MANIFEST_FILE_NAME);

            let mut file = File::create(&file_path)?;
            file.write_all(template::CARTON_MANIFEST_TEMPLATE.as_bytes())?;
        }

        Ok(())
    }
}
