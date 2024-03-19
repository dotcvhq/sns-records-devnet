use crate::{error::SnsRecordsError, processor::Processor};

use {
    num_traits::FromPrimitive,
    solana_program::{
        account_info::AccountInfo, decode_error::DecodeError, entrypoint::ProgramResult, msg,
        program_error::PrintProgramError, pubkey::Pubkey,
    },
};

#[cfg(not(feature = "no-entrypoint"))]
use solana_program::entrypoint;
#[cfg(not(feature = "no-entrypoint"))]
entrypoint!(process_instruction);

/// The entrypoint to the program
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!("Entrypoint");
    if let Err(error) = Processor::process_instruction(program_id, accounts, instruction_data) {
        // catch the error so we can print it
        error.print::<SnsRecordsError>();
        return Err(error);
    }
    Ok(())
}

impl PrintProgramError for SnsRecordsError {
    fn print<E>(&self)
    where
        E: 'static + std::error::Error + DecodeError<E> + PrintProgramError + FromPrimitive,
    {
        match self {
            SnsRecordsError::AlreadyInitialized => {
                msg!("Error: This account is already initialized")
            }
            SnsRecordsError::DataTypeMismatch => msg!("Error: Data type mismatch"),
            SnsRecordsError::WrongOwner => msg!("Error: Wrong account owner"),
            SnsRecordsError::Uninitialized => msg!("Error: Account is uninitialized"),
            SnsRecordsError::UnsupportedValidation => msg!("Error: Unsupported validation"),
            SnsRecordsError::Secp256k1Recover => msg!("Error: Could not recover public key"),
            SnsRecordsError::EthPubkeyMismatch => msg!("Error: ETH public key mismatch"),
            SnsRecordsError::WrongDomainOwner => msg!("Error: Wrong domain owner"),
            SnsRecordsError::NumericalOverflow => msg!("Error: Numerical overflow"),
            SnsRecordsError::OutOfBound => msg!("Error: Array out of bound"),
            SnsRecordsError::InvalidVerifier => msg!("Error: Invalid verifier"),
            SnsRecordsError::WrongParent => msg!("Error: Wrong parent owner"),
            SnsRecordsError::WrongClass => msg!("Error: Wrong class"),
        }
    }
}
