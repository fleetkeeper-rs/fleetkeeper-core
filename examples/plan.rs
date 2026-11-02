fn main() {
    let owner = "CCSC3UOXCCNB3VV7MM2P63IPNVKHQQ3TUMBCIW6OW42F75FBGMI4Y3VA";
    let tag = "fleetwatch-demo";
    let new_hash = "8d68f7ede8dd8656c8068736054f8991e2f29a8ff6d1215a5116875f7ea8987d";
    let members = vec![
        "CD6GZKPCBF2OFUV7VAGBFMWM4H2S2BBPI66CQ674RMFQAVLSVZOAWVSM".to_string(),
        "CDL6H5M2VYLOR536DDUG7SQXKJNLSADSIKNNMH5EW3R5R55QZJ2KXOZY".to_string(),
    ];
    match fleetkeeper_core::fleet::plan_upgrade(
        fleetkeeper_core::rpc::TESTNET_RPC_URL,
        owner,
        tag,
        new_hash,
        &members,
    ) {
        Ok(plan) => {
            println!("Upgrade plan for tag \"{}\" on owner {}", plan.tag, plan.owner);
            println!("  current hash: {}", plan.current_hash);
            println!("  new hash:     {}", plan.new_hash);
            if plan.already_current {
                println!("  nothing to do: the tag already holds the new hash");
            }
            for m in &plan.members {
                let mark = if m.in_fleet { "WILL CHANGE" } else { "not affected" };
                println!("  {mark}: {} ({})", m.contract_id, m.detail);
            }
        }
        Err(e) => println!("error: {e}"),
    }
}
