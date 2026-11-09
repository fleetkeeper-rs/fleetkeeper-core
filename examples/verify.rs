fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.len() < 4 {
        println!("usage: verify OWNER TAG EXPECTED_HASH MEMBER [MEMBER...]");
        return;
    }
    let members: Vec<String> = args[3..].to_vec();
    match fleetkeeper_core::fleet::verify_fleet(
        fleetkeeper_core::rpc::TESTNET_RPC_URL,
        &args[0],
        &args[1],
        &args[2],
        &members,
    ) {
        Ok(report) => {
            println!("expected hash: {}", report.expected_hash);
            println!("tag holds:     {}", report.tag_hash);
            println!("tag matches:   {}", report.tag_matches);
            for m in &report.members {
                let mark = if m.in_fleet { "ok" } else { "PROBLEM" };
                println!("  {mark}: {} ({})", m.contract_id, m.detail);
            }
            println!("{}", if report.ok { "RESULT: OK" } else { "RESULT: FAILED" });
        }
        Err(e) => println!("error: {e}"),
    }
}
