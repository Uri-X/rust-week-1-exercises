use rand::Rng;
use std::collections::HashMap;

// Name Assignment (variables and constants)
pub const MINING_REWARD: f64 = 3.125;
pub const CURRENT_BLOCK_HEIGHT: u64 = 895_000;
pub const BTC_TO_SATS: u64 = 100_000_000;

#[derive(Debug, Clone, PartialEq)]
pub struct Utxo {
    pub txid: String,
    pub vout: u32,
    pub value: u64,
}

pub fn calculate_total_reward(blocks_mined: u64) -> f64 {
    blocks_mined as f64 * MINING_REWARD
}

pub fn is_valid_tx_fee(fee: f64) -> bool {
    fee >= 0.00001 && fee <= 0.01
}

pub fn is_large_balance(balance: f64) -> bool {
    balance > 50.0
}

pub fn tx_priority(size_bytes: u64, fee_btc: f64) -> &'static str {
    let fee_rate = fee_btc / size_bytes as f64;
    if fee_rate > 0.00005 {
        "high"
    } else if fee_rate > 0.00001 {
        "medium"
    } else {
        "low"
    }
}

pub fn is_mainnet(network: &str) -> bool {
    network.to_lowercase() == "mainnet"
}

pub fn is_in_range(value: i64) -> bool {
    value >= 100 && value <= 200
}

pub fn is_same_wallet<T>(wallet1: &T, wallet2: &T) -> bool {
    std::ptr::eq(wallet1, wallet2)
}

pub fn normalize_address(address: &str) -> String {
    address.trim().to_lowercase()
}

pub fn add_utxo(mut utxos: Vec<Utxo>, new_utxo: Utxo) -> Vec<Utxo> {
    utxos.push(new_utxo);
    utxos
}

pub fn find_high_fee(fee_list: &[f64]) -> Option<(usize, f64)> {
    fee_list
        .iter()
        .enumerate()
        .find(|(_, &fee)| fee > 0.005)
        .map(|(i, &fee)| (i, fee))
}
pub fn get_wallet_details() -> (String, f64) {
    ("satoshi_wallet".to_string(), 50.0)
}

pub fn get_tx_status(tx_pool: &HashMap<String, String>, txid: &str) -> String {
    tx_pool
        .get(txid)
        .cloned()
        .unwrap_or_else(|| "not found".to_string())
}

pub fn unpack_wallet_info(wallet_info: (String, f64)) -> String {
    let (name, balance) = wallet_info;
    format!("Wallet {} has balance: {} BTC", name, balance)
}

pub fn calculate_sats(btc: f64) -> u64 {
    (btc * BTC_TO_SATS as f64) as u64
}

pub fn generate_address(prefix: &str) -> String {
    let mut rng = rand::thread_rng();
    let suffix_len = 32 - prefix.len();
    let charset: Vec<char> = "abcdefghijklmnopqrstuvwxyz0123456789".chars().collect();
    let suffix: String = (0..suffix_len)
        .map(|_| charset[rng.gen_range(0..charset.len())])
        .collect();
    format!("{}{}", prefix, suffix)
}

pub fn validate_block_height(height: i64) -> (bool, String) {
    if height < 0 {
        return (false, "Block height cannot be negative".to_string());
    }
    if height > 800_000 {
        return (false, "Block height is unrealistic".to_string());
    }
    (true, "Valid block height".to_string())
}

pub fn halving_schedule(blocks: &[u64]) -> HashMap<u64, u64> {
    let base_reward: u64 = 50 * 100_000_000;
    let halving_interval: u64 = 210_000;
    let mut result = HashMap::new();
    for &block in blocks {
        let halvings = block / halving_interval;
        let reward = base_reward >> halvings;
        result.insert(block, reward);
    }
    result
}

pub fn find_utxo_with_min_value(utxos: &[Utxo], target: u64) -> Option<Utxo> {
    utxos
        .iter()
        .filter(|u| u.value >= target)
        .min_by_key(|u| u.value)
        .cloned()
}

pub fn create_utxo(
    txid: &str,
    vout: u32,
    extra: HashMap<String, String>,
) -> HashMap<String, String> {
    let mut map = HashMap::new();
    map.insert("txid".to_string(), txid.to_string());
    map.insert("vout".to_string(), vout.to_string());
    for (k, v) in extra {
        map.insert(k, v);
    }
    map
}

pub fn extract_tx_version(raw_tx_hex: &str) -> Result<u32, String> {
    let bytes = match hex::decode(raw_tx_hex) {
        Ok(b) => b,
        Err(e) => return Err(format!("Hex decode error: {}", e)),
    };
    if bytes.len() < 4 {
        return Err("Transaction data too short".to_string());
    }
    let version = u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
    Ok(version)
}
