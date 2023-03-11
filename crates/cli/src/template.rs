pub const CARTON_MANIFEST_TEMPLATE: &str = "
[provider]
address = \"0000000000000000000000000000000000000000\"
env = \"devnet\"
config = \"~/.sui/sui_config/client.yaml\"

[envs]
devnet = { url = \"https://fullnode.devnet.sui.io:443/\" }
testnet = { url = \"https://fullnode.testnet.sui.io:443/\" }
";
