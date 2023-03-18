use std::{
    env,
    process::{self, Command},
};

use anyhow::{bail, Result};
use clap::Parser;
use move_package::BuildConfig;

use carton_core::{path, state::State};
use move_cli::base::test::UnitTestResult;
use move_unit_test::UnitTestingConfig;
use sui_move::unit_test::Test as MoveTest;

use crate::constants;

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
            if !tests_path.is_dir() {
                bail!("Unable to find tests directory")
            }

            let private_key = state.get_active_private_key()?;
            let env = state.get_active_env()?;

            let mut output = Command::new(constants::NPX_CMD)
                .arg(constants::CARTON_TEST)
                .env(constants::NODE_URL_ARG, env.rpc.as_str())
                .env(constants::PRIVATE_KEY_ARG, private_key)
                .current_dir(tests_path)
                .spawn()?;

            let status = output.wait()?;
            process::exit(status.code().unwrap())
        } else {
            let MoveTest { test } = &self.test;

            let unit_test_config = UnitTestingConfig {
                gas_limit: test.gas_limit,
                filter: test.filter.clone(),
                list: test.list,
                num_threads: test.num_threads,
                report_statistics: test.report_statistics.clone(),
                report_storage_on_error: test.report_storage_on_error,
                check_stackless_vm: test.check_stackless_vm,
                verbose: test.verbose_mode,
                ..UnitTestingConfig::default_with_bound(None)
            };

            if let UnitTestResult::Failure =
                self.test
                    .execute(Some(package_path), self.build_config, unit_test_config)?
            {
                process::exit(1)
            }
        }

        Ok(())
    }
}
