use crate::i18n::Message;
use std::rc::Rc;
use super::slice::Slice;
use crate::rom::mapper::{find_mapper, Mapper};
use crate::rom::Timing;

const K: u32 = 1024;

enum Version {
    V1 = 1,
    V2 = 2
}

enum Console {
    Nes,
    VsSystem,
    PlayChoice10,
    Extended
}

#[allow(dead_code)]
pub struct Rom {
    prg_rom_size: u32,
    chr_rom_size: u32,
    mirroring: Option<u8>,
    extra_memory: bool,
    console: Console,
    version: Version,
    prg_ram_size: u32,
    prg_nv_ram_size: u32,
    timing: Timing,
    chr_ram_size: u32,
    chr_nv_ram_size: u32,
    vs_ppu_type: u8,
    vs_hardware: u8,
    ext_console_type: u8,
    misc_roms: u8,
    exp_device: u8,
    trainer: Option<Slice>,
    mapper: Box<dyn Mapper>
}

impl Rom {
    pub fn parse(data: Box<[u8]>) -> Result<Rom, Message> {
        let data = Rc::<[u8]>::from(data);
        if data.len() < 16 {
            return Err(Message::MalformedFileFormat)
        }
        if data[0] != 0x4E || data[1] != 0x45 || data[2] != 0x53 || data[3] != 0x1A {
            return Err(Message::MalformedFileFormat)
        }
        let mut prg_rom_size = data[4] as u32 * 16 * K;
        let mut chr_rom_size = data[5] as u32 * 8 * K;

        let mirroring = if data[6] & 0x08 != 0 {
            None
        } else {
            Some(data[6] & 0x01)
        };
        let extra_memory = data[6] & 0x02 != 0;
        let trainer_exists = data[6] & 0x04 != 0;

        let console = match data[7] & 0x03 {
            0 => Console::Nes,
            1 => Console::VsSystem,
            2 => Console::PlayChoice10,
            _ => Console::Extended
        };
        let version = if data[7] & 0x0C == 0x08 {
            Version::V2
        } else {
            Version::V1
        };

        let mut mapper_id = (((data[6] & 0xF0) >> 4) | (data[7] & 0xF0)) as u16;
        let mut sub_mapper: Option<u8> = None;
        let mut prg_ram_size: u32 = 0;
        let mut prg_nv_ram_size: u32 = 0;
        let mut timing = Timing::NTSC;
        let mut chr_ram_size: u32 = 0;
        let mut chr_nv_ram_size: u32 = 0;
        let mut vs_ppu_type: u8 = 0;
        let mut vs_hardware: u8 = 0;
        let mut ext_console_type: u8 = 0;
        let mut misc_roms: u8 = 0;
        let mut exp_device: u8 = 0;
        match version {
            Version::V1 => {
                prg_ram_size = data[8] as u32 * 8 * K;
                if data[9] & 0x01 > 0 {
                    timing = Timing::PAL;
                }
            },
            Version::V2 => {
                mapper_id |= (data[8] as u16 & 0x0F) << 8;
                sub_mapper = Some((data[8] & 0xF0) >> 4);

                let r_nibble: u32 = data[9] as u32 & 0x0F;
                if r_nibble != 0x0F {
                    prg_rom_size += (r_nibble << 8) * 16 * K;
                } else {
                    prg_rom_size = ((data[4] as u32 & 0x03) * 2 + 1) * ((data[4] as u32 & 0xFC) >> 2);
                }

                let c_nibble: u32 = data[9]  as u32 & 0xF0;
                if c_nibble != 0xF0 {
                    chr_rom_size += K * 8 * (c_nibble << 4);
                } else {
                    chr_rom_size = ((data[5] as u32 & 0x03) * 2 + 1) * ((data[5] as u32 & 0xFC) >> 2);
                }

                let p_shift = data[10] & 0x0F;
                if p_shift > 0 {
                    prg_ram_size = 64 << p_shift;
                }
                let pn_shift = (data[10] & 0xF0) >> 4;
                if pn_shift > 0 {
                    prg_nv_ram_size = 64 << pn_shift;
                }

                let c_shift = data[11] & 0x0F;
                if c_shift > 0 {
                    chr_ram_size = 64 << c_shift;
                }
                let cn_shift = (data[11] & 0xF0) >> 4;
                if cn_shift > 0 {
                    chr_nv_ram_size = 64 << cn_shift;
                }

                timing = match data[12] & 0x03 {
                    0 => Timing::NTSC,
                    1 => Timing::PAL,
                    2 => Timing::MultipleRegion,
                    _ => Timing::Dendy
                };

                match console {
                    Console::VsSystem => {
                        vs_ppu_type = data[13] & 0x0F;
                        vs_hardware = (data[13] & 0xF0) >> 4;
                    },
                    Console::Extended => {
                        ext_console_type = data[13] & 0x0F;
                    },
                    _ => ()
                };

                misc_roms = data[14] & 0x03;
                exp_device = data[15] & 0x3F;
            }
        };

        let mut start: u32 = 16;
        let trainer = if trainer_exists {
            let s = Some(Slice::new(&data, start, 512));
            start += 512;
            s
        } else {
            None
        };

        let prg_rom = Slice::new(&data, start, prg_rom_size);
        start += prg_rom_size;

        let chr_rom = if chr_rom_size > 0 {
            Some(Slice::new(&data, start, prg_rom_size))
        } else {
            None
        };

        let mapper = find_mapper(mapper_id, sub_mapper, prg_rom, chr_rom).unwrap();

        Ok(Rom {
            prg_rom_size,
            chr_rom_size,
            mirroring,
            extra_memory,
            console,
            version,
            prg_ram_size,
            prg_nv_ram_size,
            timing,
            chr_ram_size,
            chr_nv_ram_size,
            vs_ppu_type,
            vs_hardware,
            ext_console_type,
            misc_roms,
            exp_device,
            trainer,
            mapper
        })
    }

    pub fn mapper(&self) -> &dyn Mapper {
        self.mapper.as_ref()
    }

    pub fn mapper_mut(&mut self) -> &mut dyn Mapper {
        self.mapper.as_mut()
    }

    pub fn timing(&self) -> &Timing {
        &self.timing
    }

    pub fn mirroring(&self) -> Option<u8> {
        self.mirroring
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    pub fn mock() -> Rom {
        Rom {
            prg_rom_size: 0,
            chr_rom_size: 0,
            mirroring: Some(0),
            extra_memory: false,
            console: Console::Nes,
            version: Version::V1,
            prg_ram_size: 0,
            prg_nv_ram_size: 0,
            timing: Timing::NTSC,
            chr_ram_size: 0,
            chr_nv_ram_size: 0,
            vs_ppu_type: 0,
            vs_hardware: 0,
            ext_console_type: 0,
            misc_roms: 0,
            exp_device: 0,
            trainer: None,
            mapper: crate::rom::mapper::tests::mock()
        }
    }
}