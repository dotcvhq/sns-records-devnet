use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program::invoke_signed,
    program_pack::Pack, rent::Rent, sysvar::Sysvar,
};
use spl_name_service::state::NameRecordHeader;

#[allow(clippy::too_many_arguments)]
pub fn allocate_record<'a>(
    space: u32,
    hashed_name: &[u8],
    record: &AccountInfo<'a>,
    payer: &AccountInfo<'a>,
    domain: &AccountInfo<'a>,
    domain_owner: &AccountInfo<'a>,
    central_state: &AccountInfo<'a>,
    system_program: &AccountInfo<'a>,
) -> ProgramResult {
    let lamports = Rent::get()
        .unwrap()
        .minimum_balance(space as usize + NameRecordHeader::LEN);

    let ix = spl_name_service::instruction::create(
        spl_name_service::ID,
        spl_name_service::instruction::NameRegistryInstruction::Create {
            hashed_name: hashed_name.to_vec(),
            lamports,
            space,
        },
        *record.key,
        *payer.key,
        *central_state.key,
        Some(*central_state.key),
        Some(*domain.key),
        Some(*domain_owner.key),
    )?;

    invoke_signed(
        &ix,
        &[
            system_program.clone(),
            payer.clone(),
            record.clone(),
            domain_owner.clone(),
            central_state.clone(),
            domain.clone(),
            domain_owner.clone(),
        ],
        &[&crate::central_state::SIGNER_SEEDS],
    )
}

pub fn edit_record<'a>(
    data: &[u8],
    offset: u32,
    record: &AccountInfo<'a>,
    central_state: &AccountInfo<'a>,
) -> ProgramResult {
    let ix = spl_name_service::instruction::update(
        spl_name_service::ID,
        offset,
        data.to_vec(),
        *record.key,
        crate::central_state::KEY,
        None,
    )?;

    invoke_signed(
        &ix,
        &[record.clone(), central_state.clone()],
        &[&crate::central_state::SIGNER_SEEDS],
    )
}

pub fn delete_record<'a>(
    record: &AccountInfo<'a>,
    record_owner: &AccountInfo<'a>,
    refund_target: &AccountInfo<'a>,
) -> ProgramResult {
    let ix = spl_name_service::instruction::delete(
        spl_name_service::ID,
        *record.key,
        *record_owner.key,
        *refund_target.key,
    )?;

    invoke_signed(
        &ix,
        &[record.clone(), record_owner.clone(), refund_target.clone()],
        &[&crate::central_state::SIGNER_SEEDS],
    )
}

pub fn resize_record<'a>(
    record: &AccountInfo<'a>,
    record_owner: &AccountInfo<'a>,
    payer: &AccountInfo<'a>,
    system_program: &AccountInfo<'a>,
    new_size: u32,
) -> ProgramResult {
    let ix = spl_name_service::instruction::realloc(
        spl_name_service::ID,
        *payer.key,
        *record.key,
        *record_owner.key,
        new_size,
    )?;

    invoke_signed(
        &ix,
        &[
            system_program.clone(),
            payer.clone(),
            record.clone(),
            record_owner.clone(),
        ],
        &[&crate::central_state::SIGNER_SEEDS],
    )
}
