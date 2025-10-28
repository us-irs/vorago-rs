use arbitrary_int::{prelude::*, u11, u15, u3, u4};
use embedded_can::Frame;

use super::{
    regs::{
        self, BaseId, BufStatusAndControl, BufferState, ExtendedId, MmioCanMessageBuffer,
        TwoBytesData,
    },
    CanFrame, CanFrameNormal, CanFrameRtr, CanId, InvalidBufferIndexError,
};

pub struct CanChannelLowLevel {
    id: CanId,
    /// Message buffer index.
    idx: usize,
    msg_buf: MmioCanMessageBuffer<'static>,
}

impl core::fmt::Debug for CanChannelLowLevel {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("CanChannel")
            .field("can_id", &self.id)
            .field("idx", &self.idx)
            .finish()
    }
}

impl CanChannelLowLevel {
    /// Steal a low level instance of a CAN channel.
    ///
    /// # Safety
    ///
    /// Circumvents ownership and safety guarantees of the HAL.
    #[inline]
    pub unsafe fn steal(can: CanId, idx: usize) -> Result<Self, InvalidBufferIndexError> {
        if idx > 14 {
            return Err(InvalidBufferIndexError(idx));
        }
        let msg_buf = unsafe { can.steal_regs().steal_cmbs_unchecked(idx) };
        Ok(Self {
            id: can,
            idx,
            msg_buf,
        })
    }

    /// Steal a low level instance of a CAN channel without and index checks.
    ///
    /// # Safety
    ///
    /// Does not perform any bound checks. Passing an invalid index of 15 or higher leads to
    /// undefined behaviour.
    #[inline]
    pub const unsafe fn steal_unchecked(can: CanId, idx: usize) -> Self {
        if idx > 14 {
            panic!("invalid buffer index for CAN low level channel");
        }
        let msg_buf = unsafe { can.steal_regs().steal_cmbs_unchecked(idx) };
        Self {
            id: can,
            idx,
            msg_buf,
        }
    }

    /// # Safety
    ///
    /// Allows to create an aribtrary amoutn of driver handles to the same message block, which
    /// might lead to data races on invalid usage.
    #[inline]
    pub const unsafe fn clone(&self) -> Self {
        Self {
            id: self.id,
            idx: self.idx,
            msg_buf: unsafe { self.msg_buf.clone() },
        }
    }

    pub fn reset(&mut self) {
        self.msg_buf.reset();
    }

    #[inline]
    pub fn read_state(&self) -> Result<BufferState, u8> {
        self.msg_buf.read_stat_ctrl().state()
    }

    #[inline]
    pub fn write_state(&mut self, buffer_state: BufferState) {
        self.msg_buf.modify_stat_ctrl(|mut val| {
            val.set_state(buffer_state);
            val
        });
    }

    pub fn configure_for_transmission(&mut self, tx_priority: Option<u4>) {
        self.msg_buf.modify_stat_ctrl(|mut val| {
            val.set_dlc(u4::new(0));
            if let Some(tx_priority) = tx_priority {
                val.set_priority(tx_priority);
            }
            val.set_state(BufferState::TxNotActive);
            val
        });
    }

    pub fn set_standard_id(&mut self, standard_id: embedded_can::StandardId, set_rtr: bool) {
        let mut id1_reg = standard_id.as_raw() << 5;
        if set_rtr {
            id1_reg |= 1 << 4;
        }
        self.msg_buf
            .write_id1(BaseId::new_with_raw_value(id1_reg as u32));
    }

    pub fn set_extended_id(&mut self, extended_id: embedded_can::ExtendedId, set_rtr: bool) {
        let id_raw = extended_id.as_raw();
        let id1_reg = (((id_raw >> 18) & 0x7FF) << 4) as u16 | ((id_raw >> 15) & 0b111) as u16;
        self.msg_buf
            .write_id1(BaseId::new_with_raw_value(id1_reg as u32));
        let id0_reg = ((id_raw & 0x7FFF) << 1) as u16 | set_rtr as u16;
        self.msg_buf
            .write_id0(ExtendedId::new_with_raw_value(id0_reg as u32));
    }

    pub fn configure_for_reception(&mut self) {
        self.msg_buf.write_stat_ctrl(
            BufStatusAndControl::builder()
                .with_dlc(u4::new(0))
                .with_priority(u4::new(0))
                .with_state(BufferState::RxReady)
                .build(),
        );
    }

    pub fn transmit_frame_unchecked(&mut self, frame: CanFrame) {
        let is_remote = frame.is_remote_frame();
        self.write_id(frame.id(), is_remote);
        let dlc = frame.dlc();
        self.msg_buf.modify_stat_ctrl(|mut ctrl| {
            ctrl.set_dlc(u4::new(dlc as u8));
            ctrl
        });
        if !is_remote {
            self.msg_buf
                .write_data0(TwoBytesData::new_with_raw_value(0));
            self.msg_buf
                .write_data1(TwoBytesData::new_with_raw_value(0));
            self.msg_buf
                .write_data2(TwoBytesData::new_with_raw_value(0));
            self.msg_buf
                .write_data3(TwoBytesData::new_with_raw_value(0));
            for idx in 0..dlc {
                match idx {
                    0 => self.msg_buf.modify_data0(|mut val| {
                        val.set_data_upper_byte(frame.data()[idx]);
                        val
                    }),
                    1 => self.msg_buf.modify_data0(|mut val| {
                        val.set_data_lower_byte(frame.data()[idx]);
                        val
                    }),
                    2 => self.msg_buf.modify_data1(|mut val| {
                        val.set_data_upper_byte(frame.data()[idx]);
                        val
                    }),
                    3 => self.msg_buf.modify_data1(|mut val| {
                        val.set_data_lower_byte(frame.data()[idx]);
                        val
                    }),
                    4 => self.msg_buf.modify_data2(|mut val| {
                        val.set_data_upper_byte(frame.data()[idx]);
                        val
                    }),
                    5 => self.msg_buf.modify_data2(|mut val| {
                        val.set_data_lower_byte(frame.data()[idx]);
                        val
                    }),
                    6 => self.msg_buf.modify_data3(|mut val| {
                        val.set_data_upper_byte(frame.data()[idx]);
                        val
                    }),
                    7 => self.msg_buf.modify_data3(|mut val| {
                        val.set_data_lower_byte(frame.data()[idx]);
                        val
                    }),
                    _ => unreachable!(),
                }
            }
        }
        self.write_state(BufferState::TxOnce);
    }

