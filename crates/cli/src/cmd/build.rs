use std::env;

use anyhow::Result;
use clap::Parser;
use move_package::BuildConfig;
use sui_move::build::Build as BuildPackage;

use carton_core::{path, state::State};

#[derive(Parser)]
#[clap(author, version)]
/// Build a Move package
pub struct Build {
    #[clap(flatten)]
    pub build: BuildPackage,
    #[clap(flatten)]
    pub build_config: BuildConfig,
}

impl Build {
    pub async fn execute(self, package: Option<String>) -> Result<()> {
        let root_path = path::get_root_path()?;
        let state = State::load(&root_path).await?;

        let package_path = match package {
            Some(package) => state.get_package_path(package)?,
            None => env::current_dir()?,
        };

        self.build.execute(Some(package_path), self.build_config)
    }
}
