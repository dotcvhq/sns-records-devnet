use std::convert::TryFrom;

use {
    bonfida_utils::BorshSize,
    borsh::{BorshDeserialize, BorshSerialize},
};

#[derive(BorshDeserialize, BorshSerialize, BorshSize, Clone, Copy, Debug)]
#[repr(u16)]
pub enum Validation {
    None,
    Solana,
    Ethereum,
    UnverifiedSolana,
    XChain,
}

impl TryFrom<u16> for Validation {
    type Error = crate::error::SnsRecordsError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Validation::None),
            1 => Ok(Validation::Solana),
            2 => Ok(Validation::Ethereum),
            3 => Ok(Validation::UnverifiedSolana),
            4 => Ok(Validation::XChain),
            _ => Err(crate::error::SnsRecordsError::DataTypeMismatch),
        }
    }
}

pub fn get_validation_length(validation: Validation) -> u32 {
    match validation {
        Validation::None => 0,
        Validation::Ethereum => 20,
        Validation::Solana => 32,
        Validation::UnverifiedSolana => 32,
        Validation::XChain => 34,
    }
}