    #[inline]
    pub fn clear_interrupt(&mut self) {
        let mut regs = unsafe { self.id.steal_regs() };
        let mut clear = regs::InterruptClear::new_with_raw_value(0);
        clear.set_buffer(self.idx, true);
        regs.write_iclr(clear);
    }

    pub fn enable_error_interrupt(&mut self, enable_translation: bool) {
        let mut regs = unsafe { self.id.steal_regs() };
        if enable_translation {
            regs.modify_icen(|mut val| {
                val.set_error(true);
                val
            });
        }
        regs.modify_ien(|mut val| {
            val.set_error(true);
            val
        });
    }

    pub fn enable_interrupt(&mut self, enable_translation: bool) {
        let mut regs = unsafe { self.id.steal_regs() };
        if enable_translation {
            regs.modify_icen(|mut val| {
                val.set_buffer(self.idx, true);
                val
            });
        }
        regs.modify_ien(|mut val| {
            val.set_buffer(self.idx, true);
            val
        });
    }

    fn write_id(&mut self, id: embedded_can::Id, is_remote: bool) {
        match id {
            embedded_can::Id::Standard(standard_id) => {
                self.msg_buf.write_id1(
                    BaseId::builder()
                        .with_mask_28_18(u11::new(standard_id.as_raw()))
                        .with_rtr_or_srr(is_remote)
                        .with_ide(false)
                        .with_mask_17_15(u3::new(0))
                        .build(),
                );
                self.msg_buf.write_id0(ExtendedId::new_with_raw_value(0));
            }
            embedded_can::Id::Extended(extended_id) => {
                let id_raw = extended_id.as_raw();
                self.msg_buf.write_id1(
                    BaseId::builder()
                        .with_mask_28_18(u11::new(((id_raw >> 18) & 0x7FF) as u16))
                        .with_rtr_or_srr(true)
                        .with_ide(true)
                        .with_mask_17_15(u3::new(((id_raw >> 15) & 0b111) as u8))
                        .build(),
                );
                self.msg_buf.write_id0(
                    ExtendedId::builder()
                        .with_mask_14_0(u15::new((id_raw & 0x7FFF) as u16))
                        .with_xrtr(is_remote)
                        .build(),
                );
            }
        }
    }

    /// Reads a received CAN frame from the message buffer.
    ///
    /// This function does not check whether the pre-requisites for reading a CAN frame were
    /// met and assumes this was already checked by the user.
    pub fn read_frame_unchecked(&self) -> CanFrame {
        let id0 = self.msg_buf.read_id0();
        let id1 = self.msg_buf.read_id1();
        let data0 = self.msg_buf.read_data0();
        let data1 = self.msg_buf.read_data1();
        let data2 = self.msg_buf.read_data2();
        let data3 = self.msg_buf.read_data3();
        let mut data: [u8; 8] = [0; 8];
        let mut read_data = |dlc: u4| {
            (0..dlc.as_usize()).for_each(|i| match i {
                0 => data[i] = data0.data_upper_byte().as_u8(),
                1 => data[i] = data0.data_lower_byte().as_u8(),
                2 => data[i] = data1.data_upper_byte().as_u8(),
                3 => data[i] = data1.data_lower_byte().as_u8(),
                4 => data[i] = data2.data_upper_byte().as_u8(),
                5 => data[i] = data2.data_lower_byte().as_u8(),
                6 => data[i] = data3.data_upper_byte().as_u8(),
                7 => data[i] = data3.data_lower_byte().as_u8(),
                _ => unreachable!(),
            });
        };
        let (id, rtr) = if !id1.ide() {
            let id = embedded_can::Id::Standard(
                embedded_can::StandardId::new(id1.mask_28_18().as_u16()).unwrap(),
            );
            if id1.rtr_or_srr() {
                (id, true)
            } else {
                (id, false)
            }
        } else {
            let id_raw = (id1.mask_28_18().as_u32() << 18)
                | (id1.mask_17_15().as_u32() << 15)
                | id0.mask_14_0().as_u32();
            let id = embedded_can::Id::Extended(embedded_can::ExtendedId::new(id_raw).unwrap());
            if id0.xrtr() {
                (id, true)
            } else {
                (id, false)
            }
        };
        if rtr {
            CanFrameRtr::new(id, self.msg_buf.read_stat_ctrl().dlc().as_usize()).into()
        } else {
            let dlc = self.msg_buf.read_stat_ctrl().dlc();
            read_data(dlc);
            CanFrameNormal::new(id, &data[0..dlc.as_usize()])
                .unwrap()
                .into()
        }
    }
}
