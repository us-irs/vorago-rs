use std::path::Path;

use anyhow::Context;
use crc::{CRC_16_IBM_3740, Crc};
use spacepackets::CcsdsPacketReader;
use tmtc_utils::transport::serial::PacketTransportSerialCobs;

use crate::{
    Target,
    elf::{check_total_size, log_segments_info, parse_elf},
    tc,
};
use models::{
    APP_A_CRC_ADDR, APP_A_SIZE_ADDR, APP_B_CRC_ADDR, APP_B_SIZE_ADDR, BOOTLOADER_CRC_ADDR,
};
use models::{Request, Response};

pub const CHUNK_SIZE: usize = 256;

fn send_and_await_ok(
    transport: &mut PacketTransportSerialCobs,
    request: &Request,
    payload: &[u8],
    timeout: std::time::Duration,
) -> anyhow::Result<()> {
    let tc = tc::create_tc_with_additional_payload(request, payload);
    log::debug!(
        "TX TC {:#010x}: {request:?} + {} payload bytes",
        tc.ccsds_packet_id_and_psc().raw(),
        payload.len()
    );
    transport.send(&tc.to_vec())?;

    let deadline = std::time::Instant::now() + timeout;
    let mut got_ok = false;

    while !got_ok {
        if std::time::Instant::now() > deadline {
            anyhow::bail!("timeout waiting for Ok response to {request:?}");
        }
        transport.receive(|packet| {
            let Ok(reader) = CcsdsPacketReader::new_with_checksum(packet) else {
                return;
            };
            match postcard::take_from_bytes::<Response>(reader.packet_data()) {
                Ok((Response::Ok, _)) => got_ok = true,
                #[allow(unreachable_patterns)]
                Ok((other, _)) => log::warn!("unexpected response: {other:?}"),
                Err(e) => log::error!("failed to deserialise response: {e}"),
            }
        })?;
    }

    Ok(())
}

pub fn flash_image(
    transport: &mut PacketTransportSerialCobs,
    target: &Target,
    path: &Path,
) -> anyhow::Result<()> {
    let segments = parse_elf(target, path)
        .with_context(|| format!("failed to parse ELF: {}", path.display()))?;

    let total_size: usize = segments.iter().map(|s| s.data.len()).sum();
    check_total_size(target, total_size)?;
    log_segments_info(target, &segments, total_size, path);

    let ack_timeout = std::time::Duration::from_secs(2);

    // 1. Write all segments in chunks, waiting for Ok after each.
    for seg in &segments {
        let mut pos = 0usize;
        while pos < seg.data.len() {
            let chunk_len = CHUNK_SIZE.min(seg.data.len() - pos);
            let offset = seg.offset + pos as u32;
            let chunk = &seg.data[pos..pos + chunk_len];

            log::info!(
                "Writing {chunk_len} bytes @ {offset:#010x}  ({}/{} of '{}')",
                pos + chunk_len,
                seg.data.len(),
                seg.name,
            );

            send_and_await_ok(transport, &Request::WriteNvm { offset }, chunk, ack_timeout)?;
            pos += chunk_len;
        }
    }

    // 2. CRC + size postprocessing.
    match target {
        Target::Bl => {
            log::info!("Blanking bootloader CRC @ {BOOTLOADER_CRC_ADDR:#010x}");
            send_and_await_ok(
                transport,
                &Request::WriteNvm {
                    offset: BOOTLOADER_CRC_ADDR,
                },
                &[0x00, 0x00],
                ack_timeout,
            )?;
        }
        Target::A | Target::B => {
            let (size_addr, crc_addr) = match target {
                Target::A => (APP_A_SIZE_ADDR, APP_A_CRC_ADDR),
                Target::B => (APP_B_SIZE_ADDR, APP_B_CRC_ADDR),
                Target::Bl => unreachable!(),
            };

            log::info!("Writing app size {total_size} @ {size_addr:#010x}");
            send_and_await_ok(
                transport,
                &Request::WriteNvm { offset: size_addr },
                &(total_size as u32).to_be_bytes(),
                ack_timeout,
            )?;

            let crc = Crc::<u16>::new(&CRC_16_IBM_3740);
            let mut digest = crc.digest();
            for seg in &segments {
                digest.update(&seg.data);
            }
            let checksum = digest.finalize().to_be_bytes();

            log::info!(
                "Writing CRC [{:#04x}, {:#04x}] @ {crc_addr:#010x}",
                checksum[0],
                checksum[1]
            );
            send_and_await_ok(
                transport,
                &Request::WriteNvm { offset: crc_addr },
                &checksum,
                ack_timeout,
            )?;
        }
    }

    log::info!("Flash complete.");
    Ok(())
}
