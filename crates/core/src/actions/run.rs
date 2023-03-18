use std::{
    path::Path,
    process::{self, Command},
};

use anyhow::{bail, Result};

use crate::constants;

pub fn run_js_script(
    scripts_path: &Path,
    script: &str,
    private_key: &str,
    node_url: &str,
) -> Result<()> {
    let script_file = scripts_path.join(format!("{}", script));
    if !script_file.is_file() {
        bail!("Script file {} could not be found", script)
    }

    let mut output = Command::new(constants::NPX_CMD)
        .arg(constants::CARTON_TEST)
        .arg(format!(
            "--file={}",
            script_file.canonicalize()?.to_str().unwrap()
        ))
        .env(constants::NODE_URL_ARG, node_url)
        .env(constants::PRIVATE_KEY_ARG, private_key)
        .spawn()?;

    let status = output.wait()?;
    process::exit(status.code().unwrap())
}
