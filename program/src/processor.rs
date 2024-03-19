use {
    borsh::BorshDeserialize,
    num_traits::FromPrimitive,
    solana_program::{
        account_info::AccountInfo, entrypoint::ProgramResult, msg, program_error::ProgramError,
        pubkey::Pubkey,
    },
};

use crate::instruction::ProgramInstruction;

pub mod allocate_and_post_record;
pub mod allocate_record;
pub mod delete_record;
pub mod edit_record;
pub mod unverify_roa;
pub mod validate_ethereum_signature;
pub mod validate_solana_signature;
pub mod write_roa;

pub struct Processor {}

impl Processor {
    pub fn process_instruction(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        msg!("Beginning processing");
        let instruction = FromPrimitive::from_u8(instruction_data[0])
            .ok_or(ProgramError::InvalidInstructionData)?;
        let instruction_data = &instruction_data[1..];
        msg!("Instruction unpacked");

        match instruction {
            ProgramInstruction::AllocateRecord => {
                msg!("[+] Instruction: Allocate record");
                let params = allocate_record::Params::try_from_slice(instruction_data)?;
                allocate_record::process(program_id, accounts, params)?;
            }
            ProgramInstruction::AllocateAndPostRecord => {
                msg!("[+] Instruction: Allocate and post record");
                let params = allocate_and_post_record::Params::try_from_slice(instruction_data)?;
                allocate_and_post_record::process(program_id, accounts, params)?;
            }
            ProgramInstruction::EditRecord => {
                msg!("[+] Instruction: Edit record");
                let params = edit_record::Params::try_from_slice(instruction_data)?;
                edit_record::process(program_id, accounts, params)?;
            }
            ProgramInstruction::ValidateSolanaSignature => {
                msg!("[+] Instruction: Validate Solana signature");
                let params = validate_solana_signature::Params::try_from_slice(instruction_data)?;
                validate_solana_signature::process(program_id, accounts, params)?;
            }
            ProgramInstruction::ValidateEthereumSignature => {
                msg!("[+] Instruction: Validate Ethereum signature");
                let params = validate_ethereum_signature::Params::try_from_slice(instruction_data)?;
                validate_ethereum_signature::process(program_id, accounts, params)?;
            }
            ProgramInstruction::DeleteRecord => {
                msg!("[+] Instruction: Delete record");
                let params = delete_record::Params::try_from_slice(instruction_data)?;
                delete_record::process(program_id, accounts, params)?;
            }
            ProgramInstruction::WriteRoa => {
                msg!("[+] Instruction: Write RoA");
                let params = write_roa::Params::try_from_slice(instruction_data)?;
                write_roa::process(program_id, accounts, params)?;
            }
            ProgramInstruction::UnverifyRoa => {
                msg!("[+] Instruction: Unverify RoA");
                unverify_roa::process(program_id, accounts)?;
            }
        }

        Ok(())
    }
}
