use std::{path::PathBuf, rc::Rc};

use anyhow::{bail, Ok, Result};
use sui::{
    client_commands::WalletContext,
    config::{PersistedConfig, SuiClientConfig, SuiEnv},
};
use sui_types::base_types::SuiAddress;

use crate::manifest::{Envs, Manifest};

type StateManifest = Rc<Manifest>;

pub struct State {
    manifest: StateManifest,
    pub root_path: PathBuf,
    pub context: WalletContext,
}

impl State {
    pub async fn load(path: PathBuf) -> Result<Self> {
        let manifest = Rc::new(Manifest::parse_from_path(&path)?);
        let context = get_context(Rc::clone(&manifest)).await;

        Ok(Self {
            context,
            manifest,
            root_path: path,
        })
    }

    pub fn set_active_address(&mut self, address: SuiAddress) {
        set_active_address(&mut self.context.config, address);
    }

    pub fn set_active_env(&mut self, env: &str) {
        set_active_env(&mut self.context.config, env);
    }

    pub fn get_package_path(&self, package_name: String) -> Result<PathBuf> {
        if let Some(members) = &self.manifest.members {
            match members.get(&package_name) {
                Some(package_path) => Ok(self.root_path.join(package_path)),
                None => bail!(
                    "The package \"{}\" cannot not found in the current workspace",
                    package_name
                ),
            }
        } else {
            bail!("Cannot specify package in a non workspace project")
        }
    }
}

async fn get_context(manifest: StateManifest) -> WalletContext {
    let mut context = WalletContext::new(&manifest.provider.config, None)
        .await
        .unwrap();

    if let Some(envs) = &manifest.envs {
        set_envs(&mut context.config, envs);
    }

    set_active_env(&mut context.config, &manifest.provider.env);
    set_active_address(&mut context.config, manifest.provider.address);

    context
}

fn set_envs(config: &mut PersistedConfig<SuiClientConfig>, envs: &Envs) {
    for (key, value) in envs.into_iter() {
        if let Some(idx) = config.envs.iter().position(|e| &e.alias == key) {
            config.envs.remove(idx);
        };

        config.add_env(SuiEnv {
            alias: key.to_string(),
            rpc: value.url.to_string(),
            ws: None,
        });
    }
}

fn set_active_address(config: &mut PersistedConfig<SuiClientConfig>, address: SuiAddress) {
    config.active_address = Some(address);
}

fn set_active_env(config: &mut PersistedConfig<SuiClientConfig>, env: &str) {
    config.active_env = Some(env.to_string());
}
