fn main() {
    let owner = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "CCSC3UOXCCNB3VV7MM2P63IPNVKHQQ3TUMBCIW6OW42F75FBGMI4Y3VA".to_string());
    let tag = std::env::args()
        .nth(2)
        .unwrap_or_else(|| "fleetwatch-demo".to_string());
    match fleetkeeper_core::fleet::get_tag_value(
        fleetkeeper_core::rpc::TESTNET_RPC_URL,
        &owner,
        &tag,
    ) {
        Ok(value) => println!("owner {owner}\n  tag \"{tag}\" holds: {value:?}"),
        Err(err) => println!("error: {err}"),
    }
}
