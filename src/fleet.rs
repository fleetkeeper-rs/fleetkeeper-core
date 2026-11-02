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
