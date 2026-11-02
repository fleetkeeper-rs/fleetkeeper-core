use stellar_xdr::{
    ContractDataDurability, ContractExecutable, ContractId, Hash, LedgerEntryData, LedgerKey,
    LedgerKeyContractData, Limits, ReadXdr, ScAddress, ScVal, WriteXdr,
};

pub fn get_contract_executable(
    rpc_url: &str,
    contract_id: &str,
) -> Result<ContractExecutable, String> {
    let contract = stellar_strkey::Contract::from_string(contract_id)
        .map_err(|e| format!("bad contract id: {e:?}"))?;
    let key = LedgerKey::ContractData(LedgerKeyContractData {
        contract: ScAddress::Contract(ContractId(Hash(contract.0))),
        key: ScVal::LedgerKeyContractInstance,
        durability: ContractDataDurability::Persistent,
    });
    let key_xdr = key
        .to_xdr_base64(Limits::none())
        .map_err(|e| e.to_string())?;
    let entry_xdr = crate::rpc::get_ledger_entry_xdr(rpc_url, &key_xdr)?
        .ok_or_else(|| "contract not found on this network".to_string())?;
    let data = LedgerEntryData::from_xdr_base64(entry_xdr, Limits::none())
        .map_err(|e| e.to_string())?;
    match data {
        LedgerEntryData::ContractData(entry) => match entry.val {
            ScVal::ContractInstance(instance) => Ok(instance.executable),
            _ => Err("entry is not a contract instance".to_string()),
        },
        _ => Err("entry is not contract data".to_string()),
    }
}

fn to_hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect::<String>()
}

fn owner_text(owner: &ScAddress) -> String {
    match owner {
        ScAddress::Contract(ContractId(Hash(bytes))) => {
            stellar_strkey::Contract(*bytes).to_string().as_str().to_string()
        }
        other => format!("{other:?}"),
    }
}

pub fn describe_executable(executable: &ContractExecutable) -> String {
    match executable {
        ContractExecutable::Wasm(hash) => format!("Wasm hash {}", to_hex(&hash.0)),
        ContractExecutable::StellarAsset => "Stellar asset contract (built-in code)".to_string(),
        ContractExecutable::ExternalRef(r) => format!(
            "External reference: owner {}, tag \"{}\"",
            owner_text(&r.executable_owner),
            r.tag.to_utf8_string_lossy()
        ),
    }
}

pub fn get_tag_value(rpc_url: &str, owner_id: &str, tag: &str) -> Result<ScVal, String> {
    let owner = stellar_strkey::Contract::from_string(owner_id)
        .map_err(|e| format!("bad owner id: {e:?}"))?;
    let tag_string = stellar_xdr::ScString(
        tag.as_bytes()
            .to_vec()
            .try_into()
            .map_err(|_| "tag is too long".to_string())?,
    );
    let key = LedgerKey::ContractData(LedgerKeyContractData {
        contract: ScAddress::Contract(ContractId(Hash(owner.0))),
        key: ScVal::ExecutableTag(tag_string),
        durability: ContractDataDurability::Persistent,
    });
    let key_xdr = key
        .to_xdr_base64(Limits::none())
        .map_err(|e| e.to_string())?;
    let entry_xdr = crate::rpc::get_ledger_entry_xdr(rpc_url, &key_xdr)?
        .ok_or_else(|| "tag entry not found on this network".to_string())?;
    let data = LedgerEntryData::from_xdr_base64(entry_xdr, Limits::none())
        .map_err(|e| e.to_string())?;
    match data {
        LedgerEntryData::ContractData(entry) => Ok(entry.val),
        _ => Err("entry is not contract data".to_string()),
    }
}

pub fn get_tag_hash(rpc_url: &str, owner_id: &str, tag: &str) -> Result<String, String> {
    match get_tag_value(rpc_url, owner_id, tag)? {
        ScVal::Bytes(bytes) => Ok(to_hex(bytes.0.as_slice())),
        other => Err(format!("tag holds an unexpected value: {other:?}")),
    }
}

pub struct MemberStatus {
    pub contract_id: String,
    pub in_fleet: bool,
    pub detail: String,
}

pub struct UpgradePlan {
    pub owner: String,
    pub tag: String,
    pub current_hash: String,
    pub new_hash: String,
    pub already_current: bool,
    pub members: Vec<MemberStatus>,
}

pub fn plan_upgrade(
    rpc_url: &str,
    owner_id: &str,
    tag: &str,
    new_hash: &str,
    members: &[String],
) -> Result<UpgradePlan, String> {
    let new_hash = new_hash.to_lowercase();
    if new_hash.len() != 64 || !new_hash.chars().all(|c| c.is_ascii_hexdigit()) {
        return Err("new hash must be 64 hex characters".to_string());
    }
    let current_hash = get_tag_hash(rpc_url, owner_id, tag)?;
    let mut statuses = Vec::new();
    for contract_id in members {
        let status = match get_contract_executable(rpc_url, contract_id) {
            Ok(ContractExecutable::ExternalRef(r))
                if owner_text(&r.executable_owner) == owner_id
                    && r.tag.to_utf8_string_lossy() == tag =>
            {
                MemberStatus {
                    contract_id: contract_id.clone(),
                    in_fleet: true,
                    detail: "points at this owner and tag".to_string(),
                }
            }
            Ok(other) => MemberStatus {
                contract_id: contract_id.clone(),
                in_fleet: false,
                detail: describe_executable(&other),
            },
            Err(e) => MemberStatus {
                contract_id: contract_id.clone(),
                in_fleet: false,
                detail: format!("error: {e}"),
            },
        };
        statuses.push(status);
    }
    Ok(UpgradePlan {
        owner: owner_id.to_string(),
        tag: tag.to_string(),
        already_current: current_hash == new_hash,
        current_hash,
        new_hash,
        members: statuses,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hex_formatting_is_lowercase_and_padded() {
        assert_eq!(to_hex(&[0x10, 0xc0, 0x09, 0xff]), "10c009ff");
    }

    #[test]
    fn describes_wasm_executable_with_full_hash() {
        let text = describe_executable(&ContractExecutable::Wasm(Hash([0xab; 32])));
        assert_eq!(text, format!("Wasm hash {}", "ab".repeat(32)));
    }

    #[test]
    fn describes_stellar_asset_executable() {
        let text = describe_executable(&ContractExecutable::StellarAsset);
        assert!(text.contains("Stellar asset"));
    }

    #[test]
    fn plan_rejects_a_short_hash_before_any_network_call() {
        let result = plan_upgrade(
            "http://127.0.0.1:1",
            "CCSC3UOXCCNB3VV7MM2P63IPNVKHQQ3TUMBCIW6OW42F75FBGMI4Y3VA",
            "fleetwatch-demo",
            "abc",
            &[],
        );
        assert!(result.is_err());
    }
}
