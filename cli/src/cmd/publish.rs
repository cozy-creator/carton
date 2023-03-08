use anyhow::{Ok, Result};
use clap::Parser;
use move_package::BuildConfig;
use sui_types::base_types::SuiAddress;

use carton_core::{actions, path, state::State};

#[derive(Parser)]
#[clap(author, version)]
pub struct Publish {
    #[clap(flatten)]
    build_config: BuildConfig,
    #[clap(flatten)]
    options: PublishOptions,
}

#[derive(Parser)]
pub struct PublishOptions {
    #[clap(long)]
    env: Option<String>,
    #[clap(long)]
    publisher: Option<SuiAddress>,
    // /// ID of the gas object for gas payment, in 20 bytes Hex string
    // /// If not provided, a gas object with at least gas_budget value will be selected
    // #[clap(long)]
    // gas: Option<ObjectID>,

    // /// Gas budget for running module initializers
    // #[clap(long)]
    // gas_budget: u64,

    // /// Publish the package without checking whether compiling dependencies from source results
    // /// in bytecode matching the dependencies found on-chain.
    // #[clap(long)]
    // skip_dependency_verification: bool,

    // /// Also publish transitive dependencies that have not already been published.
    // #[clap(long)]
    // with_unpublished_dependencies: bool,
}

impl Publish {
    pub async fn execute(self, package: Option<String>) -> Result<()> {
        let root_path = path::get_root_path(None).unwrap();
        let mut state = State::load(root_path.to_path_buf()).await?;

        if let Some(env) = self.options.env {
            state.set_active_env(&env);
        }

        if let Some(publisher) = self.options.publisher {
            state.set_active_address(publisher);
        }

        let package_path = match package {
            Some(package) => state.get_package_path(package)?,
            None => root_path,
        };

        actions::publish_package(package_path, &mut state.context, self.build_config).await?;

        Ok(())
    }
}
