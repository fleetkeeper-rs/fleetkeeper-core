use fleetkeeper_core::upgrade::{rollback_fleet, upgrade_fleet, OwnerCall};

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.len() < 5 {
        println!("usage: upgrade ACTION OWNER TAG HASH MEMBER [MEMBER...]");
        println!("ACTION is upgrade, dry-run or rollback (use - as HASH for rollback)");
        return;
    }
    let call = OwnerCall {
        owner: args[1].clone(),
        function: "set_ref".to_string(),
        tag_arg: "name".to_string(),
        hash_arg: "wasm_hash".to_string(),
        source: "alice".to_string(),
        network: "testnet".to_string(),
    };
    let rpc = fleetkeeper_core::rpc::TESTNET_RPC_URL;
    let members: Vec<String> = args[4..].to_vec();
    let result = match args[0].as_str() {
        "upgrade" => upgrade_fleet(rpc, &call, &args[2], &args[3], &members, true),
        "dry-run" => upgrade_fleet(rpc, &call, &args[2], &args[3], &members, false),
        "rollback" => rollback_fleet(rpc, &call, &args[2], &members, true),
        other => Err(format!("unknown action: {other}")),
    };
    match result {
        Ok(text) => println!("{text}"),
        Err(e) => println!("error: {e}"),
    }
}
