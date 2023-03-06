pub const CARTON_MANIFEST_TEMPLATE: &str = "
[provider]
address = \"0x0\"
env = \"devnet\"
config = \"~/.sui/sui_config/client.yaml\"

[envs]
devnet = { url = \"https://fullnode.devnet.sui.io/\" }
testnet = { url = \"https://fullnode.testnet.sui.io/\" }
";
