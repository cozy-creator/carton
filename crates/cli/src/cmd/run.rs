use std::{
    env,
    process::{self, Command},
};

use anyhow::{bail, Result};
use clap::Parser;

use carton_core::{path, state::State};

use crate::constants;

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

        let script_file = scripts_path.join(format!("{}", self.script));
        if !script_file.is_file() {
            bail!("Script file {} could not be found", self.script)
        }

        let private_key = state.get_active_private_key()?;
        let env = state.get_active_env()?;

        let mut output = Command::new(constants::NPX_CMD)
            .arg(constants::CARTON_TEST)
            .arg(format!(
                "--file={}",
                script_file.canonicalize()?.to_str().unwrap()
            ))
            .env(constants::NODE_URL_ARG, env.rpc.as_str())
            .env(constants::PRIVATE_KEY_ARG, private_key)
            .spawn()?;

        let status = output.wait()?;
        process::exit(status.code().unwrap())
    }
}
