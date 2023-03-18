use std::env;

use anyhow::{Ok, Result};
use clap::Parser;
use move_package::BuildConfig;
use sui_types::base_types::{ObjectID, SuiAddress};

use carton_core::{
    actions::publish::{self as publish_action, PublishOptions},
    path,
    state::State,
};

#[derive(Parser)]
#[clap(author, version)]
/// Publish a Move package to the network
pub struct Publish {
    #[clap(flatten)]
    build_config: BuildConfig,

    /// Network environment where the package should be deployed to
    /// If not provided, the env in Carton.toml will be used
    #[clap(long)]
    env: Option<String>,

    /// Address of the account that should be used in publishing the package
    /// If not provided, the address in Carton.toml will be used used
    #[clap(long)]
    publisher: Option<SuiAddress>,

    /// ID of the gas object for gas payment, in 20 bytes Hex string
    /// If not provided, a gas object with at least gas_budget value will be selected
    #[clap(long)]
    gas: Option<ObjectID>,

    /// Gas budget for running module initializers
    #[clap(long)]
    gas_budget: u64,

    /// Publish the package without checking whether compiling dependencies from source results
    /// in bytecode matching the dependencies found on-chain.
    #[clap(long)]
    skip_dependency_verification: bool,

    /// Also publish transitive dependencies that have not already been published.
    #[clap(long)]
    with_unpublished_dependencies: bool,
}

impl Publish {
    pub async fn execute(self, package: Option<String>) -> Result<()> {
        let root_path = path::get_root_path()?;
        let mut state = State::load(&root_path).await?;

        if let Some(env) = self.env {
            state.set_active_env(&env);
        }

        if let Some(publisher) = self.publisher {
            state.set_active_address(publisher);
        }

        let package_path = match package {
            Some(package) => state.get_package_path(package)?,
            None => env::current_dir()?,
        };

        let options = PublishOptions {
            gas: self.gas,
            gas_budget: self.gas_budget,
            skip_dependency_verification: self.skip_dependency_verification,
            with_unpublished_dependencies: self.with_unpublished_dependencies,
        };

        publish_action::publish_package(
            package_path,
            options,
            self.build_config,
            &mut state.context,
        )
        .await?;

        Ok(())
    }
}
