use solana_program::{account_info::AccountInfo, hash::hashv, program_pack::Pack};
use spl_name_service::state::NameRecordHeader;
use spl_name_service::state::HASH_PREFIX;

use crate::error::SnsRecordsError;

use {
    solana_program::pubkey, solana_program::pubkey::Pubkey,
    spl_name_service::state::get_seeds_and_key,
};

pub const ROOT_DOMAIN: Pubkey = pubkey!("58PwtjSDuFHuUkYjH9BYnnQKHfwo9reZhC2zMJv9JPkx");

pub fn get_hashed_name(record: &str) -> Vec<u8> {
    hashv(&[(HASH_PREFIX.to_owned() + record).as_bytes()])
        .as_ref()
        .to_vec()
}

pub fn get_record_key_and_seeds(domain: &Pubkey, record: &str) -> (Pubkey, Vec<u8>) {
    let hashed = get_hashed_name(record);
    get_seeds_and_key(
        &spl_name_service::ID,
        hashed,
        Some(&crate::central_state::KEY),
        Some(domain),
    )
}

pub fn check_domain_owner(
    account: &AccountInfo,
    expected_owner: &Pubkey,
) -> Result<(), SnsRecordsError> {
    let hd = NameRecordHeader::unpack_from_slice(&account.data.borrow())
        .map_err(|_| SnsRecordsError::DataTypeMismatch)?;

    if hd.owner != *expected_owner {
        return Err(SnsRecordsError::WrongDomainOwner);
    }

    Ok(())
}

pub fn check_domain_parent(
    account: &AccountInfo,
    expected_parent: &Pubkey,
) -> Result<(), SnsRecordsError> {
    let hd = NameRecordHeader::unpack_from_slice(&account.data.borrow())
        .map_err(|_| SnsRecordsError::DataTypeMismatch)?;

    if hd.class != crate::central_state::KEY {
        return Err(SnsRecordsError::WrongClass);
    }

    if hd.parent_name != *expected_parent {
        return Err(SnsRecordsError::WrongParent);
    }

    Ok(())
}
