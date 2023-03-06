use anyhow::{Ok, Result};
use clap::Parser;
use std::env;
use sui_move::new::New as NewPackage;

use super::init::Init;

#[derive(Parser)]
/// Create a move package and initialize Carton.toml
pub struct Create {
    #[clap(flatten)]
    pub new: NewPackage,
}

impl Create {
    pub fn execute(self) -> Result<()> {
        let name = self.new.new.name.clone();
        let path = env::current_dir()?;

        self.new
            .execute(Some(path.clone()))
            .expect("An error occured while generating package");

        Init::init_carton(&path.join(name), false)?;

        Ok(())
    }
}
