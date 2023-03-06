use anyhow::{bail, Ok, Result};
use clap::Parser;
use std::{fs, io::Write};

use carton_core::{manifest, path};

#[derive(Parser)]
/// Initialize Carton in an existing directory
pub struct Init {}

const DEVNET_ENV_LINE: &str = "devnet = { url = \"https://fullnode.devnet.sui.io/\" }";
const TESTNET_ENV_LINE: &str = "testnet = { url = \"https://fullnode.testnet.sui.io/\" }";

impl Init {
    pub fn execute(self) -> Result<()> {
        let path = path::get_current_path()?.join(manifest::CARTON_MANIFEST_FILE_NAME);

        if !path.is_file() {
            let mut file = fs::File::create(&path)?;

            write!(
                file,
                "
[provider]
address = \"0x0\"
env = \"devnet\"
config = \"~/.sui/sui_config/client.yaml\"

[envs]
"
            )?;
            writeln!(
                file,
                "{}",
                format_args!("{}\n{}", DEVNET_ENV_LINE, TESTNET_ENV_LINE)
            )?;
        } else {
            bail!("Carton has already been initialized in this directory")
        }

        Ok(())
    }
}
