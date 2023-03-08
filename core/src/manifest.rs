use std::{
    collections::BTreeMap,
    env, fs,
    path::{Path, PathBuf},
    str::FromStr,
};

use anyhow::{bail, format_err, Context, Ok, Result};
use glob::glob;
use sui_types::base_types::SuiAddress;
use toml::Value as TomlValue;

use crate::path;

const ENVS_NAME: &str = "envs";
const PROVIDER_NAME: &str = "provider";
const WORKSPACE_NAME: &str = "workspace";

pub const MOVE_MANIFEST_FILE_NAME: &str = "Move.toml";
pub const CARTON_MANIFEST_FILE_NAME: &str = "Carton.toml";

pub type Envs = BTreeMap<String, Env>;
pub type Members = BTreeMap<String, String>;

pub struct Manifest {
    pub envs: Option<Envs>,
    pub provider: Provider,
    pub members: Option<Members>,
}

#[derive(Clone)]
pub struct Provider {
    pub env: String,
    pub address: SuiAddress,
    pub config: PathBuf,
}

#[derive(Clone)]
pub struct Env {
    pub url: String,
}

impl Manifest {
    pub fn load_from_path(path: &Path) -> String {
        if path.is_file() {
            fs::read_to_string(path).unwrap()
        } else {
            fs::read_to_string(path.join(CARTON_MANIFEST_FILE_NAME)).unwrap()
        }
    }

    pub fn parse_from_path(path: &Path) -> Result<Self> {
        let content = Self::load_from_path(path);
        parse_manifest_value(toml::from_str::<TomlValue>(&content).unwrap())
    }
}

fn parse_manifest_value(value: TomlValue) -> Result<Manifest> {
    match value {
        TomlValue::Table(mut value) => {
            let envs = value.remove(ENVS_NAME).map(parse_envs).transpose()?;
            let provider = value
                .remove(PROVIDER_NAME)
                .map(parse_provider)
                .transpose()?
                .context("Error parsing the [provider] section of the Carton manifest")?;
            let members = value
                .remove(WORKSPACE_NAME)
                .map(parse_members)
                .transpose()?;

            Ok(Manifest {
                envs,
                members,
                provider,
            })
        }
        v => bail!(
            "Invalid value {}. Expected \"table\" but found \"{}\"",
            v,
            v.type_str()
        ),
    }
}

fn parse_provider(value: TomlValue) -> Result<Provider> {
    match value {
        TomlValue::Table(mut value) => {
            let env = value
                .remove("env")
                .ok_or_else(|| format_err!("Provider env value must be a string"))
                .map(parse_string)??;

            let config_path = value
                .remove("config")
                .ok_or_else(|| format_err!("Provider config value must be a string"))
                .map(parse_string)??;

            let address = value
                .remove("address")
                .ok_or_else(|| format_err!("Provider address value must be a string"))
                .map(parse_string)??;

            let config = match config_path.strip_prefix("~/") {
                Some(path) => dirs::home_dir().unwrap().join(path),
                None => env::current_dir()?.join(config_path),
            };

            Ok(Provider {
                env,
                config,
                address: SuiAddress::from_str(&address)?,
            })
        }
        v => bail!(
            "Invalid value {}. Expected \"table\" but found \"{}\"",
            v,
            v.type_str()
        ),
    }
}

fn parse_envs(value: TomlValue) -> Result<Envs> {
    match value {
        TomlValue::Table(value) => {
            let mut envs = BTreeMap::new();

            for (n, value) in value.into_iter() {
                envs.insert(n, parse_env(value)?);
            }

            Ok(envs)
        }
        v => bail!(
            "Invalid value {}. Expected \"table\" but found \"{}\"",
            v,
            v.type_str()
        ),
    }
}

fn parse_members(value: TomlValue) -> Result<Members> {
    match value {
        TomlValue::Table(mut value) => value
            .remove("members")
            .map(|v| match v {
                TomlValue::Array(v) => {
                    let mut map = BTreeMap::new();

                    for member in v.into_iter() {
                        let member_path = parse_string(member)?;

                        if member_path.contains("*") {
                            let entries = glob(&member_path)?;
                            for entry in entries {
                                let entry = entry?;

                                if entry.is_dir() {
                                    let path = entry.to_str().unwrap();
                                    let name = get_move_package_name(&path)?;
                                    map.insert(name, path.to_string());
                                }
                            }
                        } else {
                            let name = get_move_package_name(&member_path)?;
                            map.insert(name, member_path);
                        }
                    }

                    Ok(map)
                }
                v => bail!(
                    "Invalid value {}. Expected \"array\" but found \"{}\"",
                    v,
                    v.type_str()
                ),
            })
            .unwrap(),
        v => bail!(
            "Invalid value {}. Expected \"table\" but found \"{}\"",
            v,
            v.type_str()
        ),
    }
}

fn parse_env(value: TomlValue) -> Result<Env> {
    match value {
        TomlValue::Table(mut value) => {
            let url = parse_string(value.remove("url").unwrap())?;

            Ok(Env { url })
        }
        v => bail!(
            "Invalid value {}. Expected \"table\" but found \"{}\"",
            v,
            v.type_str()
        ),
    }
}

fn parse_string(value: TomlValue) -> Result<String> {
    match value {
        TomlValue::String(v) => Ok(v),
        v => bail!(
            "Invalid value {}. Expected \"string\" but found \"{}\"",
            v,
            v.type_str()
        ),
    }
}

fn get_move_package_name(package_path: &str) -> Result<String> {
    let mut path = path::get_root_path(None)
        .unwrap()
        .join(package_path)
        .join(MOVE_MANIFEST_FILE_NAME);

    if !path.is_file() {
        path.pop();
        bail!(
            "Carton workspace member does not exist at {}",
            path.to_string_lossy()
        );
    }

    let table = toml::from_str::<TomlValue>(&Manifest::load_from_path(&path)).unwrap();

    match table {
        TomlValue::Table(mut v) => {
            let package = v.remove("package").ok_or_else(|| {
                format_err!("Error parsing the [package] section of package Move.toml")
            })?;

            match package {
                TomlValue::Table(mut t) => Ok(t.remove("name").map(parse_string).unwrap()?),
                v => bail!(
                    "Invalid value {}. Expected \"table\" but found \"{}\"",
                    v,
                    v.type_str()
                ),
            }
        }
        v => bail!(
            "Invalid value {}. Expected \"table\" but found \"{}\"",
            v,
            v.type_str()
        ),
    }
}
