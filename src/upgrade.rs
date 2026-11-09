use crate::fleet::{get_tag_hash, verify_fleet};
use std::fs::OpenOptions;
use std::io::Write;
use std::process::Command;

const HISTORY_FILE: &str = ".fleetkeeper-history.csv";

pub struct OwnerCall {
    pub owner: String,
    pub function: String,
    pub tag_arg: String,
    pub hash_arg: String,
    pub source: String,
    pub network: String,
}

fn check_hash(hash: &str) -> Result<String, String> {
    let hash = hash.to_lowercase();
    if hash.len() != 64 || !hash.chars().all(|c| c.is_ascii_hexdigit()) {
        return Err("hash must be 64 hex characters".to_string());
    }
    Ok(hash)
}

fn run_owner_call(call: &OwnerCall, tag: &str, hash: &str, send: bool) -> Result<String, String> {
    let mut cmd = Command::new("stellar");
    cmd.arg("contract")
        .arg("invoke")
        .arg("--id")
        .arg(&call.owner)
        .arg("--source-account")
        .arg(&call.source)
        .arg("--network")
        .arg(&call.network);
    if !send {
        cmd.arg("--send=no");
    }
    cmd.arg("--")
        .arg(&call.function)
        .arg(format!("--{}", call.tag_arg))
        .arg(tag)
        .arg(format!("--{}", call.hash_arg))
        .arg(hash);
    let output = cmd
        .output()
        .map_err(|e| format!("could not run the stellar CLI: {e}"))?;
    let text = format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    if output.status.success() {
        Ok(text)
    } else {
        Err(text)
    }
}

fn record_history(owner: &str, tag: &str, old_hash: &str, new_hash: &str) -> Result<(), String> {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(HISTORY_FILE)
        .map_err(|e| e.to_string())?;
    writeln!(file, "{owner},{tag},{old_hash},{new_hash}").map_err(|e| e.to_string())
}

pub fn previous_hash(owner: &str, tag: &str) -> Option<String> {
    let text = std::fs::read_to_string(HISTORY_FILE).ok()?;
    text.lines().rev().find_map(|line| {
        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() == 4 && parts[0] == owner && parts[1] == tag {
            Some(parts[2].to_string())
        } else {
            None
        }
    })
}

fn change_tag(
    rpc_url: &str,
    call: &OwnerCall,
    tag: &str,
    new_hash: &str,
    members: &[String],
    send: bool,
) -> Result<String, String> {
    let new_hash = check_hash(new_hash)?;
    let current = get_tag_hash(rpc_url, &call.owner, tag)?;
    if current == new_hash {
        return Err("the tag already holds that hash, nothing to do".to_string());
    }
    let output = run_owner_call(call, tag, &new_hash, send)?;
    if !send {
        return Ok(format!("dry run, nothing sent:\n{output}"));
    }
    record_history(&call.owner, tag, &current, &new_hash)?;
    let report = verify_fleet(rpc_url, &call.owner, tag, &new_hash, members)?;
    Ok(format!(
        "sent: tag moved from {current} to {new_hash}\nverify: {}",
        if report.ok { "OK" } else { "FAILED" }
    ))
}

pub fn upgrade_fleet(
    rpc_url: &str,
    call: &OwnerCall,
    tag: &str,
    new_hash: &str,
    members: &[String],
    send: bool,
) -> Result<String, String> {
    change_tag(rpc_url, call, tag, new_hash, members, send)
}

pub fn rollback_fleet(
    rpc_url: &str,
    call: &OwnerCall,
    tag: &str,
    members: &[String],
    send: bool,
) -> Result<String, String> {
    let previous = previous_hash(&call.owner, tag)
        .ok_or_else(|| "no history for this owner and tag".to_string())?;
    change_tag(rpc_url, call, tag, &previous, members, send)
}
