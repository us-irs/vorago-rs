#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cnstat_cmb0: CnstatCmb0,
    tstp_cmb0: TstpCmb0,
    data3_cmb0: Data3Cmb0,
    data2_cmb0: Data2Cmb0,
    data1_cmb0: Data1Cmb0,
    data0_cmb0: Data0Cmb0,
    id0_cmb0: Id0Cmb0,
    id1_cmb0: Id1Cmb0,
    cnstat_cmb1: CnstatCmb1,
    tstp_cmb1: TstpCmb1,
    data3_cmb1: Data3Cmb1,
    data2_cmb1: Data2Cmb1,
    data1_cmb1: Data1Cmb1,
    data0_cmb1: Data0Cmb1,
    id0_cmb1: Id0Cmb1,
    id1_cmb1: Id1Cmb1,
    cnstat_cmb2: CnstatCmb2,
    tstp_cmb2: TstpCmb2,
    data3_cmb2: Data3Cmb2,
    data2_cmb2: Data2Cmb2,
    data1_cmb2: Data1Cmb2,
    data0_cmb2: Data0Cmb2,
    id0_cmb2: Id0Cmb2,
    id1_cmb2: Id1Cmb2,
    cnstat_cmb3: CnstatCmb3,
    tstp_cmb3: TstpCmb3,
    data3_cmb3: Data3Cmb3,
    data2_cmb3: Data2Cmb3,
    data1_cmb3: Data1Cmb3,
    data0_cmb3: Data0Cmb3,
    id0_cmb3: Id0Cmb3,
    id1_cmb3: Id1Cmb3,
    cnstat_cmb4: CnstatCmb4,
    tstp_cmb4: TstpCmb4,
    data3_cmb4: Data3Cmb4,
    data2_cmb4: Data2Cmb4,
    data1_cmb4: Data1Cmb4,
    data0_cmb4: Data0Cmb4,
    id0_cmb4: Id0Cmb4,
    id1_cmb4: Id1Cmb4,
    cnstat_cmb5: CnstatCmb5,
    tstp_cmb5: TstpCmb5,
    data3_cmb5: Data3Cmb5,
    data2_cmb5: Data2Cmb5,
    data1_cmb5: Data1Cmb5,
    data0_cmb5: Data0Cmb5,
    id0_cmb5: Id0Cmb5,
    id1_cmb5: Id1Cmb5,
    cnstat_cmb6: CnstatCmb6,
    tstp_cmb6: TstpCmb6,
    data3_cmb6: Data3Cmb6,
    data2_cmb6: Data2Cmb6,
    data1_cmb6: Data1Cmb6,
    data0_cmb6: Data0Cmb6,
    id0_cmb6: Id0Cmb6,
    id1_cmb6: Id1Cmb6,
    cnstat_cmb7: CnstatCmb7,
    tstp_cmb7: TstpCmb7,
    data3_cmb7: Data3Cmb7,
    data2_cmb7: Data2Cmb7,
    data1_cmb7: Data1Cmb7,
    data0_cmb7: Data0Cmb7,
    id0_cmb7: Id0Cmb7,
    id1_cmb7: Id1Cmb7,
    cnstat_cmb8: CnstatCmb8,
    tstp_cmb8: TstpCmb8,
    data3_cmb8: Data3Cmb8,
    data2_cmb8: Data2Cmb8,
    data1_cmb8: Data1Cmb8,
    data0_cmb8: Data0Cmb8,
    id0_cmb8: Id0Cmb8,
    id1_cmb8: Id1Cmb8,
    cnstat_cmb9: CnstatCmb9,
    tstp_cmb9: TstpCmb9,
    data3_cmb9: Data3Cmb9,
    data2_cmb9: Data2Cmb9,
    data1_cmb9: Data1Cmb9,
    data0_cmb9: Data0Cmb9,
    id0_cmb9: Id0Cmb9,
    id1_cmb9: Id1Cmb9,
    cnstat_cmb10: CnstatCmb10,
    tstp_cmb10: TstpCmb10,
    data3_cmb10: Data3Cmb10,
    data2_cmb10: Data2Cmb10,
    data1_cmb10: Data1Cmb10,
    data0_cmb10: Data0Cmb10,
    id0_cmb10: Id0Cmb10,
    id1_cmb10: Id1Cmb10,
    cnstat_cmb11: CnstatCmb11,
    tstp_cmb11: TstpCmb11,
    data3_cmb11: Data3Cmb11,
    data2_cmb11: Data2Cmb11,
    data1_cmb11: Data1Cmb11,
    data0_cmb11: Data0Cmb11,
    id0_cmb11: Id0Cmb11,
    id1_cmb11: Id1Cmb11,
    cnstat_cmb12: CnstatCmb12,
    tstp_cmb12: TstpCmb12,
    data3_cmb12: Data3Cmb12,
    data2_cmb12: Data2Cmb12,
    data1_cmb12: Data1Cmb12,
    data0_cmb12: Data0Cmb12,
    id0_cmb12: Id0Cmb12,
    id1_cmb12: Id1Cmb12,
    cnstat_cmb13: CnstatCmb13,
    tstp_cmb13: TstpCmb13,
    data3_cmb13: Data3Cmb13,
    data2_cmb13: Data2Cmb13,
    data1_cmb13: Data1Cmb13,
    data0_cmb13: Data0Cmb13,
    id0_cmb13: Id0Cmb13,
    id1_cmb13: Id1Cmb13,
    cnstat_cmb14: CnstatCmb14,
    tstp_cmb14: TstpCmb14,
    data3_cmb14: Data3Cmb14,
    data2_cmb14: Data2Cmb14,
    data1_cmb14: Data1Cmb14,
    data0_cmb14: Data0Cmb14,
    id0_cmb14: Id0Cmb14,
    id1_cmb14: Id1Cmb14,
    cnstat_hcmb: CnstatHcmb,
    tstp_hcmb: TstpHcmb,
    data3_hcmb: Data3Hcmb,
    data2_hcmb: Data2Hcmb,
    data1_hcmb: Data1Hcmb,
    data0_hcmb: Data0Hcmb,
    id0_hcmb: Id0Hcmb,
    id1_hcmb: Id1Hcmb,
    cgcr: Cgcr,
    ctim: Ctim,
    gmskx: Gmskx,
    gmskb: Gmskb,
    bmskx: Bmskx,
    bmskb: Bmskb,
    cien: Cien,
    cipnd: Cipnd,
    ciclr: Ciclr,
    cicen: Cicen,
    cstpnd: Cstpnd,
    canec: Canec,
    cediag: Cediag,
    ctmr: Ctmr,
}
impl RegisterBlock {
    #[doc = "0x00 - Buffer Status / Control Register"]
    #[inline(always)]
    pub const fn cnstat_cmb0(&self) -> &CnstatCmb0 {
        &self.cnstat_cmb0
    }
    #[doc = "0x04 - CAN Frame Timestamp"]
    #[inline(always)]
    pub const fn tstp_cmb0(&self) -> &TstpCmb0 {
        &self.tstp_cmb0
    }
    #[doc = "0x08 - CAN Frame Data Word 3"]
    #[inline(always)]
    pub const fn data3_cmb0(&self) -> &Data3Cmb0 {
        &self.data3_cmb0
    }
    #[doc = "0x0c - CAN Frame Data Word 2"]
    #[inline(always)]
    pub const fn data2_cmb0(&self) -> &Data2Cmb0 {
        &self.data2_cmb0
    }
    #[doc = "0x10 - CAN Frame Data Word 1"]
    #[inline(always)]
    pub const fn data1_cmb0(&self) -> &Data1Cmb0 {
        &self.data1_cmb0
    }
    #[doc = "0x14 - CAN Frame Data Word 0"]
    #[inline(always)]
    pub const fn data0_cmb0(&self) -> &Data0Cmb0 {
        &self.data0_cmb0
    }
    #[doc = "0x18 - CAN Frame Identifier Word 0"]
    #[inline(always)]
    pub const fn id0_cmb0(&self) -> &Id0Cmb0 {
        &self.id0_cmb0
    }
    #[doc = "0x1c - CAN Frame Identifier Word 1"]
    #[inline(always)]
    pub const fn id1_cmb0(&self) -> &Id1Cmb0 {
        &self.id1_cmb0
    }
    #[doc = "0x20 - Buffer Status / Control Register"]
    #[inline(always)]
    pub const fn cnstat_cmb1(&self) -> &CnstatCmb1 {
        &self.cnstat_cmb1
    }
    #[doc = "0x24 - CAN Frame Timestamp"]
    #[inline(always)]
    pub const fn tstp_cmb1(&self) -> &TstpCmb1 {
        &self.tstp_cmb1
    }
    #[doc = "0x28 - CAN Frame Data Word 3"]
    #[inline(always)]
    pub const fn data3_cmb1(&self) -> &Data3Cmb1 {
        &self.data3_cmb1
    }
    #[doc = "0x2c - CAN Frame Data Word 2"]
    #[inline(always)]
    pub const fn data2_cmb1(&self) -> &Data2Cmb1 {
        &self.data2_cmb1
    }
    #[doc = "0x30 - CAN Frame Data Word 2"]
    #[inline(always)]
    pub const fn data1_cmb1(&self) -> &Data1Cmb1 {
        &self.data1_cmb1
    }
    #[doc = "0x34 - CAN Frame Data Word 0"]
    #[inline(always)]
    pub const fn data0_cmb1(&self) -> &Data0Cmb1 {
        &self.data0_cmb1
    }
    #[doc = "0x38 - CAN Frame Identifier Word 0"]
    #[inline(always)]
    pub const fn id0_cmb1(&self) -> &Id0Cmb1 {
        &self.id0_cmb1
    }
    #[doc = "0x3c - CAN Frame Identifier Word 1"]
    #[inline(always)]
    pub const fn id1_cmb1(&self) -> &Id1Cmb1 {
        &self.id1_cmb1
    }
    #[doc = "0x40 - Buffer Status / Control Register"]
    #[inline(always)]
    pub const fn cnstat_cmb2(&self) -> &CnstatCmb2 {
        &self.cnstat_cmb2
    }
    #[doc = "0x44 - CAN Frame Timestamp"]
    #[inline(always)]
    pub const fn tstp_cmb2(&self) -> &TstpCmb2 {
        &self.tstp_cmb2
    }
    #[doc = "0x48 - CAN Frame Data Word 3"]
    #[inline(always)]
    pub const fn data3_cmb2(&self) -> &Data3Cmb2 {
        &self.data3_cmb2
    }
    #[doc = "0x4c - CAN Frame Data Word 2"]
    #[inline(always)]
    pub const fn data2_cmb2(&self) -> &Data2Cmb2 {
        &self.data2_cmb2
    }
    #[doc = "0x50 - CAN Frame Data Word 2"]
    #[inline(always)]
    pub const fn data1_cmb2(&self) -> &Data1Cmb2 {
        &self.data1_cmb2
    }
    #[doc = "0x54 - CAN Frame Data Word 0"]
    #[inline(always)]
    pub const fn data0_cmb2(&self) -> &Data0Cmb2 {
        &self.data0_cmb2
    }
    #[doc = "0x58 - CAN Frame Identifier Word 0"]
    #[inline(always)]
    pub const fn id0_cmb2(&self) -> &Id0Cmb2 {
        &self.id0_cmb2
    }
    #[doc = "0x5c - CAN Frame Identifier Word 1"]
    #[inline(always)]
    pub const fn id1_cmb2(&self) -> &Id1Cmb2 {
        &self.id1_cmb2
    }
    #[doc = "0x60 - Buffer Status / Control Register"]
    #[inline(always)]
    pub const fn cnstat_cmb3(&self) -> &CnstatCmb3 {
        &self.cnstat_cmb3
    }
    #[doc = "0x64 - CAN Frame Timestamp"]
    #[inline(always)]
    pub const fn tstp_cmb3(&self) -> &TstpCmb3 {
        &self.tstp_cmb3
    }
    #[doc = "0x68 - CAN Frame Data Word 3"]
    #[inline(always)]
    pub const fn data3_cmb3(&self) -> &Data3Cmb3 {
        &self.data3_cmb3
    }
    #[doc = "0x6c - CAN Frame Data Word 2"]
    #[inline(always)]
    pub const fn data2_cmb3(&self) -> &Data2Cmb3 {
        &self.data2_cmb3
    }
    #[doc = "0x70 - CAN Frame Data Word 2"]
    #[inline(always)]
    pub const fn data1_cmb3(&self) -> &Data1Cmb3 {
        &self.data1_cmb3
    }
    #[doc = "0x74 - CAN Frame Data Word 0"]
    #[inline(always)]
    pub const fn data0_cmb3(&self) -> &Data0Cmb3 {
        &self.data0_cmb3
    }
    #[doc = "0x78 - CAN Frame Identifier Word 0"]
    #[inline(always)]
    pub const fn id0_cmb3(&self) -> &Id0Cmb3 {
        &self.id0_cmb3
    }
    #[doc = "0x7c - CAN Frame Identifier Word 1"]
    #[inline(always)]
    pub const fn id1_cmb3(&self) -> &Id1Cmb3 {
        &self.id1_cmb3
    }
    #[doc = "0x80 - Buffer Status / Control Register"]
    #[inline(always)]
    pub const fn cnstat_cmb4(&self) -> &CnstatCmb4 {
        &self.cnstat_cmb4
    }
    #[doc = "0x84 - CAN Frame Timestamp"]
    #[inline(always)]
    pub const fn tstp_cmb4(&self) -> &TstpCmb4 {
        &self.tstp_cmb4
    }
    #[doc = "0x88 - CAN Frame Data Word 3"]
    #[inline(always)]
    pub const fn data3_cmb4(&self) -> &Data3Cmb4 {
        &self.data3_cmb4
    }
    #[doc = "0x8c - CAN Frame Data Word 2"]
    #[inline(always)]
    pub const fn data2_cmb4(&self) -> &Data2Cmb4 {
        &self.data2_cmb4
    }
    #[doc = "0x90 - CAN Frame Data Word 2"]
    #[inline(always)]
    pub const fn data1_cmb4(&self) -> &Data1Cmb4 {
        &self.data1_cmb4
    }
    #[doc = "0x94 - CAN Frame Data Word 0"]
    #[inline(always)]
    pub const fn data0_cmb4(&self) -> &Data0Cmb4 {
        &self.data0_cmb4
    }
    #[doc = "0x98 - CAN Frame Identifier Word 0"]
    #[inline(always)]
    pub const fn id0_cmb4(&self) -> &Id0Cmb4 {
        &self.id0_cmb4
    }
    #[doc = "0x9c - CAN Frame Identifier Word 1"]
    #[inline(always)]
    pub const fn id1_cmb4(&self) -> &Id1Cmb4 {
        &self.id1_cmb4
    }
    #[doc = "0xa0 - Buffer Status / Control Register"]
    #[inline(always)]
    pub const fn cnstat_cmb5(&self) -> &CnstatCmb5 {
        &self.cnstat_cmb5
    }
    #[doc = "0xa4 - CAN Frame Timestamp"]
    #[inline(always)]
    pub const fn tstp_cmb5(&self) -> &TstpCmb5 {
        &self.tstp_cmb5
    }
    #[doc = "0xa8 - CAN Frame Data Word 3"]
    #[inline(always)]
    pub const fn data3_cmb5(&self) -> &Data3Cmb5 {
        &self.data3_cmb5
    }
    #[doc = "0xac - CAN Frame Data Word 2"]
    #[inline(always)]
    pub const fn data2_cmb5(&self) -> &Data2Cmb5 {
        &self.data2_cmb5
    }
    #[doc = "0xb0 - CAN Frame Data Word 2"]
    #[inline(always)]
    pub const fn data1_cmb5(&self) -> &Data1Cmb5 {
        &self.data1_cmb5
    }
    #[doc = "0xb4 - CAN Frame Data Word 0"]
    #[inline(always)]
    pub const fn data0_cmb5(&self) -> &Data0Cmb5 {
        &self.data0_cmb5
    }
    #[doc = "0xb8 - CAN Frame Identifier Word 0"]
    #[inline(always)]
    pub const fn id0_cmb5(&self) -> &Id0Cmb5 {
        &self.id0_cmb5
    }
    #[doc = "0xbc - CAN Frame Identifier Word 1"]
    #[inline(always)]
    pub const fn id1_cmb5(&self) -> &Id1Cmb5 {
        &self.id1_cmb5
    }
    #[doc = "0xc0 - Buffer Status / Control Register"]
    #[inline(always)]
    pub const fn cnstat_cmb6(&self) -> &CnstatCmb6 {
        &self.cnstat_cmb6
    }
    #[doc = "0xc4 - CAN Frame Timestamp"]
    #[inline(always)]
    pub const fn tstp_cmb6(&self) -> &TstpCmb6 {
        &self.tstp_cmb6
    }
    #[doc = "0xc8 - CAN Frame Data Word 3"]
    #[inline(always)]
    pub const fn data3_cmb6(&self) -> &Data3Cmb6 {
        &self.data3_cmb6
    }
    #[doc = "0xcc - CAN Frame Data Word 2"]
    #[inline(always)]
    pub const fn data2_cmb6(&self) -> &Data2Cmb6 {
        &self.data2_cmb6
    }
    #[doc = "0xd0 - CAN Frame Data Word 2"]
    #[inline(always)]
    pub const fn data1_cmb6(&self) -> &Data1Cmb6 {
        &self.data1_cmb6
    }
    #[doc = "0xd4 - CAN Frame Data Word 0"]
    #[inline(always)]
    pub const fn data0_cmb6(&self) -> &Data0Cmb6 {
        &self.data0_cmb6
    }
    #[doc = "0xd8 - CAN Frame Identifier Word 0"]
    #[inline(always)]
    pub const fn id0_cmb6(&self) -> &Id0Cmb6 {
        &self.id0_cmb6
    }
    #[doc = "0xdc - CAN Frame Identifier Word 1"]
    #[inline(always)]
    pub const fn id1_cmb6(&self) -> &Id1Cmb6 {
        &self.id1_cmb6
    }
    #[doc = "0xe0 - Buffer Status / Control Register"]
    #[inline(always)]
    pub const fn cnstat_cmb7(&self) -> &CnstatCmb7 {
        &self.cnstat_cmb7
    }
    #[doc = "0xe4 - CAN Frame Timestamp"]
    #[inline(always)]
    pub const fn tstp_cmb7(&self) -> &TstpCmb7 {
        &self.tstp_cmb7
    }
    #[doc = "0xe8 - CAN Frame Data Word 3"]
    #[inline(always)]
    pub const fn data3_cmb7(&self) -> &Data3Cmb7 {
        &self.data3_cmb7
    }
    #[doc = "0xec - CAN Frame Data Word 2"]
    #[inline(always)]
    pub const fn data2_cmb7(&self) -> &Data2Cmb7 {
        &self.data2_cmb7
    }
    #[doc = "0xf0 - CAN Frame Data Word 2"]
    #[inline(always)]
    pub const fn data1_cmb7(&self) -> &Data1Cmb7 {
        &self.data1_cmb7
    }
    #[doc = "0xf4 - CAN Frame Data Word 0"]
    #[inline(always)]
    pub const fn data0_cmb7(&self) -> &Data0Cmb7 {
        &self.data0_cmb7
    }
    #[doc = "0xf8 - CAN Frame Identifier Word 0"]
    #[inline(always)]
    pub const fn id0_cmb7(&self) -> &Id0Cmb7 {
        &self.id0_cmb7
    }
    #[doc = "0xfc - CAN Frame Identifier Word 1"]
    #[inline(always)]
    pub const fn id1_cmb7(&self) -> &Id1Cmb7 {
        &self.id1_cmb7
    }
    #[doc = "0x100 - Buffer Status / Control Register"]
    #[inline(always)]
    pub const fn cnstat_cmb8(&self) -> &CnstatCmb8 {
        &self.cnstat_cmb8
    }
    #[doc = "0x104 - CAN Frame Timestamp"]
    #[inline(always)]
    pub const fn tstp_cmb8(&self) -> &TstpCmb8 {
        &self.tstp_cmb8
    }
    #[doc = "0x108 - CAN Frame Data Word 3"]
    #[inline(always)]
    pub const fn data3_cmb8(&self) -> &Data3Cmb8 {
        &self.data3_cmb8
    }
    #[doc = "0x10c - CAN Frame Data Word 2"]
    #[inline(always)]
    pub const fn data2_cmb8(&self) -> &Data2Cmb8 {
        &self.data2_cmb8
    }
    #[doc = "0x110 - CAN Frame Data Word 2"]
    #[inline(always)]
    pub const fn data1_cmb8(&self) -> &Data1Cmb8 {
        &self.data1_cmb8
    }
    #[doc = "0x114 - CAN Frame Data Word 0"]
    #[inline(always)]
    pub const fn data0_cmb8(&self) -> &Data0Cmb8 {
        &self.data0_cmb8
    }
    #[doc = "0x118 - CAN Frame Identifier Word 0"]
    #[inline(always)]
    pub const fn id0_cmb8(&self) -> &Id0Cmb8 {
        &self.id0_cmb8
    }
    #[doc = "0x11c - CAN Frame Identifier Word 1"]
    #[inline(always)]
    pub const fn id1_cmb8(&self) -> &Id1Cmb8 {
        &self.id1_cmb8
    }
    #[doc = "0x120 - Buffer Status / Control Register"]
    #[inline(always)]
    pub const fn cnstat_cmb9(&self) -> &CnstatCmb9 {
        &self.cnstat_cmb9
    }
    #[doc = "0x124 - CAN Frame Timestamp"]
    #[inline(always)]
    pub const fn tstp_cmb9(&self) -> &TstpCmb9 {
        &self.tstp_cmb9
    }
    #[doc = "0x128 - CAN Frame Data Word 3"]
    #[inline(always)]
    pub const fn data3_cmb9(&self) -> &Data3Cmb9 {
        &self.data3_cmb9
    }
    #[doc = "0x12c - CAN Frame Data Word 2"]
    #[inline(always)]
    pub const fn data2_cmb9(&self) -> &Data2Cmb9 {
        &self.data2_cmb9
    }
    #[doc = "0x130 - CAN Frame Data Word 2"]
    #[inline(always)]
    pub const fn data1_cmb9(&self) -> &Data1Cmb9 {
        &self.data1_cmb9
    }
    #[doc = "0x134 - CAN Frame Data Word 0"]
    #[inline(always)]
    pub const fn data0_cmb9(&self) -> &Data0Cmb9 {
        &self.data0_cmb9
    }
    #[doc = "0x138 - CAN Frame Identifier Word 0"]
    #[inline(always)]
    pub const fn id0_cmb9(&self) -> &Id0Cmb9 {
        &self.id0_cmb9
    }
    #[doc = "0x13c - CAN Frame Identifier Word 1"]
    #[inline(always)]
    pub const fn id1_cmb9(&self) -> &Id1Cmb9 {
        &self.id1_cmb9
    }
    #[doc = "0x140 - Buffer Status / Control Register"]
    #[inline(always)]
    pub const fn cnstat_cmb10(&self) -> &CnstatCmb10 {
        &self.cnstat_cmb10
    }
    #[doc = "0x144 - CAN Frame Timestamp"]
    #[inline(always)]
    pub const fn tstp_cmb10(&self) -> &TstpCmb10 {
        &self.tstp_cmb10
    }
    #[doc = "0x148 - CAN Frame Data Word 3"]
    #[inline(always)]
    pub const fn data3_cmb10(&self) -> &Data3Cmb10 {
        &self.data3_cmb10
    }
    #[doc = "0x14c - CAN Frame Data Word 2"]
    #[inline(always)]
    pub const fn data2_cmb10(&self) -> &Data2Cmb10 {
        &self.data2_cmb10
    }
    #[doc = "0x150 - CAN Frame Data Word 2"]
    #[inline(always)]
    pub const fn data1_cmb10(&self) -> &Data1Cmb10 {
        &self.data1_cmb10
    }
    #[doc = "0x154 - CAN Frame Data Word 0"]
    #[inline(always)]
    pub const fn data0_cmb10(&self) -> &Data0Cmb10 {
        &self.data0_cmb10
    }
    #[doc = "0x158 - CAN Frame Identifier Word 0"]
    #[inline(always)]
    pub const fn id0_cmb10(&self) -> &Id0Cmb10 {
        &self.id0_cmb10
    }
    #[doc = "0x15c - CAN Frame Identifier Word 1"]
    #[inline(always)]
    pub const fn id1_cmb10(&self) -> &Id1Cmb10 {
        &self.id1_cmb10
    }
    #[doc = "0x160 - Buffer Status / Control Register"]
    #[inline(always)]
    pub const fn cnstat_cmb11(&self) -> &CnstatCmb11 {
        &self.cnstat_cmb11
    }
    #[doc = "0x164 - CAN Frame Timestamp"]
    #[inline(always)]
    pub const fn tstp_cmb11(&self) -> &TstpCmb11 {
        &self.tstp_cmb11
    }
    #[doc = "0x168 - CAN Frame Data Word 3"]
    #[inline(always)]
    pub const fn data3_cmb11(&self) -> &Data3Cmb11 {
        &self.data3_cmb11
    }
    #[doc = "0x16c - CAN Frame Data Word 2"]
    #[inline(always)]
    pub const fn data2_cmb11(&self) -> &Data2Cmb11 {
        &self.data2_cmb11
    }
    #[doc = "0x170 - CAN Frame Data Word 2"]
    #[inline(always)]
    pub const fn data1_cmb11(&self) -> &Data1Cmb11 {
        &self.data1_cmb11
    }
    #[doc = "0x174 - CAN Frame Data Word 0"]
    #[inline(always)]
    pub const fn data0_cmb11(&self) -> &Data0Cmb11 {
        &self.data0_cmb11
    }
    #[doc = "0x178 - CAN Frame Identifier Word 0"]
    #[inline(always)]
    pub const fn id0_cmb11(&self) -> &Id0Cmb11 {
        &self.id0_cmb11
    }
    #[doc = "0x17c - CAN Frame Identifier Word 1"]
    #[inline(always)]
    pub const fn id1_cmb11(&self) -> &Id1Cmb11 {
        &self.id1_cmb11
    }
    #[doc = "0x180 - Buffer Status / Control Register"]
    #[inline(always)]
    pub const fn cnstat_cmb12(&self) -> &CnstatCmb12 {
        &self.cnstat_cmb12
    }
    #[doc = "0x184 - CAN Frame Timestamp"]
    #[inline(always)]
    pub const fn tstp_cmb12(&self) -> &TstpCmb12 {
        &self.tstp_cmb12
    }
    #[doc = "0x188 - CAN Frame Data Word 3"]
    #[inline(always)]
    pub const fn data3_cmb12(&self) -> &Data3Cmb12 {
        &self.data3_cmb12
    }
    #[doc = "0x18c - CAN Frame Data Word 2"]
    #[inline(always)]
    pub const fn data2_cmb12(&self) -> &Data2Cmb12 {
        &self.data2_cmb12
    }
    #[doc = "0x190 - CAN Frame Data Word 2"]
    #[inline(always)]
    pub const fn data1_cmb12(&self) -> &Data1Cmb12 {
        &self.data1_cmb12
    }
    #[doc = "0x194 - CAN Frame Data Word 0"]
    #[inline(always)]
    pub const fn data0_cmb12(&self) -> &Data0Cmb12 {
        &self.data0_cmb12
    }
    #[doc = "0x198 - CAN Frame Identifier Word 0"]
    #[inline(always)]
    pub const fn id0_cmb12(&self) -> &Id0Cmb12 {
        &self.id0_cmb12
    }
    #[doc = "0x19c - CAN Frame Identifier Word 1"]
    #[inline(always)]
    pub const fn id1_cmb12(&self) -> &Id1Cmb12 {
        &self.id1_cmb12
    }
    #[doc = "0x1a0 - Buffer Status / Control Register"]
    #[inline(always)]
    pub const fn cnstat_cmb13(&self) -> &CnstatCmb13 {
        &self.cnstat_cmb13
    }
    #[doc = "0x1a4 - CAN Frame Timestamp"]
    #[inline(always)]
    pub const fn tstp_cmb13(&self) -> &TstpCmb13 {
        &self.tstp_cmb13
    }
    #[doc = "0x1a8 - CAN Frame Data Word 3"]
    #[inline(always)]
    pub const fn data3_cmb13(&self) -> &Data3Cmb13 {
        &self.data3_cmb13
    }
    #[doc = "0x1ac - CAN Frame Data Word 2"]
    #[inline(always)]
    pub const fn data2_cmb13(&self) -> &Data2Cmb13 {
        &self.data2_cmb13
    }
    #[doc = "0x1b0 - CAN Frame Data Word 2"]
    #[inline(always)]
    pub const fn data1_cmb13(&self) -> &Data1Cmb13 {
        &self.data1_cmb13
    }
    #[doc = "0x1b4 - CAN Frame Data Word 0"]
    #[inline(always)]
    pub const fn data0_cmb13(&self) -> &Data0Cmb13 {
        &self.data0_cmb13
    }
    #[doc = "0x1b8 - CAN Frame Identifier Word 0"]
    #[inline(always)]
    pub const fn id0_cmb13(&self) -> &Id0Cmb13 {
        &self.id0_cmb13
    }
    #[doc = "0x1bc - CAN Frame Identifier Word 1"]
    #[inline(always)]
    pub const fn id1_cmb13(&self) -> &Id1Cmb13 {
        &self.id1_cmb13
    }
    #[doc = "0x1c0 - Buffer Status / Control Register"]
    #[inline(always)]
    pub const fn cnstat_cmb14(&self) -> &CnstatCmb14 {
        &self.cnstat_cmb14
    }
    #[doc = "0x1c4 - CAN Frame Timestamp"]
    #[inline(always)]
    pub const fn tstp_cmb14(&self) -> &TstpCmb14 {
        &self.tstp_cmb14
    }
    #[doc = "0x1c8 - CAN Frame Data Word 3"]
    #[inline(always)]
    pub const fn data3_cmb14(&self) -> &Data3Cmb14 {
        &self.data3_cmb14
    }
    #[doc = "0x1cc - CAN Frame Data Word 2"]
    #[inline(always)]
    pub const fn data2_cmb14(&self) -> &Data2Cmb14 {
        &self.data2_cmb14
    }
    #[doc = "0x1d0 - CAN Frame Data Word 2"]
    #[inline(always)]
    pub const fn data1_cmb14(&self) -> &Data1Cmb14 {
        &self.data1_cmb14
    }
    #[doc = "0x1d4 - CAN Frame Data Word 0"]
    #[inline(always)]
    pub const fn data0_cmb14(&self) -> &Data0Cmb14 {
        &self.data0_cmb14
    }
    #[doc = "0x1d8 - CAN Frame Identifier Word 0"]
    #[inline(always)]
    pub const fn id0_cmb14(&self) -> &Id0Cmb14 {
        &self.id0_cmb14
    }
    #[doc = "0x1dc - CAN Frame Identifier Word 1"]
    #[inline(always)]
    pub const fn id1_cmb14(&self) -> &Id1Cmb14 {
        &self.id1_cmb14
    }
    #[doc = "0x1e0 - Buffer Status / Control Register"]
    #[inline(always)]
    pub const fn cnstat_hcmb(&self) -> &CnstatHcmb {
        &self.cnstat_hcmb
    }
    #[doc = "0x1e4 - CAN Frame Timestamp"]
    #[inline(always)]
    pub const fn tstp_hcmb(&self) -> &TstpHcmb {
        &self.tstp_hcmb
    }
    #[doc = "0x1e8 - CAN Frame Data Word 3"]
    #[inline(always)]
    pub const fn data3_hcmb(&self) -> &Data3Hcmb {
        &self.data3_hcmb
    }
    #[doc = "0x1ec - CAN Frame Data Word 2"]
    #[inline(always)]
    pub const fn data2_hcmb(&self) -> &Data2Hcmb {
        &self.data2_hcmb
    }
    #[doc = "0x1f0 - CAN Frame Data Word 2"]
    #[inline(always)]
    pub const fn data1_hcmb(&self) -> &Data1Hcmb {
        &self.data1_hcmb
    }
    #[doc = "0x1f4 - CAN Frame Data Word 0"]
    #[inline(always)]
    pub const fn data0_hcmb(&self) -> &Data0Hcmb {
        &self.data0_hcmb
    }
    #[doc = "0x1f8 - CAN Frame Identifier Word 0"]
    #[inline(always)]
    pub const fn id0_hcmb(&self) -> &Id0Hcmb {
        &self.id0_hcmb
    }
    #[doc = "0x1fc - CAN Frame Identifier Word 1"]
    #[inline(always)]
    pub const fn id1_hcmb(&self) -> &Id1Hcmb {
        &self.id1_hcmb
    }
    #[doc = "0x200 - CAN Global Configuration Register"]
    #[inline(always)]
    pub const fn cgcr(&self) -> &Cgcr {
        &self.cgcr
    }
    #[doc = "0x204 - CAN Timing Register"]
    #[inline(always)]
    pub const fn ctim(&self) -> &Ctim {
        &self.ctim
    }
    #[doc = "0x208 - CAN Global Mask Extension"]
    #[inline(always)]
    pub const fn gmskx(&self) -> &Gmskx {
        &self.gmskx
    }
    #[doc = "0x20c - CAN Global Mask Base"]
    #[inline(always)]
    pub const fn gmskb(&self) -> &Gmskb {
        &self.gmskb
    }
    #[doc = "0x210 - CAN Basic Mask Extension"]
    #[inline(always)]
    pub const fn bmskx(&self) -> &Bmskx {
        &self.bmskx
    }
    #[doc = "0x214 - CAN Basic Mask Base"]
    #[inline(always)]
    pub const fn bmskb(&self) -> &Bmskb {
        &self.bmskb
    }
    #[doc = "0x218 - CAN Interrupt Enable Register"]
    #[inline(always)]
    pub const fn cien(&self) -> &Cien {
        &self.cien
    }
    #[doc = "0x21c - CAN Interrupt Pending Register"]
    #[inline(always)]
    pub const fn cipnd(&self) -> &Cipnd {
        &self.cipnd
    }
    #[doc = "0x220 - CAN Interrupt Clear Register"]
    #[inline(always)]
    pub const fn ciclr(&self) -> &Ciclr {
        &self.ciclr
    }
    #[doc = "0x224 - CAN Interrupt Code Enable Register"]
    #[inline(always)]
    pub const fn cicen(&self) -> &Cicen {
        &self.cicen
    }
    #[doc = "0x228 - CAN Status Pending Register"]
    #[inline(always)]
    pub const fn cstpnd(&self) -> &Cstpnd {
        &self.cstpnd
    }
    #[doc = "0x22c - CAN Error Counter Register"]
    #[inline(always)]
    pub const fn canec(&self) -> &Canec {
        &self.canec
    }
    #[doc = "0x230 - CAN Error Diagnostic Register"]
    #[inline(always)]
    pub const fn cediag(&self) -> &Cediag {
        &self.cediag
    }
    #[doc = "0x234 - CAN Timer Register"]
    #[inline(always)]
    pub const fn ctmr(&self) -> &Ctmr {
        &self.ctmr
    }
}
#[doc = "CNSTAT_CMB0 (rw) register accessor: Buffer Status / Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cnstat_cmb0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cnstat_cmb0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cnstat_cmb0`] module"]
#[doc(alias = "CNSTAT_CMB0")]
pub type CnstatCmb0 = crate::Reg<cnstat_cmb0::CnstatCmb0Spec>;
#[doc = "Buffer Status / Control Register"]
pub mod cnstat_cmb0;
#[doc = "TSTP_CMB0 (rw) register accessor: CAN Frame Timestamp\n\nYou can [`read`](crate::Reg::read) this register and get [`tstp_cmb0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tstp_cmb0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tstp_cmb0`] module"]
#[doc(alias = "TSTP_CMB0")]
pub type TstpCmb0 = crate::Reg<tstp_cmb0::TstpCmb0Spec>;
#[doc = "CAN Frame Timestamp"]
pub mod tstp_cmb0;
#[doc = "DATA3_CMB0 (rw) register accessor: CAN Frame Data Word 3\n\nYou can [`read`](crate::Reg::read) this register and get [`data3_cmb0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data3_cmb0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data3_cmb0`] module"]
#[doc(alias = "DATA3_CMB0")]
pub type Data3Cmb0 = crate::Reg<data3_cmb0::Data3Cmb0Spec>;
#[doc = "CAN Frame Data Word 3"]
pub mod data3_cmb0;
#[doc = "DATA2_CMB0 (rw) register accessor: CAN Frame Data Word 2\n\nYou can [`read`](crate::Reg::read) this register and get [`data2_cmb0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data2_cmb0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data2_cmb0`] module"]
#[doc(alias = "DATA2_CMB0")]
pub type Data2Cmb0 = crate::Reg<data2_cmb0::Data2Cmb0Spec>;
#[doc = "CAN Frame Data Word 2"]
pub mod data2_cmb0;
#[doc = "DATA1_CMB0 (rw) register accessor: CAN Frame Data Word 1\n\nYou can [`read`](crate::Reg::read) this register and get [`data1_cmb0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data1_cmb0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data1_cmb0`] module"]
#[doc(alias = "DATA1_CMB0")]
pub type Data1Cmb0 = crate::Reg<data1_cmb0::Data1Cmb0Spec>;
#[doc = "CAN Frame Data Word 1"]
pub mod data1_cmb0;
#[doc = "DATA0_CMB0 (rw) register accessor: CAN Frame Data Word 0\n\nYou can [`read`](crate::Reg::read) this register and get [`data0_cmb0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data0_cmb0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data0_cmb0`] module"]
#[doc(alias = "DATA0_CMB0")]
pub type Data0Cmb0 = crate::Reg<data0_cmb0::Data0Cmb0Spec>;
#[doc = "CAN Frame Data Word 0"]
pub mod data0_cmb0;
#[doc = "ID0_CMB0 (rw) register accessor: CAN Frame Identifier Word 0\n\nYou can [`read`](crate::Reg::read) this register and get [`id0_cmb0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`id0_cmb0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@id0_cmb0`] module"]
#[doc(alias = "ID0_CMB0")]
pub type Id0Cmb0 = crate::Reg<id0_cmb0::Id0Cmb0Spec>;
#[doc = "CAN Frame Identifier Word 0"]
pub mod id0_cmb0;
#[doc = "ID1_CMB0 (rw) register accessor: CAN Frame Identifier Word 1\n\nYou can [`read`](crate::Reg::read) this register and get [`id1_cmb0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`id1_cmb0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@id1_cmb0`] module"]
#[doc(alias = "ID1_CMB0")]
pub type Id1Cmb0 = crate::Reg<id1_cmb0::Id1Cmb0Spec>;
#[doc = "CAN Frame Identifier Word 1"]
pub mod id1_cmb0;
#[doc = "CNSTAT_CMB1 (rw) register accessor: Buffer Status / Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cnstat_cmb1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cnstat_cmb1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cnstat_cmb1`] module"]
#[doc(alias = "CNSTAT_CMB1")]
pub type CnstatCmb1 = crate::Reg<cnstat_cmb1::CnstatCmb1Spec>;
#[doc = "Buffer Status / Control Register"]
pub mod cnstat_cmb1;
#[doc = "TSTP_CMB1 (rw) register accessor: CAN Frame Timestamp\n\nYou can [`read`](crate::Reg::read) this register and get [`tstp_cmb1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tstp_cmb1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tstp_cmb1`] module"]
#[doc(alias = "TSTP_CMB1")]
pub type TstpCmb1 = crate::Reg<tstp_cmb1::TstpCmb1Spec>;
#[doc = "CAN Frame Timestamp"]
pub mod tstp_cmb1;
#[doc = "DATA3_CMB1 (rw) register accessor: CAN Frame Data Word 3\n\nYou can [`read`](crate::Reg::read) this register and get [`data3_cmb1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data3_cmb1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data3_cmb1`] module"]
#[doc(alias = "DATA3_CMB1")]
pub type Data3Cmb1 = crate::Reg<data3_cmb1::Data3Cmb1Spec>;
#[doc = "CAN Frame Data Word 3"]
pub mod data3_cmb1;
#[doc = "DATA2_CMB1 (rw) register accessor: CAN Frame Data Word 2\n\nYou can [`read`](crate::Reg::read) this register and get [`data2_cmb1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data2_cmb1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data2_cmb1`] module"]
#[doc(alias = "DATA2_CMB1")]
pub type Data2Cmb1 = crate::Reg<data2_cmb1::Data2Cmb1Spec>;
#[doc = "CAN Frame Data Word 2"]
pub mod data2_cmb1;
#[doc = "DATA1_CMB1 (rw) register accessor: CAN Frame Data Word 2\n\nYou can [`read`](crate::Reg::read) this register and get [`data1_cmb1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data1_cmb1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data1_cmb1`] module"]
#[doc(alias = "DATA1_CMB1")]
pub type Data1Cmb1 = crate::Reg<data1_cmb1::Data1Cmb1Spec>;
#[doc = "CAN Frame Data Word 2"]
pub mod data1_cmb1;
#[doc = "DATA0_CMB1 (rw) register accessor: CAN Frame Data Word 0\n\nYou can [`read`](crate::Reg::read) this register and get [`data0_cmb1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data0_cmb1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data0_cmb1`] module"]
#[doc(alias = "DATA0_CMB1")]
pub type Data0Cmb1 = crate::Reg<data0_cmb1::Data0Cmb1Spec>;
#[doc = "CAN Frame Data Word 0"]
pub mod data0_cmb1;
#[doc = "ID0_CMB1 (rw) register accessor: CAN Frame Identifier Word 0\n\nYou can [`read`](crate::Reg::read) this register and get [`id0_cmb1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`id0_cmb1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@id0_cmb1`] module"]
#[doc(alias = "ID0_CMB1")]
pub type Id0Cmb1 = crate::Reg<id0_cmb1::Id0Cmb1Spec>;
#[doc = "CAN Frame Identifier Word 0"]
pub mod id0_cmb1;
#[doc = "ID1_CMB1 (rw) register accessor: CAN Frame Identifier Word 1\n\nYou can [`read`](crate::Reg::read) this register and get [`id1_cmb1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`id1_cmb1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@id1_cmb1`] module"]
#[doc(alias = "ID1_CMB1")]
pub type Id1Cmb1 = crate::Reg<id1_cmb1::Id1Cmb1Spec>;
#[doc = "CAN Frame Identifier Word 1"]
pub mod id1_cmb1;
#[doc = "CNSTAT_CMB2 (rw) register accessor: Buffer Status / Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cnstat_cmb2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cnstat_cmb2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cnstat_cmb2`] module"]
#[doc(alias = "CNSTAT_CMB2")]
pub type CnstatCmb2 = crate::Reg<cnstat_cmb2::CnstatCmb2Spec>;
#[doc = "Buffer Status / Control Register"]
pub mod cnstat_cmb2;
#[doc = "TSTP_CMB2 (rw) register accessor: CAN Frame Timestamp\n\nYou can [`read`](crate::Reg::read) this register and get [`tstp_cmb2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tstp_cmb2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tstp_cmb2`] module"]
#[doc(alias = "TSTP_CMB2")]
pub type TstpCmb2 = crate::Reg<tstp_cmb2::TstpCmb2Spec>;
#[doc = "CAN Frame Timestamp"]
pub mod tstp_cmb2;
#[doc = "DATA3_CMB2 (rw) register accessor: CAN Frame Data Word 3\n\nYou can [`read`](crate::Reg::read) this register and get [`data3_cmb2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data3_cmb2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data3_cmb2`] module"]
#[doc(alias = "DATA3_CMB2")]
pub type Data3Cmb2 = crate::Reg<data3_cmb2::Data3Cmb2Spec>;
#[doc = "CAN Frame Data Word 3"]
pub mod data3_cmb2;
#[doc = "DATA2_CMB2 (rw) register accessor: CAN Frame Data Word 2\n\nYou can [`read`](crate::Reg::read) this register and get [`data2_cmb2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data2_cmb2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data2_cmb2`] module"]
#[doc(alias = "DATA2_CMB2")]
pub type Data2Cmb2 = crate::Reg<data2_cmb2::Data2Cmb2Spec>;
#[doc = "CAN Frame Data Word 2"]
pub mod data2_cmb2;
#[doc = "DATA1_CMB2 (rw) register accessor: CAN Frame Data Word 2\n\nYou can [`read`](crate::Reg::read) this register and get [`data1_cmb2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data1_cmb2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data1_cmb2`] module"]
#[doc(alias = "DATA1_CMB2")]
pub type Data1Cmb2 = crate::Reg<data1_cmb2::Data1Cmb2Spec>;
#[doc = "CAN Frame Data Word 2"]
pub mod data1_cmb2;
#[doc = "DATA0_CMB2 (rw) register accessor: CAN Frame Data Word 0\n\nYou can [`read`](crate::Reg::read) this register and get [`data0_cmb2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data0_cmb2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data0_cmb2`] module"]
#[doc(alias = "DATA0_CMB2")]
pub type Data0Cmb2 = crate::Reg<data0_cmb2::Data0Cmb2Spec>;
#[doc = "CAN Frame Data Word 0"]
pub mod data0_cmb2;
#[doc = "ID0_CMB2 (rw) register accessor: CAN Frame Identifier Word 0\n\nYou can [`read`](crate::Reg::read) this register and get [`id0_cmb2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`id0_cmb2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@id0_cmb2`] module"]
#[doc(alias = "ID0_CMB2")]
pub type Id0Cmb2 = crate::Reg<id0_cmb2::Id0Cmb2Spec>;
#[doc = "CAN Frame Identifier Word 0"]
pub mod id0_cmb2;
#[doc = "ID1_CMB2 (rw) register accessor: CAN Frame Identifier Word 1\n\nYou can [`read`](crate::Reg::read) this register and get [`id1_cmb2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`id1_cmb2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@id1_cmb2`] module"]
#[doc(alias = "ID1_CMB2")]
pub type Id1Cmb2 = crate::Reg<id1_cmb2::Id1Cmb2Spec>;
#[doc = "CAN Frame Identifier Word 1"]
pub mod id1_cmb2;
#[doc = "CNSTAT_CMB3 (rw) register accessor: Buffer Status / Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cnstat_cmb3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cnstat_cmb3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cnstat_cmb3`] module"]
#[doc(alias = "CNSTAT_CMB3")]
pub type CnstatCmb3 = crate::Reg<cnstat_cmb3::CnstatCmb3Spec>;
#[doc = "Buffer Status / Control Register"]
pub mod cnstat_cmb3;
#[doc = "TSTP_CMB3 (rw) register accessor: CAN Frame Timestamp\n\nYou can [`read`](crate::Reg::read) this register and get [`tstp_cmb3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tstp_cmb3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tstp_cmb3`] module"]
#[doc(alias = "TSTP_CMB3")]
pub type TstpCmb3 = crate::Reg<tstp_cmb3::TstpCmb3Spec>;
#[doc = "CAN Frame Timestamp"]
pub mod tstp_cmb3;
#[doc = "DATA3_CMB3 (rw) register accessor: CAN Frame Data Word 3\n\nYou can [`read`](crate::Reg::read) this register and get [`data3_cmb3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data3_cmb3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data3_cmb3`] module"]
#[doc(alias = "DATA3_CMB3")]
pub type Data3Cmb3 = crate::Reg<data3_cmb3::Data3Cmb3Spec>;
#[doc = "CAN Frame Data Word 3"]
pub mod data3_cmb3;
#[doc = "DATA2_CMB3 (rw) register accessor: CAN Frame Data Word 2\n\nYou can [`read`](crate::Reg::read) this register and get [`data2_cmb3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data2_cmb3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data2_cmb3`] module"]
#[doc(alias = "DATA2_CMB3")]
pub type Data2Cmb3 = crate::Reg<data2_cmb3::Data2Cmb3Spec>;
#[doc = "CAN Frame Data Word 2"]
pub mod data2_cmb3;
#[doc = "DATA1_CMB3 (rw) register accessor: CAN Frame Data Word 2\n\nYou can [`read`](crate::Reg::read) this register and get [`data1_cmb3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data1_cmb3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data1_cmb3`] module"]
#[doc(alias = "DATA1_CMB3")]
pub type Data1Cmb3 = crate::Reg<data1_cmb3::Data1Cmb3Spec>;
#[doc = "CAN Frame Data Word 2"]
pub mod data1_cmb3;
#[doc = "DATA0_CMB3 (rw) register accessor: CAN Frame Data Word 0\n\nYou can [`read`](crate::Reg::read) this register and get [`data0_cmb3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data0_cmb3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data0_cmb3`] module"]
#[doc(alias = "DATA0_CMB3")]
pub type Data0Cmb3 = crate::Reg<data0_cmb3::Data0Cmb3Spec>;
#[doc = "CAN Frame Data Word 0"]
pub mod data0_cmb3;
#[doc = "ID0_CMB3 (rw) register accessor: CAN Frame Identifier Word 0\n\nYou can [`read`](crate::Reg::read) this register and get [`id0_cmb3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`id0_cmb3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@id0_cmb3`] module"]
#[doc(alias = "ID0_CMB3")]
pub type Id0Cmb3 = crate::Reg<id0_cmb3::Id0Cmb3Spec>;
#[doc = "CAN Frame Identifier Word 0"]
pub mod id0_cmb3;
#[doc = "ID1_CMB3 (rw) register accessor: CAN Frame Identifier Word 1\n\nYou can [`read`](crate::Reg::read) this register and get [`id1_cmb3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`id1_cmb3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@id1_cmb3`] module"]
#[doc(alias = "ID1_CMB3")]
pub type Id1Cmb3 = crate::Reg<id1_cmb3::Id1Cmb3Spec>;
#[doc = "CAN Frame Identifier Word 1"]
pub mod id1_cmb3;
#[doc = "CNSTAT_CMB4 (rw) register accessor: Buffer Status / Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cnstat_cmb4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cnstat_cmb4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cnstat_cmb4`] module"]
#[doc(alias = "CNSTAT_CMB4")]
pub type CnstatCmb4 = crate::Reg<cnstat_cmb4::CnstatCmb4Spec>;
#[doc = "Buffer Status / Control Register"]
pub mod cnstat_cmb4;
#[doc = "TSTP_CMB4 (rw) register accessor: CAN Frame Timestamp\n\nYou can [`read`](crate::Reg::read) this register and get [`tstp_cmb4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tstp_cmb4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tstp_cmb4`] module"]
#[doc(alias = "TSTP_CMB4")]
pub type TstpCmb4 = crate::Reg<tstp_cmb4::TstpCmb4Spec>;
#[doc = "CAN Frame Timestamp"]
pub mod tstp_cmb4;
#[doc = "DATA3_CMB4 (rw) register accessor: CAN Frame Data Word 3\n\nYou can [`read`](crate::Reg::read) this register and get [`data3_cmb4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data3_cmb4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data3_cmb4`] module"]
#[doc(alias = "DATA3_CMB4")]
pub type Data3Cmb4 = crate::Reg<data3_cmb4::Data3Cmb4Spec>;
#[doc = "CAN Frame Data Word 3"]
pub mod data3_cmb4;
#[doc = "DATA2_CMB4 (rw) register accessor: CAN Frame Data Word 2\n\nYou can [`read`](crate::Reg::read) this register and get [`data2_cmb4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data2_cmb4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data2_cmb4`] module"]
#[doc(alias = "DATA2_CMB4")]
pub type Data2Cmb4 = crate::Reg<data2_cmb4::Data2Cmb4Spec>;
#[doc = "CAN Frame Data Word 2"]
pub mod data2_cmb4;
#[doc = "DATA1_CMB4 (rw) register accessor: CAN Frame Data Word 2\n\nYou can [`read`](crate::Reg::read) this register and get [`data1_cmb4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data1_cmb4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data1_cmb4`] module"]
#[doc(alias = "DATA1_CMB4")]
pub type Data1Cmb4 = crate::Reg<data1_cmb4::Data1Cmb4Spec>;
#[doc = "CAN Frame Data Word 2"]
pub mod data1_cmb4;
#[doc = "DATA0_CMB4 (rw) register accessor: CAN Frame Data Word 0\n\nYou can [`read`](crate::Reg::read) this register and get [`data0_cmb4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data0_cmb4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data0_cmb4`] module"]
#[doc(alias = "DATA0_CMB4")]
pub type Data0Cmb4 = crate::Reg<data0_cmb4::Data0Cmb4Spec>;
#[doc = "CAN Frame Data Word 0"]
pub mod data0_cmb4;
#[doc = "ID0_CMB4 (rw) register accessor: CAN Frame Identifier Word 0\n\nYou can [`read`](crate::Reg::read) this register and get [`id0_cmb4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`id0_cmb4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@id0_cmb4`] module"]
#[doc(alias = "ID0_CMB4")]
pub type Id0Cmb4 = crate::Reg<id0_cmb4::Id0Cmb4Spec>;
#[doc = "CAN Frame Identifier Word 0"]
pub mod id0_cmb4;
#[doc = "ID1_CMB4 (rw) register accessor: CAN Frame Identifier Word 1\n\nYou can [`read`](crate::Reg::read) this register and get [`id1_cmb4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`id1_cmb4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@id1_cmb4`] module"]
#[doc(alias = "ID1_CMB4")]
pub type Id1Cmb4 = crate::Reg<id1_cmb4::Id1Cmb4Spec>;
#[doc = "CAN Frame Identifier Word 1"]
pub mod id1_cmb4;
#[doc = "CNSTAT_CMB5 (rw) register accessor: Buffer Status / Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cnstat_cmb5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cnstat_cmb5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cnstat_cmb5`] module"]
#[doc(alias = "CNSTAT_CMB5")]
pub type CnstatCmb5 = crate::Reg<cnstat_cmb5::CnstatCmb5Spec>;
#[doc = "Buffer Status / Control Register"]
pub mod cnstat_cmb5;
#[doc = "TSTP_CMB5 (rw) register accessor: CAN Frame Timestamp\n\nYou can [`read`](crate::Reg::read) this register and get [`tstp_cmb5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tstp_cmb5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tstp_cmb5`] module"]
#[doc(alias = "TSTP_CMB5")]
pub type TstpCmb5 = crate::Reg<tstp_cmb5::TstpCmb5Spec>;
#[doc = "CAN Frame Timestamp"]
pub mod tstp_cmb5;
#[doc = "DATA3_CMB5 (rw) register accessor: CAN Frame Data Word 3\n\nYou can [`read`](crate::Reg::read) this register and get [`data3_cmb5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data3_cmb5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data3_cmb5`] module"]
#[doc(alias = "DATA3_CMB5")]
pub type Data3Cmb5 = crate::Reg<data3_cmb5::Data3Cmb5Spec>;
#[doc = "CAN Frame Data Word 3"]
pub mod data3_cmb5;
#[doc = "DATA2_CMB5 (rw) register accessor: CAN Frame Data Word 2\n\nYou can [`read`](crate::Reg::read) this register and get [`data2_cmb5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data2_cmb5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data2_cmb5`] module"]
#[doc(alias = "DATA2_CMB5")]
pub type Data2Cmb5 = crate::Reg<data2_cmb5::Data2Cmb5Spec>;
#[doc = "CAN Frame Data Word 2"]
pub mod data2_cmb5;
#[doc = "DATA1_CMB5 (rw) register accessor: CAN Frame Data Word 2\n\nYou can [`read`](crate::Reg::read) this register and get [`data1_cmb5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data1_cmb5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data1_cmb5`] module"]
#[doc(alias = "DATA1_CMB5")]
pub type Data1Cmb5 = crate::Reg<data1_cmb5::Data1Cmb5Spec>;
#[doc = "CAN Frame Data Word 2"]
pub mod data1_cmb5;
#[doc = "DATA0_CMB5 (rw) register accessor: CAN Frame Data Word 0\n\nYou can [`read`](crate::Reg::read) this register and get [`data0_cmb5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data0_cmb5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data0_cmb5`] module"]
#[doc(alias = "DATA0_CMB5")]
pub type Data0Cmb5 = crate::Reg<data0_cmb5::Data0Cmb5Spec>;
#[doc = "CAN Frame Data Word 0"]
pub mod data0_cmb5;
#[doc = "ID0_CMB5 (rw) register accessor: CAN Frame Identifier Word 0\n\nYou can [`read`](crate::Reg::read) this register and get [`id0_cmb5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`id0_cmb5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@id0_cmb5`] module"]
#[doc(alias = "ID0_CMB5")]
pub type Id0Cmb5 = crate::Reg<id0_cmb5::Id0Cmb5Spec>;
#[doc = "CAN Frame Identifier Word 0"]
pub mod id0_cmb5;
#[doc = "ID1_CMB5 (rw) register accessor: CAN Frame Identifier Word 1\n\nYou can [`read`](crate::Reg::read) this register and get [`id1_cmb5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`id1_cmb5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@id1_cmb5`] module"]
#[doc(alias = "ID1_CMB5")]
pub type Id1Cmb5 = crate::Reg<id1_cmb5::Id1Cmb5Spec>;
#[doc = "CAN Frame Identifier Word 1"]
pub mod id1_cmb5;
#[doc = "CNSTAT_CMB6 (rw) register accessor: Buffer Status / Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cnstat_cmb6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cnstat_cmb6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cnstat_cmb6`] module"]
#[doc(alias = "CNSTAT_CMB6")]
pub type CnstatCmb6 = crate::Reg<cnstat_cmb6::CnstatCmb6Spec>;
#[doc = "Buffer Status / Control Register"]
pub mod cnstat_cmb6;
#[doc = "TSTP_CMB6 (rw) register accessor: CAN Frame Timestamp\n\nYou can [`read`](crate::Reg::read) this register and get [`tstp_cmb6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tstp_cmb6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tstp_cmb6`] module"]
#[doc(alias = "TSTP_CMB6")]
pub type TstpCmb6 = crate::Reg<tstp_cmb6::TstpCmb6Spec>;
#[doc = "CAN Frame Timestamp"]
pub mod tstp_cmb6;
#[doc = "DATA3_CMB6 (rw) register accessor: CAN Frame Data Word 3\n\nYou can [`read`](crate::Reg::read) this register and get [`data3_cmb6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data3_cmb6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data3_cmb6`] module"]
#[doc(alias = "DATA3_CMB6")]
pub type Data3Cmb6 = crate::Reg<data3_cmb6::Data3Cmb6Spec>;
#[doc = "CAN Frame Data Word 3"]
pub mod data3_cmb6;
#[doc = "DATA2_CMB6 (rw) register accessor: CAN Frame Data Word 2\n\nYou can [`read`](crate::Reg::read) this register and get [`data2_cmb6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data2_cmb6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data2_cmb6`] module"]
#[doc(alias = "DATA2_CMB6")]
pub type Data2Cmb6 = crate::Reg<data2_cmb6::Data2Cmb6Spec>;
#[doc = "CAN Frame Data Word 2"]
pub mod data2_cmb6;
#[doc = "DATA1_CMB6 (rw) register accessor: CAN Frame Data Word 2\n\nYou can [`read`](crate::Reg::read) this register and get [`data1_cmb6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data1_cmb6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data1_cmb6`] module"]
#[doc(alias = "DATA1_CMB6")]
pub type Data1Cmb6 = crate::Reg<data1_cmb6::Data1Cmb6Spec>;
#[doc = "CAN Frame Data Word 2"]
pub mod data1_cmb6;
#[doc = "DATA0_CMB6 (rw) register accessor: CAN Frame Data Word 0\n\nYou can [`read`](crate::Reg::read) this register and get [`data0_cmb6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data0_cmb6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data0_cmb6`] module"]
#[doc(alias = "DATA0_CMB6")]
pub type Data0Cmb6 = crate::Reg<data0_cmb6::Data0Cmb6Spec>;
#[doc = "CAN Frame Data Word 0"]
pub mod data0_cmb6;
#[doc = "ID0_CMB6 (rw) register accessor: CAN Frame Identifier Word 0\n\nYou can [`read`](crate::Reg::read) this register and get [`id0_cmb6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`id0_cmb6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@id0_cmb6`] module"]
#[doc(alias = "ID0_CMB6")]
pub type Id0Cmb6 = crate::Reg<id0_cmb6::Id0Cmb6Spec>;
#[doc = "CAN Frame Identifier Word 0"]
pub mod id0_cmb6;
#[doc = "ID1_CMB6 (rw) register accessor: CAN Frame Identifier Word 1\n\nYou can [`read`](crate::Reg::read) this register and get [`id1_cmb6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`id1_cmb6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@id1_cmb6`] module"]
#[doc(alias = "ID1_CMB6")]
pub type Id1Cmb6 = crate::Reg<id1_cmb6::Id1Cmb6Spec>;
#[doc = "CAN Frame Identifier Word 1"]
pub mod id1_cmb6;
#[doc = "CNSTAT_CMB7 (rw) register accessor: Buffer Status / Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cnstat_cmb7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cnstat_cmb7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cnstat_cmb7`] module"]
#[doc(alias = "CNSTAT_CMB7")]
pub type CnstatCmb7 = crate::Reg<cnstat_cmb7::CnstatCmb7Spec>;
#[doc = "Buffer Status / Control Register"]
pub mod cnstat_cmb7;
#[doc = "TSTP_CMB7 (rw) register accessor: CAN Frame Timestamp\n\nYou can [`read`](crate::Reg::read) this register and get [`tstp_cmb7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tstp_cmb7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tstp_cmb7`] module"]
#[doc(alias = "TSTP_CMB7")]
pub type TstpCmb7 = crate::Reg<tstp_cmb7::TstpCmb7Spec>;
#[doc = "CAN Frame Timestamp"]
pub mod tstp_cmb7;
#[doc = "DATA3_CMB7 (rw) register accessor: CAN Frame Data Word 3\n\nYou can [`read`](crate::Reg::read) this register and get [`data3_cmb7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data3_cmb7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data3_cmb7`] module"]
#[doc(alias = "DATA3_CMB7")]
pub type Data3Cmb7 = crate::Reg<data3_cmb7::Data3Cmb7Spec>;
#[doc = "CAN Frame Data Word 3"]
pub mod data3_cmb7;
#[doc = "DATA2_CMB7 (rw) register accessor: CAN Frame Data Word 2\n\nYou can [`read`](crate::Reg::read) this register and get [`data2_cmb7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data2_cmb7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data2_cmb7`] module"]
#[doc(alias = "DATA2_CMB7")]
pub type Data2Cmb7 = crate::Reg<data2_cmb7::Data2Cmb7Spec>;
#[doc = "CAN Frame Data Word 2"]
pub mod data2_cmb7;
#[doc = "DATA1_CMB7 (rw) register accessor: CAN Frame Data Word 2\n\nYou can [`read`](crate::Reg::read) this register and get [`data1_cmb7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data1_cmb7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data1_cmb7`] module"]
#[doc(alias = "DATA1_CMB7")]
pub type Data1Cmb7 = crate::Reg<data1_cmb7::Data1Cmb7Spec>;
#[doc = "CAN Frame Data Word 2"]
pub mod data1_cmb7;
#[doc = "DATA0_CMB7 (rw) register accessor: CAN Frame Data Word 0\n\nYou can [`read`](crate::Reg::read) this register and get [`data0_cmb7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data0_cmb7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data0_cmb7`] module"]
#[doc(alias = "DATA0_CMB7")]
pub type Data0Cmb7 = crate::Reg<data0_cmb7::Data0Cmb7Spec>;
#[doc = "CAN Frame Data Word 0"]
pub mod data0_cmb7;
#[doc = "ID0_CMB7 (rw) register accessor: CAN Frame Identifier Word 0\n\nYou can [`read`](crate::Reg::read) this register and get [`id0_cmb7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`id0_cmb7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@id0_cmb7`] module"]
#[doc(alias = "ID0_CMB7")]
pub type Id0Cmb7 = crate::Reg<id0_cmb7::Id0Cmb7Spec>;
#[doc = "CAN Frame Identifier Word 0"]
pub mod id0_cmb7;
#[doc = "ID1_CMB7 (rw) register accessor: CAN Frame Identifier Word 1\n\nYou can [`read`](crate::Reg::read) this register and get [`id1_cmb7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`id1_cmb7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@id1_cmb7`] module"]
#[doc(alias = "ID1_CMB7")]
pub type Id1Cmb7 = crate::Reg<id1_cmb7::Id1Cmb7Spec>;
#[doc = "CAN Frame Identifier Word 1"]
pub mod id1_cmb7;
#[doc = "CNSTAT_CMB8 (rw) register accessor: Buffer Status / Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cnstat_cmb8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cnstat_cmb8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cnstat_cmb8`] module"]
#[doc(alias = "CNSTAT_CMB8")]
pub type CnstatCmb8 = crate::Reg<cnstat_cmb8::CnstatCmb8Spec>;
#[doc = "Buffer Status / Control Register"]
pub mod cnstat_cmb8;
#[doc = "TSTP_CMB8 (rw) register accessor: CAN Frame Timestamp\n\nYou can [`read`](crate::Reg::read) this register and get [`tstp_cmb8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tstp_cmb8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tstp_cmb8`] module"]
#[doc(alias = "TSTP_CMB8")]
pub type TstpCmb8 = crate::Reg<tstp_cmb8::TstpCmb8Spec>;
#[doc = "CAN Frame Timestamp"]
pub mod tstp_cmb8;
#[doc = "DATA3_CMB8 (rw) register accessor: CAN Frame Data Word 3\n\nYou can [`read`](crate::Reg::read) this register and get [`data3_cmb8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data3_cmb8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data3_cmb8`] module"]
#[doc(alias = "DATA3_CMB8")]
pub type Data3Cmb8 = crate::Reg<data3_cmb8::Data3Cmb8Spec>;
#[doc = "CAN Frame Data Word 3"]
pub mod data3_cmb8;
#[doc = "DATA2_CMB8 (rw) register accessor: CAN Frame Data Word 2\n\nYou can [`read`](crate::Reg::read) this register and get [`data2_cmb8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data2_cmb8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data2_cmb8`] module"]
#[doc(alias = "DATA2_CMB8")]
pub type Data2Cmb8 = crate::Reg<data2_cmb8::Data2Cmb8Spec>;
#[doc = "CAN Frame Data Word 2"]
pub mod data2_cmb8;
#[doc = "DATA1_CMB8 (rw) register accessor: CAN Frame Data Word 2\n\nYou can [`read`](crate::Reg::read) this register and get [`data1_cmb8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data1_cmb8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data1_cmb8`] module"]
#[doc(alias = "DATA1_CMB8")]
pub type Data1Cmb8 = crate::Reg<data1_cmb8::Data1Cmb8Spec>;
#[doc = "CAN Frame Data Word 2"]
pub mod data1_cmb8;
#[doc = "DATA0_CMB8 (rw) register accessor: CAN Frame Data Word 0\n\nYou can [`read`](crate::Reg::read) this register and get [`data0_cmb8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data0_cmb8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data0_cmb8`] module"]
#[doc(alias = "DATA0_CMB8")]
pub type Data0Cmb8 = crate::Reg<data0_cmb8::Data0Cmb8Spec>;
#[doc = "CAN Frame Data Word 0"]
pub mod data0_cmb8;
#[doc = "ID0_CMB8 (rw) register accessor: CAN Frame Identifier Word 0\n\nYou can [`read`](crate::Reg::read) this register and get [`id0_cmb8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`id0_cmb8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@id0_cmb8`] module"]
#[doc(alias = "ID0_CMB8")]
pub type Id0Cmb8 = crate::Reg<id0_cmb8::Id0Cmb8Spec>;
#[doc = "CAN Frame Identifier Word 0"]
pub mod id0_cmb8;
#[doc = "ID1_CMB8 (rw) register accessor: CAN Frame Identifier Word 1\n\nYou can [`read`](crate::Reg::read) this register and get [`id1_cmb8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`id1_cmb8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@id1_cmb8`] module"]
#[doc(alias = "ID1_CMB8")]
pub type Id1Cmb8 = crate::Reg<id1_cmb8::Id1Cmb8Spec>;
#[doc = "CAN Frame Identifier Word 1"]
pub mod id1_cmb8;
#[doc = "CNSTAT_CMB9 (rw) register accessor: Buffer Status / Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cnstat_cmb9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cnstat_cmb9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cnstat_cmb9`] module"]
#[doc(alias = "CNSTAT_CMB9")]
pub type CnstatCmb9 = crate::Reg<cnstat_cmb9::CnstatCmb9Spec>;
#[doc = "Buffer Status / Control Register"]
pub mod cnstat_cmb9;
#[doc = "TSTP_CMB9 (rw) register accessor: CAN Frame Timestamp\n\nYou can [`read`](crate::Reg::read) this register and get [`tstp_cmb9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tstp_cmb9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tstp_cmb9`] module"]
#[doc(alias = "TSTP_CMB9")]
pub type TstpCmb9 = crate::Reg<tstp_cmb9::TstpCmb9Spec>;
#[doc = "CAN Frame Timestamp"]
pub mod tstp_cmb9;
#[doc = "DATA3_CMB9 (rw) register accessor: CAN Frame Data Word 3\n\nYou can [`read`](crate::Reg::read) this register and get [`data3_cmb9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data3_cmb9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data3_cmb9`] module"]
#[doc(alias = "DATA3_CMB9")]
pub type Data3Cmb9 = crate::Reg<data3_cmb9::Data3Cmb9Spec>;
#[doc = "CAN Frame Data Word 3"]
pub mod data3_cmb9;
#[doc = "DATA2_CMB9 (rw) register accessor: CAN Frame Data Word 2\n\nYou can [`read`](crate::Reg::read) this register and get [`data2_cmb9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data2_cmb9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data2_cmb9`] module"]
#[doc(alias = "DATA2_CMB9")]
pub type Data2Cmb9 = crate::Reg<data2_cmb9::Data2Cmb9Spec>;
#[doc = "CAN Frame Data Word 2"]
pub mod data2_cmb9;
#[doc = "DATA1_CMB9 (rw) register accessor: CAN Frame Data Word 2\n\nYou can [`read`](crate::Reg::read) this register and get [`data1_cmb9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data1_cmb9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data1_cmb9`] module"]
#[doc(alias = "DATA1_CMB9")]
pub type Data1Cmb9 = crate::Reg<data1_cmb9::Data1Cmb9Spec>;
#[doc = "CAN Frame Data Word 2"]
pub mod data1_cmb9;
#[doc = "DATA0_CMB9 (rw) register accessor: CAN Frame Data Word 0\n\nYou can [`read`](crate::Reg::read) this register and get [`data0_cmb9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data0_cmb9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data0_cmb9`] module"]
#[doc(alias = "DATA0_CMB9")]
pub type Data0Cmb9 = crate::Reg<data0_cmb9::Data0Cmb9Spec>;
#[doc = "CAN Frame Data Word 0"]
pub mod data0_cmb9;
#[doc = "ID0_CMB9 (rw) register accessor: CAN Frame Identifier Word 0\n\nYou can [`read`](crate::Reg::read) this register and get [`id0_cmb9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`id0_cmb9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@id0_cmb9`] module"]
#[doc(alias = "ID0_CMB9")]
pub type Id0Cmb9 = crate::Reg<id0_cmb9::Id0Cmb9Spec>;
#[doc = "CAN Frame Identifier Word 0"]
pub mod id0_cmb9;
#[doc = "ID1_CMB9 (rw) register accessor: CAN Frame Identifier Word 1\n\nYou can [`read`](crate::Reg::read) this register and get [`id1_cmb9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`id1_cmb9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@id1_cmb9`] module"]
#[doc(alias = "ID1_CMB9")]
pub type Id1Cmb9 = crate::Reg<id1_cmb9::Id1Cmb9Spec>;
#[doc = "CAN Frame Identifier Word 1"]
pub mod id1_cmb9;
#[doc = "CNSTAT_CMB10 (rw) register accessor: Buffer Status / Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cnstat_cmb10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cnstat_cmb10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cnstat_cmb10`] module"]
#[doc(alias = "CNSTAT_CMB10")]
pub type CnstatCmb10 = crate::Reg<cnstat_cmb10::CnstatCmb10Spec>;
#[doc = "Buffer Status / Control Register"]
pub mod cnstat_cmb10;
#[doc = "TSTP_CMB10 (rw) register accessor: CAN Frame Timestamp\n\nYou can [`read`](crate::Reg::read) this register and get [`tstp_cmb10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tstp_cmb10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tstp_cmb10`] module"]
#[doc(alias = "TSTP_CMB10")]
pub type TstpCmb10 = crate::Reg<tstp_cmb10::TstpCmb10Spec>;
#[doc = "CAN Frame Timestamp"]
pub mod tstp_cmb10;
#[doc = "DATA3_CMB10 (rw) register accessor: CAN Frame Data Word 3\n\nYou can [`read`](crate::Reg::read) this register and get [`data3_cmb10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data3_cmb10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data3_cmb10`] module"]
#[doc(alias = "DATA3_CMB10")]
pub type Data3Cmb10 = crate::Reg<data3_cmb10::Data3Cmb10Spec>;
#[doc = "CAN Frame Data Word 3"]
pub mod data3_cmb10;
#[doc = "DATA2_CMB10 (rw) register accessor: CAN Frame Data Word 2\n\nYou can [`read`](crate::Reg::read) this register and get [`data2_cmb10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data2_cmb10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data2_cmb10`] module"]
#[doc(alias = "DATA2_CMB10")]
pub type Data2Cmb10 = crate::Reg<data2_cmb10::Data2Cmb10Spec>;
#[doc = "CAN Frame Data Word 2"]
pub mod data2_cmb10;
#[doc = "DATA1_CMB10 (rw) register accessor: CAN Frame Data Word 2\n\nYou can [`read`](crate::Reg::read) this register and get [`data1_cmb10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data1_cmb10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data1_cmb10`] module"]
#[doc(alias = "DATA1_CMB10")]
pub type Data1Cmb10 = crate::Reg<data1_cmb10::Data1Cmb10Spec>;
#[doc = "CAN Frame Data Word 2"]
pub mod data1_cmb10;
#[doc = "DATA0_CMB10 (rw) register accessor: CAN Frame Data Word 0\n\nYou can [`read`](crate::Reg::read) this register and get [`data0_cmb10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data0_cmb10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data0_cmb10`] module"]
#[doc(alias = "DATA0_CMB10")]
pub type Data0Cmb10 = crate::Reg<data0_cmb10::Data0Cmb10Spec>;
#[doc = "CAN Frame Data Word 0"]
pub mod data0_cmb10;
#[doc = "ID0_CMB10 (rw) register accessor: CAN Frame Identifier Word 0\n\nYou can [`read`](crate::Reg::read) this register and get [`id0_cmb10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`id0_cmb10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@id0_cmb10`] module"]
#[doc(alias = "ID0_CMB10")]
pub type Id0Cmb10 = crate::Reg<id0_cmb10::Id0Cmb10Spec>;
#[doc = "CAN Frame Identifier Word 0"]
pub mod id0_cmb10;
#[doc = "ID1_CMB10 (rw) register accessor: CAN Frame Identifier Word 1\n\nYou can [`read`](crate::Reg::read) this register and get [`id1_cmb10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`id1_cmb10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@id1_cmb10`] module"]
#[doc(alias = "ID1_CMB10")]
pub type Id1Cmb10 = crate::Reg<id1_cmb10::Id1Cmb10Spec>;
#[doc = "CAN Frame Identifier Word 1"]
pub mod id1_cmb10;
#[doc = "CNSTAT_CMB11 (rw) register accessor: Buffer Status / Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cnstat_cmb11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cnstat_cmb11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cnstat_cmb11`] module"]
#[doc(alias = "CNSTAT_CMB11")]
pub type CnstatCmb11 = crate::Reg<cnstat_cmb11::CnstatCmb11Spec>;
#[doc = "Buffer Status / Control Register"]
pub mod cnstat_cmb11;
#[doc = "TSTP_CMB11 (rw) register accessor: CAN Frame Timestamp\n\nYou can [`read`](crate::Reg::read) this register and get [`tstp_cmb11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tstp_cmb11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tstp_cmb11`] module"]
#[doc(alias = "TSTP_CMB11")]
pub type TstpCmb11 = crate::Reg<tstp_cmb11::TstpCmb11Spec>;
#[doc = "CAN Frame Timestamp"]
pub mod tstp_cmb11;
#[doc = "DATA3_CMB11 (rw) register accessor: CAN Frame Data Word 3\n\nYou can [`read`](crate::Reg::read) this register and get [`data3_cmb11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data3_cmb11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data3_cmb11`] module"]
#[doc(alias = "DATA3_CMB11")]
pub type Data3Cmb11 = crate::Reg<data3_cmb11::Data3Cmb11Spec>;
#[doc = "CAN Frame Data Word 3"]
pub mod data3_cmb11;
#[doc = "DATA2_CMB11 (rw) register accessor: CAN Frame Data Word 2\n\nYou can [`read`](crate::Reg::read) this register and get [`data2_cmb11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data2_cmb11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data2_cmb11`] module"]
#[doc(alias = "DATA2_CMB11")]
pub type Data2Cmb11 = crate::Reg<data2_cmb11::Data2Cmb11Spec>;
#[doc = "CAN Frame Data Word 2"]
pub mod data2_cmb11;
#[doc = "DATA1_CMB11 (rw) register accessor: CAN Frame Data Word 2\n\nYou can [`read`](crate::Reg::read) this register and get [`data1_cmb11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data1_cmb11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data1_cmb11`] module"]
#[doc(alias = "DATA1_CMB11")]
pub type Data1Cmb11 = crate::Reg<data1_cmb11::Data1Cmb11Spec>;
#[doc = "CAN Frame Data Word 2"]
pub mod data1_cmb11;
#[doc = "DATA0_CMB11 (rw) register accessor: CAN Frame Data Word 0\n\nYou can [`read`](crate::Reg::read) this register and get [`data0_cmb11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data0_cmb11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data0_cmb11`] module"]
#[doc(alias = "DATA0_CMB11")]
pub type Data0Cmb11 = crate::Reg<data0_cmb11::Data0Cmb11Spec>;
#[doc = "CAN Frame Data Word 0"]
pub mod data0_cmb11;
#[doc = "ID0_CMB11 (rw) register accessor: CAN Frame Identifier Word 0\n\nYou can [`read`](crate::Reg::read) this register and get [`id0_cmb11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`id0_cmb11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@id0_cmb11`] module"]
#[doc(alias = "ID0_CMB11")]
pub type Id0Cmb11 = crate::Reg<id0_cmb11::Id0Cmb11Spec>;
#[doc = "CAN Frame Identifier Word 0"]
pub mod id0_cmb11;
#[doc = "ID1_CMB11 (rw) register accessor: CAN Frame Identifier Word 1\n\nYou can [`read`](crate::Reg::read) this register and get [`id1_cmb11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`id1_cmb11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@id1_cmb11`] module"]
#[doc(alias = "ID1_CMB11")]
pub type Id1Cmb11 = crate::Reg<id1_cmb11::Id1Cmb11Spec>;
#[doc = "CAN Frame Identifier Word 1"]
pub mod id1_cmb11;
#[doc = "CNSTAT_CMB12 (rw) register accessor: Buffer Status / Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cnstat_cmb12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cnstat_cmb12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cnstat_cmb12`] module"]
#[doc(alias = "CNSTAT_CMB12")]
pub type CnstatCmb12 = crate::Reg<cnstat_cmb12::CnstatCmb12Spec>;
#[doc = "Buffer Status / Control Register"]
pub mod cnstat_cmb12;
#[doc = "TSTP_CMB12 (rw) register accessor: CAN Frame Timestamp\n\nYou can [`read`](crate::Reg::read) this register and get [`tstp_cmb12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tstp_cmb12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tstp_cmb12`] module"]
#[doc(alias = "TSTP_CMB12")]
pub type TstpCmb12 = crate::Reg<tstp_cmb12::TstpCmb12Spec>;
#[doc = "CAN Frame Timestamp"]
pub mod tstp_cmb12;
#[doc = "DATA3_CMB12 (rw) register accessor: CAN Frame Data Word 3\n\nYou can [`read`](crate::Reg::read) this register and get [`data3_cmb12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data3_cmb12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data3_cmb12`] module"]
#[doc(alias = "DATA3_CMB12")]
pub type Data3Cmb12 = crate::Reg<data3_cmb12::Data3Cmb12Spec>;
#[doc = "CAN Frame Data Word 3"]
pub mod data3_cmb12;
#[doc = "DATA2_CMB12 (rw) register accessor: CAN Frame Data Word 2\n\nYou can [`read`](crate::Reg::read) this register and get [`data2_cmb12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data2_cmb12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data2_cmb12`] module"]
#[doc(alias = "DATA2_CMB12")]
pub type Data2Cmb12 = crate::Reg<data2_cmb12::Data2Cmb12Spec>;
#[doc = "CAN Frame Data Word 2"]
pub mod data2_cmb12;
#[doc = "DATA1_CMB12 (rw) register accessor: CAN Frame Data Word 2\n\nYou can [`read`](crate::Reg::read) this register and get [`data1_cmb12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data1_cmb12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data1_cmb12`] module"]
#[doc(alias = "DATA1_CMB12")]
pub type Data1Cmb12 = crate::Reg<data1_cmb12::Data1Cmb12Spec>;
#[doc = "CAN Frame Data Word 2"]
pub mod data1_cmb12;
#[doc = "DATA0_CMB12 (rw) register accessor: CAN Frame Data Word 0\n\nYou can [`read`](crate::Reg::read) this register and get [`data0_cmb12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data0_cmb12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data0_cmb12`] module"]
#[doc(alias = "DATA0_CMB12")]
pub type Data0Cmb12 = crate::Reg<data0_cmb12::Data0Cmb12Spec>;
#[doc = "CAN Frame Data Word 0"]
pub mod data0_cmb12;
#[doc = "ID0_CMB12 (rw) register accessor: CAN Frame Identifier Word 0\n\nYou can [`read`](crate::Reg::read) this register and get [`id0_cmb12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`id0_cmb12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@id0_cmb12`] module"]
#[doc(alias = "ID0_CMB12")]
pub type Id0Cmb12 = crate::Reg<id0_cmb12::Id0Cmb12Spec>;
#[doc = "CAN Frame Identifier Word 0"]
pub mod id0_cmb12;
#[doc = "ID1_CMB12 (rw) register accessor: CAN Frame Identifier Word 1\n\nYou can [`read`](crate::Reg::read) this register and get [`id1_cmb12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`id1_cmb12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@id1_cmb12`] module"]
#[doc(alias = "ID1_CMB12")]
pub type Id1Cmb12 = crate::Reg<id1_cmb12::Id1Cmb12Spec>;
#[doc = "CAN Frame Identifier Word 1"]
pub mod id1_cmb12;
#[doc = "CNSTAT_CMB13 (rw) register accessor: Buffer Status / Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cnstat_cmb13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cnstat_cmb13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cnstat_cmb13`] module"]
#[doc(alias = "CNSTAT_CMB13")]
pub type CnstatCmb13 = crate::Reg<cnstat_cmb13::CnstatCmb13Spec>;
#[doc = "Buffer Status / Control Register"]
pub mod cnstat_cmb13;
#[doc = "TSTP_CMB13 (rw) register accessor: CAN Frame Timestamp\n\nYou can [`read`](crate::Reg::read) this register and get [`tstp_cmb13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tstp_cmb13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tstp_cmb13`] module"]
#[doc(alias = "TSTP_CMB13")]
pub type TstpCmb13 = crate::Reg<tstp_cmb13::TstpCmb13Spec>;
#[doc = "CAN Frame Timestamp"]
pub mod tstp_cmb13;
#[doc = "DATA3_CMB13 (rw) register accessor: CAN Frame Data Word 3\n\nYou can [`read`](crate::Reg::read) this register and get [`data3_cmb13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data3_cmb13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data3_cmb13`] module"]
#[doc(alias = "DATA3_CMB13")]
pub type Data3Cmb13 = crate::Reg<data3_cmb13::Data3Cmb13Spec>;
#[doc = "CAN Frame Data Word 3"]
pub mod data3_cmb13;
#[doc = "DATA2_CMB13 (rw) register accessor: CAN Frame Data Word 2\n\nYou can [`read`](crate::Reg::read) this register and get [`data2_cmb13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data2_cmb13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data2_cmb13`] module"]
#[doc(alias = "DATA2_CMB13")]
pub type Data2Cmb13 = crate::Reg<data2_cmb13::Data2Cmb13Spec>;
#[doc = "CAN Frame Data Word 2"]
pub mod data2_cmb13;
#[doc = "DATA1_CMB13 (rw) register accessor: CAN Frame Data Word 2\n\nYou can [`read`](crate::Reg::read) this register and get [`data1_cmb13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data1_cmb13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data1_cmb13`] module"]
#[doc(alias = "DATA1_CMB13")]
pub type Data1Cmb13 = crate::Reg<data1_cmb13::Data1Cmb13Spec>;
#[doc = "CAN Frame Data Word 2"]
pub mod data1_cmb13;
#[doc = "DATA0_CMB13 (rw) register accessor: CAN Frame Data Word 0\n\nYou can [`read`](crate::Reg::read) this register and get [`data0_cmb13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data0_cmb13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data0_cmb13`] module"]
#[doc(alias = "DATA0_CMB13")]
pub type Data0Cmb13 = crate::Reg<data0_cmb13::Data0Cmb13Spec>;
#[doc = "CAN Frame Data Word 0"]
pub mod data0_cmb13;
#[doc = "ID0_CMB13 (rw) register accessor: CAN Frame Identifier Word 0\n\nYou can [`read`](crate::Reg::read) this register and get [`id0_cmb13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`id0_cmb13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@id0_cmb13`] module"]
#[doc(alias = "ID0_CMB13")]
pub type Id0Cmb13 = crate::Reg<id0_cmb13::Id0Cmb13Spec>;
#[doc = "CAN Frame Identifier Word 0"]
pub mod id0_cmb13;
#[doc = "ID1_CMB13 (rw) register accessor: CAN Frame Identifier Word 1\n\nYou can [`read`](crate::Reg::read) this register and get [`id1_cmb13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`id1_cmb13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@id1_cmb13`] module"]
#[doc(alias = "ID1_CMB13")]
pub type Id1Cmb13 = crate::Reg<id1_cmb13::Id1Cmb13Spec>;
#[doc = "CAN Frame Identifier Word 1"]
pub mod id1_cmb13;
#[doc = "CNSTAT_CMB14 (rw) register accessor: Buffer Status / Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cnstat_cmb14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cnstat_cmb14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cnstat_cmb14`] module"]
#[doc(alias = "CNSTAT_CMB14")]
pub type CnstatCmb14 = crate::Reg<cnstat_cmb14::CnstatCmb14Spec>;
#[doc = "Buffer Status / Control Register"]
pub mod cnstat_cmb14;
#[doc = "TSTP_CMB14 (rw) register accessor: CAN Frame Timestamp\n\nYou can [`read`](crate::Reg::read) this register and get [`tstp_cmb14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tstp_cmb14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tstp_cmb14`] module"]
#[doc(alias = "TSTP_CMB14")]
pub type TstpCmb14 = crate::Reg<tstp_cmb14::TstpCmb14Spec>;
#[doc = "CAN Frame Timestamp"]
pub mod tstp_cmb14;
#[doc = "DATA3_CMB14 (rw) register accessor: CAN Frame Data Word 3\n\nYou can [`read`](crate::Reg::read) this register and get [`data3_cmb14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data3_cmb14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data3_cmb14`] module"]
#[doc(alias = "DATA3_CMB14")]
pub type Data3Cmb14 = crate::Reg<data3_cmb14::Data3Cmb14Spec>;
#[doc = "CAN Frame Data Word 3"]
pub mod data3_cmb14;
#[doc = "DATA2_CMB14 (rw) register accessor: CAN Frame Data Word 2\n\nYou can [`read`](crate::Reg::read) this register and get [`data2_cmb14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data2_cmb14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data2_cmb14`] module"]
#[doc(alias = "DATA2_CMB14")]
pub type Data2Cmb14 = crate::Reg<data2_cmb14::Data2Cmb14Spec>;
#[doc = "CAN Frame Data Word 2"]
pub mod data2_cmb14;
#[doc = "DATA1_CMB14 (rw) register accessor: CAN Frame Data Word 2\n\nYou can [`read`](crate::Reg::read) this register and get [`data1_cmb14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data1_cmb14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data1_cmb14`] module"]
#[doc(alias = "DATA1_CMB14")]
pub type Data1Cmb14 = crate::Reg<data1_cmb14::Data1Cmb14Spec>;
#[doc = "CAN Frame Data Word 2"]
pub mod data1_cmb14;
#[doc = "DATA0_CMB14 (rw) register accessor: CAN Frame Data Word 0\n\nYou can [`read`](crate::Reg::read) this register and get [`data0_cmb14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data0_cmb14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data0_cmb14`] module"]
#[doc(alias = "DATA0_CMB14")]
pub type Data0Cmb14 = crate::Reg<data0_cmb14::Data0Cmb14Spec>;
#[doc = "CAN Frame Data Word 0"]
pub mod data0_cmb14;
#[doc = "ID0_CMB14 (rw) register accessor: CAN Frame Identifier Word 0\n\nYou can [`read`](crate::Reg::read) this register and get [`id0_cmb14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`id0_cmb14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@id0_cmb14`] module"]
#[doc(alias = "ID0_CMB14")]
pub type Id0Cmb14 = crate::Reg<id0_cmb14::Id0Cmb14Spec>;
#[doc = "CAN Frame Identifier Word 0"]
pub mod id0_cmb14;
#[doc = "ID1_CMB14 (rw) register accessor: CAN Frame Identifier Word 1\n\nYou can [`read`](crate::Reg::read) this register and get [`id1_cmb14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`id1_cmb14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@id1_cmb14`] module"]
#[doc(alias = "ID1_CMB14")]
pub type Id1Cmb14 = crate::Reg<id1_cmb14::Id1Cmb14Spec>;
#[doc = "CAN Frame Identifier Word 1"]
pub mod id1_cmb14;
#[doc = "CNSTAT_HCMB (rw) register accessor: Buffer Status / Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cnstat_hcmb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cnstat_hcmb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cnstat_hcmb`] module"]
#[doc(alias = "CNSTAT_HCMB")]
pub type CnstatHcmb = crate::Reg<cnstat_hcmb::CnstatHcmbSpec>;
#[doc = "Buffer Status / Control Register"]
pub mod cnstat_hcmb;
#[doc = "TSTP_HCMB (rw) register accessor: CAN Frame Timestamp\n\nYou can [`read`](crate::Reg::read) this register and get [`tstp_hcmb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tstp_hcmb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tstp_hcmb`] module"]
#[doc(alias = "TSTP_HCMB")]
pub type TstpHcmb = crate::Reg<tstp_hcmb::TstpHcmbSpec>;
#[doc = "CAN Frame Timestamp"]
pub mod tstp_hcmb;
#[doc = "DATA3_HCMB (rw) register accessor: CAN Frame Data Word 3\n\nYou can [`read`](crate::Reg::read) this register and get [`data3_hcmb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data3_hcmb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data3_hcmb`] module"]
#[doc(alias = "DATA3_HCMB")]
pub type Data3Hcmb = crate::Reg<data3_hcmb::Data3HcmbSpec>;
#[doc = "CAN Frame Data Word 3"]
pub mod data3_hcmb;
#[doc = "DATA2_HCMB (rw) register accessor: CAN Frame Data Word 2\n\nYou can [`read`](crate::Reg::read) this register and get [`data2_hcmb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data2_hcmb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data2_hcmb`] module"]
#[doc(alias = "DATA2_HCMB")]
pub type Data2Hcmb = crate::Reg<data2_hcmb::Data2HcmbSpec>;
#[doc = "CAN Frame Data Word 2"]
pub mod data2_hcmb;
#[doc = "DATA1_HCMB (rw) register accessor: CAN Frame Data Word 2\n\nYou can [`read`](crate::Reg::read) this register and get [`data1_hcmb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data1_hcmb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data1_hcmb`] module"]
#[doc(alias = "DATA1_HCMB")]
pub type Data1Hcmb = crate::Reg<data1_hcmb::Data1HcmbSpec>;
#[doc = "CAN Frame Data Word 2"]
pub mod data1_hcmb;
#[doc = "DATA0_HCMB (rw) register accessor: CAN Frame Data Word 0\n\nYou can [`read`](crate::Reg::read) this register and get [`data0_hcmb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data0_hcmb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data0_hcmb`] module"]
#[doc(alias = "DATA0_HCMB")]
pub type Data0Hcmb = crate::Reg<data0_hcmb::Data0HcmbSpec>;
#[doc = "CAN Frame Data Word 0"]
pub mod data0_hcmb;
#[doc = "ID0_HCMB (rw) register accessor: CAN Frame Identifier Word 0\n\nYou can [`read`](crate::Reg::read) this register and get [`id0_hcmb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`id0_hcmb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@id0_hcmb`] module"]
#[doc(alias = "ID0_HCMB")]
pub type Id0Hcmb = crate::Reg<id0_hcmb::Id0HcmbSpec>;
#[doc = "CAN Frame Identifier Word 0"]
pub mod id0_hcmb;
#[doc = "ID1_HCMB (rw) register accessor: CAN Frame Identifier Word 1\n\nYou can [`read`](crate::Reg::read) this register and get [`id1_hcmb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`id1_hcmb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@id1_hcmb`] module"]
#[doc(alias = "ID1_HCMB")]
pub type Id1Hcmb = crate::Reg<id1_hcmb::Id1HcmbSpec>;
#[doc = "CAN Frame Identifier Word 1"]
pub mod id1_hcmb;
#[doc = "CGCR (rw) register accessor: CAN Global Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cgcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cgcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cgcr`] module"]
#[doc(alias = "CGCR")]
pub type Cgcr = crate::Reg<cgcr::CgcrSpec>;
#[doc = "CAN Global Configuration Register"]
pub mod cgcr;
#[doc = "CTIM (rw) register accessor: CAN Timing Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctim::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctim::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctim`] module"]
#[doc(alias = "CTIM")]
pub type Ctim = crate::Reg<ctim::CtimSpec>;
#[doc = "CAN Timing Register"]
pub mod ctim;
#[doc = "GMSKX (rw) register accessor: CAN Global Mask Extension\n\nYou can [`read`](crate::Reg::read) this register and get [`gmskx::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gmskx::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gmskx`] module"]
#[doc(alias = "GMSKX")]
pub type Gmskx = crate::Reg<gmskx::GmskxSpec>;
#[doc = "CAN Global Mask Extension"]
pub mod gmskx;
#[doc = "GMSKB (rw) register accessor: CAN Global Mask Base\n\nYou can [`read`](crate::Reg::read) this register and get [`gmskb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gmskb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gmskb`] module"]
#[doc(alias = "GMSKB")]
pub type Gmskb = crate::Reg<gmskb::GmskbSpec>;
#[doc = "CAN Global Mask Base"]
pub mod gmskb;
#[doc = "BMSKX (rw) register accessor: CAN Basic Mask Extension\n\nYou can [`read`](crate::Reg::read) this register and get [`bmskx::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bmskx::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bmskx`] module"]
#[doc(alias = "BMSKX")]
pub type Bmskx = crate::Reg<bmskx::BmskxSpec>;
#[doc = "CAN Basic Mask Extension"]
pub mod bmskx;
#[doc = "BMSKB (rw) register accessor: CAN Basic Mask Base\n\nYou can [`read`](crate::Reg::read) this register and get [`bmskb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bmskb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bmskb`] module"]
#[doc(alias = "BMSKB")]
pub type Bmskb = crate::Reg<bmskb::BmskbSpec>;
#[doc = "CAN Basic Mask Base"]
pub mod bmskb;
#[doc = "CIEN (rw) register accessor: CAN Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cien::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cien::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cien`] module"]
#[doc(alias = "CIEN")]
pub type Cien = crate::Reg<cien::CienSpec>;
#[doc = "CAN Interrupt Enable Register"]
pub mod cien;
#[doc = "CIPND (rw) register accessor: CAN Interrupt Pending Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cipnd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cipnd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cipnd`] module"]
#[doc(alias = "CIPND")]
pub type Cipnd = crate::Reg<cipnd::CipndSpec>;
#[doc = "CAN Interrupt Pending Register"]
pub mod cipnd;
#[doc = "CICLR (rw) register accessor: CAN Interrupt Clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ciclr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ciclr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ciclr`] module"]
#[doc(alias = "CICLR")]
pub type Ciclr = crate::Reg<ciclr::CiclrSpec>;
#[doc = "CAN Interrupt Clear Register"]
pub mod ciclr;
#[doc = "CICEN (rw) register accessor: CAN Interrupt Code Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cicen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cicen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cicen`] module"]
#[doc(alias = "CICEN")]
pub type Cicen = crate::Reg<cicen::CicenSpec>;
#[doc = "CAN Interrupt Code Enable Register"]
pub mod cicen;
#[doc = "CSTPND (rw) register accessor: CAN Status Pending Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cstpnd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cstpnd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cstpnd`] module"]
#[doc(alias = "CSTPND")]
pub type Cstpnd = crate::Reg<cstpnd::CstpndSpec>;
#[doc = "CAN Status Pending Register"]
pub mod cstpnd;
#[doc = "CANEC (rw) register accessor: CAN Error Counter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`canec::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`canec::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@canec`] module"]
#[doc(alias = "CANEC")]
pub type Canec = crate::Reg<canec::CanecSpec>;
#[doc = "CAN Error Counter Register"]
pub mod canec;
#[doc = "CEDIAG (rw) register accessor: CAN Error Diagnostic Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cediag::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cediag::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cediag`] module"]
#[doc(alias = "CEDIAG")]
pub type Cediag = crate::Reg<cediag::CediagSpec>;
#[doc = "CAN Error Diagnostic Register"]
pub mod cediag;
#[doc = "CTMR (rw) register accessor: CAN Timer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctmr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctmr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctmr`] module"]
#[doc(alias = "CTMR")]
pub type Ctmr = crate::Reg<ctmr::CtmrSpec>;
#[doc = "CAN Timer Register"]
pub mod ctmr;
