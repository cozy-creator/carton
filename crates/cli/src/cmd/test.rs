use std::{env, process::Command};

use anyhow::Result;
use clap::Parser;
use move_package::BuildConfig;

use carton_core::{path, state::State};
use move_cli::base::test::UnitTestResult;
use move_unit_test::UnitTestingConfig;
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
        let state = State::load(&root_path).await?;

        let package_path = match package {
            Some(package) => state.get_package_path(package)?,
            None => env::current_dir()?,
        };

        if self.js {
            let mut output = Command::new("npx")
                .arg("carton-test")
                .env("NODE_URL", "https://fullnode.devnet.sui.io:443/")
                .env(
                    "PRIVATE_KEY",
                    "AD3HOncCs0GkKmS0wRQHnlcb+FZXFCK455cRHs/ox4YI",
                )
                .current_dir(package_path.join("tests"))
                // .stdout(Stdio::inherit())
                // .stderr(Stdio::inherit())
                .spawn()?;

            // println!("{}", output.);
            // output.stdout.unwrap();

            // println!("{}", String::from_utf8(output.stdout.unwrap())?);
            // println!("{}", String::from_utf8(output.stderr.unwrap())?);

            let status = output.wait()?;
            println!("{}", status)
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
                std::process::exit(1)
            }
        }
        Ok(())
    }
}
