fn main() {
    match fleetkeeper_core::rpc::latest_ledger(fleetkeeper_core::rpc::TESTNET_RPC_URL) {
        Ok(n) => println!("testnet latest ledger: {n}"),
        Err(e) => println!("error: {e}"),
    }
}
