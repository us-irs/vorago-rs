#![no_std]

use arbitrary_int::u11;
use cobs::DestBufTooSmallError;
use spacepackets::{
    CcsdsPacketCreationError, CcsdsPacketCreatorWithReservedData, SpacePacketHeader,
};

pub const APID: u11 = u11::new(0x01);

pub const BOOTLOADER_START_ADDR: u32 = 0x0000_0000;
pub const BOOTLOADER_END_ADDR: u32 = 0x0000_3000;
pub const BOOTLOADER_CRC_ADDR: u32 = BOOTLOADER_END_ADDR - 2;
pub const BOOTLOADER_MAX_SIZE: u32 = BOOTLOADER_END_ADDR - BOOTLOADER_START_ADDR - 2;

pub const APP_A_START_ADDR: u32 = 0x0000_3000;
pub const APP_B_END_ADDR: u32 = 0x0002_0000 - 8;
pub const IMG_SLOT_SIZE: u32 = (APP_B_END_ADDR - APP_A_START_ADDR) / 2;

pub const APP_A_END_ADDR: u32 = APP_A_START_ADDR + IMG_SLOT_SIZE;
pub const APP_A_SIZE_ADDR: u32 = APP_A_END_ADDR - 8;
pub const APP_A_CRC_ADDR: u32 = APP_A_END_ADDR - 4;
pub const APP_A_MAX_SIZE: u32 = IMG_SLOT_SIZE - 8;

pub const APP_B_START_ADDR: u32 = APP_A_END_ADDR;
pub const APP_B_SIZE_ADDR: u32 = APP_B_END_ADDR - 8;
pub const APP_B_CRC_ADDR: u32 = APP_B_END_ADDR - 4;
pub const APP_B_MAX_SIZE: u32 = IMG_SLOT_SIZE - 8;

#[derive(
    Debug,
    Copy,
    Clone,
    PartialEq,
    Eq,
    num_enum::TryFromPrimitive,
    serde::Serialize,
    serde::Deserialize,
)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
pub enum AppSel {
    A = 0,
    B = 1,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Request {
    Ping,
    Corrupt(AppSel),
    WriteNvm { offset: u32 },
    SetBootSlot(AppSel),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Response {
    Ok,
}

pub fn create_tm_packet(
    buf: &mut [u8],
    sp_header: SpacePacketHeader,
    response: Response,
) -> Result<usize, CcsdsPacketCreationError> {
    let packet_data_size = postcard::experimental::serialized_size(&response).unwrap();
    let mut creator =
        CcsdsPacketCreatorWithReservedData::new_tm_with_checksum(sp_header, packet_data_size, buf)?;

    let current_index = 0;
    postcard::to_slice(&response, &mut creator.packet_data_mut()[current_index..]).unwrap();
    Ok(creator.finish())
}

#[derive(Debug, thiserror::Error)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PacketCreationAndEncodingError {
    #[error("packet creation failed: {0}")]
    Creation(#[from] CcsdsPacketCreationError),
    #[error("destination buffer too small: {0}")]
    Encoding(#[from] DestBufTooSmallError),
}

pub fn create_encoded_tm_packet(
    buf: &mut [u8],
    encoded_buf: &mut [u8],
    sp_header: SpacePacketHeader,
    response: Response,
) -> Result<usize, PacketCreationAndEncodingError> {
    let packet_len = create_tm_packet(buf, sp_header, response)?;
    let encoded_len =
        cobs::try_encode_including_sentinels(&buf[0..packet_len], &mut encoded_buf[..])?;
    Ok(encoded_len)
}

#[cfg(test)]
mod tests {}
