use std::path::Path;

use models::{
    APP_A_START_ADDR, APP_A_MAX_SIZE, APP_B_START_ADDR, APP_B_MAX_SIZE,
    BOOTLOADER_START_ADDR, BOOTLOADER_MAX_SIZE,
};

use crate::Target;

pub struct LoadableSegment {
    pub name:   String,
    pub offset: u32,
    pub data:   Vec<u8>,
}

pub fn parse_elf(target: &Target, path: &Path) -> anyhow::Result<Vec<LoadableSegment>> {
    use object::{Object, ObjectSection, ObjectSegment};

    let raw = std::fs::read(path)?;
    let obj = object::File::parse(raw.as_slice())?;

    let expected_base: u32 = match target {
        Target::Bl => BOOTLOADER_START_ADDR,
        Target::A  => APP_A_START_ADDR,
        Target::B  => APP_B_START_ADDR,
    };

    let mut segments = Vec::new();

    for (idx, segment) in obj.segments().enumerate() {
        let data = segment.data()?;
        if data.is_empty() {
            continue;
        }

        let addr = segment.address() as u32;

        if idx == 0 && addr != expected_base {
            anyhow::bail!(
                "unexpected base address {addr:#010x} for {target:?}, expected {expected_base:#010x}"
            );
        }

        let name = obj
            .sections()
            .find(|s| {
                let s_addr = s.address() as u32;
                s_addr >= addr
                    && s_addr < addr + data.len() as u32
                    && !s.name().unwrap_or("").is_empty()
            })
            .and_then(|s| s.name().ok().map(str::to_owned))
            .unwrap_or_else(|| format!("<segment {idx}>"));

        segments.push(LoadableSegment {
            name,
            offset: addr,
            data: data.to_vec(),
        });
    }

    Ok(segments)
}

pub fn check_total_size(target: &Target, total_size: usize) -> anyhow::Result<()> {
    let max = match target {
        Target::Bl => BOOTLOADER_MAX_SIZE,
        Target::A  => APP_A_MAX_SIZE,
        Target::B  => APP_B_MAX_SIZE,
    } as usize;

    if total_size > max {
        anyhow::bail!(
            "{target:?} image is {total_size} bytes, exceeds maximum of {max} bytes"
        );
    }
    Ok(())
}

pub fn log_segments_info(target: &Target, segments: &[LoadableSegment], total_size: usize, path: &Path) {
    log::info!(
        "Flashing {target:?} with image '{}' — {} segment(s), {total_size} bytes total",
        path.display(),
        segments.len()
    );
    for (i, seg) in segments.iter().enumerate() {
        log::info!(
            "  segment {i}: '{}' @ {:#010x}, {} bytes",
            seg.name, seg.offset, seg.data.len()
        );
    }
}
