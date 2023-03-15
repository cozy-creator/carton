use std::io::{self, Write};
use std::path::PathBuf;

use anyhow::{bail, Ok, Result};
use colored::Colorize;
use move_package::BuildConfig;
use sui::client_commands::{SuiClientCommandResult, SuiClientCommands, WalletContext};
use sui_json_rpc_types::{SuiEvent, SuiTransactionResponse};

pub async fn publish_package(
    package_path: PathBuf,
    context: &mut WalletContext,
    build_config: BuildConfig,
) -> Result<SuiTransactionResponse> {
    let mut w = io::stdout();

    let command = SuiClientCommands::Publish {
        package_path,
        build_config,
        gas: None,
        gas_budget: 30000,
        skip_dependency_verification: true,
        with_unpublished_dependencies: true,
    };

    let result = command.execute(context).await?;
    if let SuiClientCommandResult::Publish(response) = result {
        writeln!(
            &mut w,
            "\nTransaction Digest: {}",
            response.certificate.transaction_digest
        )
        .unwrap();

        response.effects.events.iter().for_each(|e| {
            if let SuiEvent::Publish { package_id, .. } = e {
                writeln!(&mut w, "Package ID: {}", package_id).unwrap()
            }
        });

        if !response.effects.created.is_empty() {
            writeln!(&mut w, "\nObjects Created")?;

            response.effects.created.iter().for_each(|object| {
                writeln!(
                    &mut w,
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
            writeln!(&mut w, "\nObjects Mutated")?;

            response.effects.mutated.iter().for_each(|object| {
                writeln!(
                    &mut w,
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
            writeln!(&mut w, "\nObjects Deleted")?;

            response.effects.deleted.iter().for_each(|object| {
                writeln!(
                    &mut w,
                    "  - Object ID: {}\n    Digest: {}\n    Version: {}",
                    object.object_id,
                    object.digest,
                    &object.version.value()
                )
                .unwrap()
            });
        }

        writeln!(&mut w, "{}", "\nPUBLISHED SUCCESSFULLY 🥳".green().bold())?;

        Ok(response)
    } else {
        bail!("")
    }
}
