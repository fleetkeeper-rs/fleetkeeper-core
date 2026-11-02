use serde_json::{json, Value};

pub const TESTNET_RPC_URL: &str = "https://soroban-testnet.stellar.org";

pub fn latest_ledger(rpc_url: &str) -> Result<u64, String> {
    let body = json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "getLatestLedger"
    });
    let response: Value = reqwest::blocking::Client::new()
        .post(rpc_url)
        .json(&body)
        .send()
        .map_err(|e| e.to_string())?
        .json()
        .map_err(|e| e.to_string())?;
    response["result"]["sequence"]
        .as_u64()
        .ok_or_else(|| format!("unexpected response: {response}"))
}
