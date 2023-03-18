use std::env;

use anyhow::{bail, Result};
use clap::Parser;

use carton_core::{actions::run as run_action, path, state::State};

#[derive(Parser)]
#[clap(author, version)]
/// Run a script in a package context
pub struct Run {
    /// Script file name (.js, .ts)
    #[clap(long)]
    pub script: String,
}

impl Run {
    pub async fn execute(self, package: Option<String>) -> Result<()> {
        let root_path = path::get_root_path()?;
        let mut state = State::load(&root_path).await?;

        let package_path = match package {
            Some(package) => state.get_package_path(package)?,
            None => env::current_dir()?,
        };

        // TODO: add scripts path to manifest
        let scripts_path = package_path.join("scripts");
        if !scripts_path.is_dir() {
            bail!("Unable to find scripts directory")
        }

        let private_key = state.get_active_private_key()?;
        let env = state.get_active_env()?;

        run_action::run_js_script(&scripts_path, &self.script, &private_key, env.rpc.as_str())
    }
}
