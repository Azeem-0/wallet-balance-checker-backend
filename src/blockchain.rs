use ethers::utils::format_units;
use ethers::{
    abi::Abi,
    core::types::Address,
    prelude::*,
    providers::{Http, Provider},
};
use serde_json::from_reader;
use std::{fs, sync::Arc};
use toml::de::from_str;

use crate::asset_model::{Config, Utxo};

pub async fn get_erc20_balance(
    wallet_address: String,
    token_name: String,
) -> Result<String, Box<dyn std::error::Error>> {
    let config_str = fs::read_to_string("Config.toml")
        .map_err(|e| format!("Failed to read Config.toml: {}", e))?;

    let config: Config =
        from_str(&config_str).map_err(|e| format!("Failed to parse Config.toml: {}", e))?;

    let token = config
        .asset
        .iter()
        .find(|a| a.name == token_name)
        .ok_or_else(|| format!("Token '{}' not found in config", token_name))?;

    let provider = Provider::<Http>::try_from(&token.rpc_url)
        .map_err(|e| format!("Failed to create provider: {}", e))?;

    let token_address = token
        .contract_address
        .parse::<Address>()
        .map_err(|e| format!("Invalid token address: {}", e))?;

    let abi_file = fs::File::open("erc20-contract.json")
        .map_err(|e| format!("Failed to open ABI file: {}", e))?;

    let abi: Abi = from_reader(abi_file).map_err(|e| format!("Failed to parse ABI JSON: {}", e))?;

    let contract = Contract::new(token_address, abi, Arc::new(provider));

    let wallet_address = wallet_address
        .parse::<Address>()
        .map_err(|e| format!("Invalid wallet address: {}", e))?;

    let balance: U256 = contract
        .method::<_, U256>("balanceOf", wallet_address)?
        .call()
        .await
        .map_err(|e| format!("Failed to call balanceOf: {}", e))?;

    let decimals: u8 = contract
        .method::<_, u8>("decimals", ())?
        .call()
        .await
        .map_err(|e| format!("Failed to call decimals: {}", e))?;

    let formatted_balance = format_units(balance, decimals as usize)
        .map_err(|e| format!("Failed to format balance: {}", e))?;

    Ok(formatted_balance.to_string())
}

pub async fn get_sepolia_eth_balance(
    wallet_address: String,
) -> Result<String, Box<dyn std::error::Error>> {
    let rpc_url = "https://1rpc.io/sepolia";

    let provider = Provider::<Http>::try_from(rpc_url)?;
    let wallet = wallet_address.parse::<Address>()?;

    let balance: U256 = provider.get_balance(wallet, None).await?;
    let balance_in_eth = format_units(balance, 18)?;

    Ok(balance_in_eth)
}

pub async fn get_citrea_balance(
    wallet_address: String,
) -> Result<String, Box<dyn std::error::Error>> {
    let rpc_url = "https://rpc.testnet.citrea.xyz";

    let provider = Provider::<Http>::try_from(rpc_url)?;
    let wallet = wallet_address.parse::<Address>()?;

    let balance: U256 = provider.get_balance(wallet, None).await?;
    let formatted_balance = format_units(balance, 18)?;

    Ok(formatted_balance)
}

pub async fn get_testnet_btc_balance(
    wallet_address: String,
) -> Result<String, Box<dyn std::error::Error>> {
    let api_url = format!(
        "https://blockstream.info/testnet/api/address/{}/utxo",
        wallet_address
    );

    let client = reqwest::Client::new();

    let utxos: Vec<Utxo> = client.get(&api_url).send().await?.json().await?;

    let balance: u64 = utxos
        .iter()
        .filter(|utxo| utxo.status.confirmed)
        .map(|utxo| utxo.value)
        .sum();

    let formatted_balance = format_units(balance, 8)?;

    Ok(formatted_balance)
}
