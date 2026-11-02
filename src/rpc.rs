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

pub fn get_ledger_entry_xdr(rpc_url: &str, key_xdr: &str) -> Result<Option<String>, String> {
    let body = json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "getLedgerEntries",
        "params": { "keys": [key_xdr] }
    });
    let response: Value = reqwest::blocking::Client::new()
        .post(rpc_url)
        .json(&body)
        .send()
        .map_err(|e| e.to_string())?
        .json()
        .map_err(|e| e.to_string())?;
    if !response["error"].is_null() {
        return Err(format!("rpc error: {}", response["error"]));
    }
    match response["result"]["entries"].as_array() {
        Some(entries) if !entries.is_empty() => entries[0]["xdr"]
            .as_str()
            .map(|s| Some(s.to_string()))
            .ok_or_else(|| format!("entry has no xdr: {}", entries[0])),
        _ => Ok(None),
    }
}
