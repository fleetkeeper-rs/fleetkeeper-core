fn main() {
    let contract_id = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCYSC".to_string());
    match fleetkeeper_core::fleet::get_contract_executable(
        fleetkeeper_core::rpc::TESTNET_RPC_URL,
        &contract_id,
    ) {
        Ok(e) => println!(
            "{contract_id}\n  runs: {}",
            fleetkeeper_core::fleet::describe_executable(&e)
        ),
        Err(err) => println!("error: {err}"),
    }
}
