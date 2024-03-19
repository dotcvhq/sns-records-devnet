use solana_program::program_pack::Pack;
use spl_name_service::state::NameRecordHeader;

use super::validation::Validation;

use bytemuck::{Pod, Zeroable};

#[derive(Clone, Copy, Zeroable, Pod, Debug)]
#[allow(missing_docs)]
#[repr(C)]
pub struct RecordHeader {
    pub staleness_validation: u16,
    pub right_of_association_validation: u16,
    pub content_length: u32,
}

impl RecordHeader {
    pub const LEN: usize = std::mem::size_of::<Self>();

    pub fn from_buffer(buffer: &[u8]) -> Self {
        let offset = NameRecordHeader::LEN;
        let (_, data) = buffer.split_at(offset);
        *bytemuck::from_bytes::<Self>(&data[..Self::LEN])
    }

    pub fn new(content_length: u32) -> Self {
        Self {
            staleness_validation: Validation::None as u16,
            right_of_association_validation: Validation::None as u16,
            content_length,
        }
    }
}
