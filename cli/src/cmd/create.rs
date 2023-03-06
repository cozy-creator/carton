use anyhow::{Ok, Result};
use clap::Parser;
use std::{env, fs, io::Write};
use sui_move::new::New as NewPackage;

use carton_core::manifest;

#[derive(Parser)]
/// Create a new move package
pub struct Create {
    #[clap(flatten)]
    pub new: NewPackage,
}

const DEVNET_ENV_LINE: &str = "devnet = { url = \"https://fullnode.devnet.sui.io/\" }";
const TESTNET_ENV_LINE: &str = "testnet = { url = \"https://fullnode.testnet.sui.io/\" }";

impl Create {
    pub fn execute(self) -> Result<()> {
        let name = self.new.new.name.clone();
        let path = env::current_dir()?;

        self.new
            .execute(Some(path.clone()))
            .expect("An error occured while generating package");

        if !path.join(manifest::CARTON_MANIFEST_FILE_NAME).is_file() {
            let config_path = path.join(&name).join(manifest::CARTON_MANIFEST_FILE_NAME);
            let mut file = fs::File::create(&config_path)?;

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
        }

        Ok(())
    }
}
