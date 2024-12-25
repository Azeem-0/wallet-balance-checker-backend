use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Asset {
    pub name: String,
    pub chain: String,
    pub contract_address: String,
    pub rpc_url: String,
}

#[derive(Deserialize, Debug)]
pub struct Config {
    pub asset: Vec<Asset>,
}

#[derive(Deserialize)]
pub struct Utxo {
    pub value: u64,
    pub status: UtxoStatus,
}

#[derive(Deserialize)]
pub struct UtxoStatus {
    pub confirmed: bool,
}
