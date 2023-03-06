use std::path::PathBuf;

use anyhow::Result;
use move_package::BuildConfig;
use sui::client_commands::{SuiClientCommandResult, SuiClientCommands, WalletContext};
use sui_json_rpc_types::SuiTransactionResponse;

pub async fn publish_package(
    package_path: PathBuf,
    context: &mut WalletContext,
    build_config: BuildConfig,
) -> Result<SuiTransactionResponse> {
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
        Ok(response)
    } else {
        panic!("")
    }
}
