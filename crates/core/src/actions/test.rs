use std::{
    path::Path,
    process::{self, Command},
};

use anyhow::{bail, Ok, Result};
use move_cli::base::test::UnitTestResult;
use move_package::BuildConfig;
use move_unit_test::UnitTestingConfig;
use sui_move::unit_test::Test;

use crate::constants;

pub fn run_js_tests(tests_path: &Path, private_key: &str, node_url: &str) -> Result<()> {
    if !tests_path.is_dir() {
        bail!("Unable to find tests directory")
    }

    let mut output = Command::new(constants::NPX_CMD)
        .arg(constants::CARTON_TEST)
        .env(constants::NODE_URL_ARG, node_url)
        .env(constants::PRIVATE_KEY_ARG, private_key)
        .current_dir(tests_path)
        .spawn()?;

    let status = output.wait()?;
    process::exit(status.code().unwrap())
}

pub fn run_move_tests(package_path: &Path, test: &Test, build_config: BuildConfig) -> Result<()> {
    let Test { test: itest } = test;

    let unit_test_config = UnitTestingConfig {
        gas_limit: itest.gas_limit,
        filter: itest.filter.clone(),
        list: itest.list,
        num_threads: itest.num_threads,
        report_statistics: itest.report_statistics.clone(),
        report_storage_on_error: itest.report_storage_on_error,
        check_stackless_vm: itest.check_stackless_vm,
        verbose: itest.verbose_mode,
        ..UnitTestingConfig::default_with_bound(None)
    };

    if let UnitTestResult::Failure = test.execute(
        Some(package_path.to_path_buf()),
        build_config,
        unit_test_config,
    )? {
        process::exit(1)
    }

    Ok(())
}
