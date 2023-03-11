use std::env;

use anyhow::Result;
use clap::Parser;
use move_package::BuildConfig;

use carton_core::{path, state::State};
use move_cli::base::test::UnitTestResult;
use move_unit_test::UnitTestingConfig;
use sui_move::unit_test::Test as MoveTest;

#[derive(Parser)]
/// Run tests in a Move package
#[clap(author, version)]
pub struct Test {
    #[clap(flatten)]
    pub test: MoveTest,
    #[clap(flatten)]
    pub build_config: BuildConfig,
}

impl Test {
    pub async fn execute(self, package: Option<String>) -> Result<()> {
        let root_path = path::get_root_path()?;
        let state = State::load(&root_path).await?;

        let package_path = match package {
            Some(package) => state.get_package_path(package)?,
            None => env::current_dir()?,
        };

        let Test { test, build_config } = self;

        let unit_test_config = UnitTestingConfig {
            gas_limit: test.test.gas_limit,
            filter: test.test.filter.clone(),
            list: test.test.list,
            num_threads: test.test.num_threads,
            report_statistics: test.test.report_statistics.clone(),
            report_storage_on_error: test.test.report_storage_on_error,
            check_stackless_vm: test.test.check_stackless_vm,
            verbose: test.test.verbose_mode,

            ..UnitTestingConfig::default_with_bound(None)
        };

        if let UnitTestResult::Failure =
            test.execute(Some(package_path), build_config, unit_test_config)?
        {
            std::process::exit(1)
        }

        Ok(())
    }
}
