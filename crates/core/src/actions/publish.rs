use std::io::{self, Stdout, Write};
use std::path::PathBuf;

use anyhow::{bail, Ok, Result};
use colored::Colorize;
use move_package::BuildConfig;
use sui::client_commands::{SuiClientCommandResult, SuiClientCommands, WalletContext};
use sui_json_rpc_types::{SuiEvent, SuiTransactionResponse};
use sui_types::base_types::ObjectID;

pub struct PublishOptions {
    pub gas: Option<ObjectID>,
    pub gas_budget: u64,
    pub skip_dependency_verification: bool,
    pub with_unpublished_dependencies: bool,
}

pub async fn run(
    package_path: PathBuf,
    options: PublishOptions,
    build_config: BuildConfig,
    context: &mut WalletContext,
) -> Result<SuiTransactionResponse> {
    let mut w = io::stdout();

    let command = SuiClientCommands::Publish {
        package_path,
        build_config,
        gas: options.gas,
        gas_budget: options.gas_budget,
        skip_dependency_verification: options.skip_dependency_verification,
        with_unpublished_dependencies: options.with_unpublished_dependencies,
    };

    let result = command.execute(context).await?;
    if let SuiClientCommandResult::Publish(response) = result {
        stdout_write_reponse(&mut w, &response)?;
        Ok(response)
    } else {
        bail!("Invalid command result")
    }
}

fn stdout_write_reponse(w: &mut Stdout, response: &SuiTransactionResponse) -> Result<()> {
    writeln!(
        w,
        "\nTransaction Digest: {}",
        response.certificate.transaction_digest
    )?;

    response.effects.events.iter().for_each(|e| {
        if let SuiEvent::Publish { package_id, .. } = e {
            writeln!(w, "Package ID: {}", package_id).unwrap()
        }
    });

    if !response.effects.created.is_empty() {
        writeln!(w, "\nObjects Created")?;

        response.effects.created.iter().for_each(|object| {
            writeln!(
                w,
                "  - Object ID: {}\n    Owner: {}\n    Digest: {}\n    Version: {}",
                object.reference.object_id,
                object.owner,
                object.reference.digest,
                &object.reference.version.value()
            )
            .unwrap();
        });
    }

    if !response.effects.mutated.is_empty() {
        writeln!(w, "\nObjects Mutated")?;

        response.effects.mutated.iter().for_each(|object| {
            writeln!(
                w,
                "  - Object ID: {}\n    Owner: {}\n    Digest: {}\n    Version: {}",
                object.reference.object_id,
                object.owner,
                object.reference.digest,
                &object.reference.version.value()
            )
            .unwrap()
        });
    }

    if !response.effects.deleted.is_empty() {
        writeln!(w, "\nObjects Deleted")?;

        response.effects.deleted.iter().for_each(|object| {
            writeln!(
                w,
                "  - Object ID: {}\n    Digest: {}\n    Version: {}",
                object.object_id,
                object.digest,
                &object.version.value()
            )
            .unwrap()
        });
    }

    writeln!(w, "{}", "\nPUBLISHED SUCCESSFULLY 🥳".green().bold())?;

    Ok(())
}
