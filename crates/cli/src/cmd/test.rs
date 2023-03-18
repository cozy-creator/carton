use std::env;

use anyhow::Result;
use clap::Parser;
use move_package::BuildConfig;

use carton_core::{actions::test as test_action, path, state::State};

use sui_move::unit_test::Test as MoveTest;

#[derive(Parser)]
#[clap(author, version)]
/// Run tests in a Move package
pub struct Test {
    #[clap(flatten)]
    pub test: MoveTest,
    #[clap(flatten)]
    pub build_config: BuildConfig,
    #[clap(long)]
    pub js: bool,
}

impl Test {
    pub async fn execute(self, package: Option<String>) -> Result<()> {
        let root_path = path::get_root_path()?;
        let mut state = State::load(&root_path).await?;

        let package_path = match package {
            Some(package) => state.get_package_path(package)?,
            None => env::current_dir()?,
        };

        if self.js {
            // TODO: add tests path to manifest
            let tests_path = package_path.join("tests");
            let private_key = state.get_active_private_key()?;
            let env = state.get_active_env()?;

            test_action::run_js_tests(&tests_path, &private_key, env.rpc.as_str())?
        } else {
            test_action::run_move_tests(&package_path, &self.test, self.build_config)?
        }

        Ok(())
    }
}
