#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved_0_cr_b: [u8; 0x04],
    _reserved_1_cmr: [u8; 0x04],
    #[doc = "0x08 - Status register"]
    pub sr: SR,
    _reserved_3_ir: [u8; 0x04],
    _reserved_4_ier_p: [u8; 0x04],
    #[doc = "0x14 - Basic Acceptance Mask register"]
    pub group0_amr_b: GROUP0_AMR_B,
    #[doc = "0x18 - Bus Timing register 0"]
    pub btr0: BTR0,
    #[doc = "0x1c - Bus Timing register 1"]
    pub btr1: BTR1,
    _reserved8: [u8; 0x08],
    #[doc = "0x28 - Basic TX ID register 0"]
    pub txid0_b: TXID0_B,
    _reserved_9_alc_p: [u8; 0x04],
    _reserved_10_ecc_p: [u8; 0x04],
    _reserved_11_ewlr_p: [u8; 0x04],
    _reserved_12_rxerr_p: [u8; 0x04],
    _reserved_13_txdr3_b: [u8; 0x04],
    _reserved_14_sff_p: [u8; 0x04],
    _reserved_15_txdr5_b: [u8; 0x04],
    _reserved_16_txdr6_b: [u8; 0x04],
    _reserved_17_txdr7_b: [u8; 0x04],
    _reserved_18_txdata1_p: [u8; 0x04],
    _reserved_19_txdata2_p: [u8; 0x04],
    _reserved_20_txdata3_p: [u8; 0x04],
    _reserved_21_txdata4_p: [u8; 0x04],
    #[doc = "0x60 - Peli TX Data register 5"]
    pub txdata5_p: TXDATA5_P,
    #[doc = "0x64 - Peli TX Data register 6"]
    pub txdata6_p: TXDATA6_P,
    #[doc = "0x68 - Peli TX Data register 7"]
    pub txdata7_p: TXDATA7_P,
    #[doc = "0x6c - Peli TX Data register 8"]
    pub txdata8_p: TXDATA8_P,
    #[doc = "0x70 - Peli TX Data register 9"]
    pub txdata9_p: TXDATA9_P,
    _reserved27: [u8; 0x08],
    #[doc = "0x7c - Clock Divider register"]
    pub cdr: CDR,
    #[doc = "0x80 - Filter Mode register 0"]
    pub afm0: AFM0,
    #[doc = "0x84 - Filter Mode register 1"]
    pub afm1: AFM1,
    #[doc = "0x88 - Filter Mode register 2"]
    pub afm2: AFM2,
    #[doc = "0x8c - Filter Group Enable Register 0"]
    pub fga0: FGA0,
    #[doc = "0x90 - Filter Group Enable Register 1"]
    pub fga1: FGA1,
    #[doc = "0x94 - Filter Group Enable Register 2"]
    pub fga2: FGA2,
    _reserved_34_group1_acr: [u8; 0x04],
    #[doc = "0x9c - Peli Acceptance Code register1"]
    pub group1_acr1_p: GROUP1_ACR1_P,
    #[doc = "0xa0 - Peli Acceptance Code register2"]
    pub group1_acr2_p: GROUP1_ACR2_P,
    #[doc = "0xa4 - Peli Acceptance Code register3"]
    pub group1_acr3_p: GROUP1_ACR3_P,
    _reserved_38_group1_amr: [u8; 0x04],
    #[doc = "0xac - Peli Acceptance Mask register 1"]
    pub group1_amr1_p: GROUP1_AMR1_P,
    #[doc = "0xb0 - Peli Acceptance Mask register 2"]
    pub group1_amr2_p: GROUP1_AMR2_P,
    #[doc = "0xb4 - Peli Acceptance Mask register 3"]
    pub group1_amr3_p: GROUP1_AMR3_P,
    _reserved_42_group2_acr: [u8; 0x04],
    #[doc = "0xbc - Peli Acceptance Code register1"]
    pub group2_acr1_p: GROUP2_ACR1_P,
    #[doc = "0xc0 - Peli Acceptance Code register2"]
    pub group2_acr2_p: GROUP2_ACR2_P,
    #[doc = "0xc4 - Peli Acceptance Code register3"]
    pub group2_acr3_p: GROUP2_ACR3_P,
    _reserved_46_group2_amr: [u8; 0x04],
    #[doc = "0xcc - Peli Acceptance Mask register 1"]
    pub group2_amr1_p: GROUP2_AMR1_P,
    #[doc = "0xd0 - Peli Acceptance Mask register 2"]
    pub group2_amr2_p: GROUP2_AMR2_P,
    #[doc = "0xd4 - Peli Acceptance Mask register 3"]
    pub group2_amr3_p: GROUP2_AMR3_P,
    _reserved_50_group3_acr: [u8; 0x04],
    #[doc = "0xdc - Peli Acceptance Code register1"]
    pub group3_acr1_p: GROUP3_ACR1_P,
    #[doc = "0xe0 - Peli Acceptance Code register2"]
    pub group3_acr2_p: GROUP3_ACR2_P,
    #[doc = "0xe4 - Peli Acceptance Code register3"]
    pub group3_acr3_p: GROUP3_ACR3_P,
    _reserved_54_group3_amr: [u8; 0x04],
    #[doc = "0xec - Peli Acceptance Mask register 1"]
    pub group3_amr1_p: GROUP3_AMR1_P,
    #[doc = "0xf0 - Peli Acceptance Mask register 2"]
    pub group3_amr2_p: GROUP3_AMR2_P,
    #[doc = "0xf4 - Peli Acceptance Mask register 3"]
    pub group3_amr3_p: GROUP3_AMR3_P,
    _reserved_58_group4_acr: [u8; 0x04],
    #[doc = "0xfc - Peli Acceptance Code register1"]
    pub group4_acr1_p: GROUP4_ACR1_P,
    #[doc = "0x100 - Peli Acceptance Code register2"]
    pub group4_acr2_p: GROUP4_ACR2_P,
    #[doc = "0x104 - Peli Acceptance Code register3"]
    pub group4_acr3_p: GROUP4_ACR3_P,
    _reserved_62_group4_amr: [u8; 0x04],
    #[doc = "0x10c - Peli Acceptance Mask register 1"]
    pub group4_amr1_p: GROUP4_AMR1_P,
    #[doc = "0x110 - Peli Acceptance Mask register 2"]
    pub group4_amr2_p: GROUP4_AMR2_P,
    #[doc = "0x114 - Peli Acceptance Mask register 3"]
    pub group4_amr3_p: GROUP4_AMR3_P,
    _reserved_66_group5_acr: [u8; 0x04],
    #[doc = "0x11c - Peli Acceptance Code register1"]
    pub group5_acr1_p: GROUP5_ACR1_P,
    #[doc = "0x120 - Peli Acceptance Code register2"]
    pub group5_acr2_p: GROUP5_ACR2_P,
    #[doc = "0x124 - Peli Acceptance Code register3"]
    pub group5_acr3_p: GROUP5_ACR3_P,
    _reserved_70_group5_amr: [u8; 0x04],
    #[doc = "0x12c - Peli Acceptance Mask register 1"]
    pub group5_amr1_p: GROUP5_AMR1_P,
    #[doc = "0x130 - Peli Acceptance Mask register 2"]
    pub group5_amr2_p: GROUP5_AMR2_P,
    #[doc = "0x134 - Peli Acceptance Mask register 3"]
    pub group5_amr3_p: GROUP5_AMR3_P,
    _reserved_74_group6_acr: [u8; 0x04],
    #[doc = "0x13c - Peli Acceptance Code register1"]
    pub group6_acr1_p: GROUP6_ACR1_P,
    #[doc = "0x140 - Peli Acceptance Code register2"]
    pub group6_acr2_p: GROUP6_ACR2_P,
    #[doc = "0x144 - Peli Acceptance Code register3"]
    pub group6_acr3_p: GROUP6_ACR3_P,
    _reserved_78_group6_amr: [u8; 0x04],
    #[doc = "0x14c - Peli Acceptance Mask register 1"]
    pub group6_amr1_p: GROUP6_AMR1_P,
    #[doc = "0x150 - Peli Acceptance Mask register 2"]
    pub group6_amr2_p: GROUP6_AMR2_P,
    #[doc = "0x154 - Peli Acceptance Mask register 3"]
    pub group6_amr3_p: GROUP6_AMR3_P,
    _reserved_82_group7_acr: [u8; 0x04],
    #[doc = "0x15c - Peli Acceptance Code register1"]
    pub group7_acr1_p: GROUP7_ACR1_P,
    #[doc = "0x160 - Peli Acceptance Code register2"]
    pub group7_acr2_p: GROUP7_ACR2_P,
    #[doc = "0x164 - Peli Acceptance Code register3"]
    pub group7_acr3_p: GROUP7_ACR3_P,
    _reserved_86_group7_amr: [u8; 0x04],
    #[doc = "0x16c - Peli Acceptance Mask register 1"]
    pub group7_amr1_p: GROUP7_AMR1_P,
    #[doc = "0x170 - Peli Acceptance Mask register 2"]
    pub group7_amr2_p: GROUP7_AMR2_P,
    #[doc = "0x174 - Peli Acceptance Mask register 3"]
    pub group7_amr3_p: GROUP7_AMR3_P,
    _reserved_90_group8_acr: [u8; 0x04],
    #[doc = "0x17c - Peli Acceptance Code register1"]
    pub group8_acr1_p: GROUP8_ACR1_P,
    #[doc = "0x180 - Peli Acceptance Code register2"]
    pub group8_acr2_p: GROUP8_ACR2_P,
    #[doc = "0x184 - Peli Acceptance Code register3"]
    pub group8_acr3_p: GROUP8_ACR3_P,
    _reserved_94_group8_amr: [u8; 0x04],
    #[doc = "0x18c - Peli Acceptance Mask register 1"]
    pub group8_amr1_p: GROUP8_AMR1_P,
    #[doc = "0x190 - Peli Acceptance Mask register 2"]
    pub group8_amr2_p: GROUP8_AMR2_P,
    #[doc = "0x194 - Peli Acceptance Mask register 3"]
    pub group8_amr3_p: GROUP8_AMR3_P,
    _reserved_98_group9_acr: [u8; 0x04],
    #[doc = "0x19c - Peli Acceptance Code register1"]
    pub group9_acr1_p: GROUP9_ACR1_P,
    #[doc = "0x1a0 - Peli Acceptance Code register2"]
    pub group9_acr2_p: GROUP9_ACR2_P,
    #[doc = "0x1a4 - Peli Acceptance Code register3"]
    pub group9_acr3_p: GROUP9_ACR3_P,
    _reserved_102_group9_amr: [u8; 0x04],
    #[doc = "0x1ac - Peli Acceptance Mask register 1"]
    pub group9_amr1_p: GROUP9_AMR1_P,
    #[doc = "0x1b0 - Peli Acceptance Mask register 2"]
    pub group9_amr2_p: GROUP9_AMR2_P,
    #[doc = "0x1b4 - Peli Acceptance Mask register 3"]
    pub group9_amr3_p: GROUP9_AMR3_P,
    _reserved_106_group10_acr: [u8; 0x04],
    #[doc = "0x1bc - Peli Acceptance Code register1"]
    pub group10_acr1_p: GROUP10_ACR1_P,
    #[doc = "0x1c0 - Peli Acceptance Code register2"]
    pub group10_acr2_p: GROUP10_ACR2_P,
    #[doc = "0x1c4 - Peli Acceptance Code register3"]
    pub group10_acr3_p: GROUP10_ACR3_P,
    _reserved_110_group10_amr: [u8; 0x04],
    #[doc = "0x1cc - Peli Acceptance Mask register 1"]
    pub group10_amr1_p: GROUP10_AMR1_P,
    #[doc = "0x1d0 - Peli Acceptance Mask register 2"]
    pub group10_amr2_p: GROUP10_AMR2_P,
    #[doc = "0x1d4 - Peli Acceptance Mask register 3"]
    pub group10_amr3_p: GROUP10_AMR3_P,
    _reserved_114_group11_acr: [u8; 0x04],
    #[doc = "0x1dc - Peli Acceptance Code register1"]
    pub group11_acr1_p: GROUP11_ACR1_P,
    #[doc = "0x1e0 - Peli Acceptance Code register2"]
    pub group11_acr2_p: GROUP11_ACR2_P,
    #[doc = "0x1e4 - Peli Acceptance Code register3"]
    pub group11_acr3_p: GROUP11_ACR3_P,
    _reserved_118_group11_amr: [u8; 0x04],
    #[doc = "0x1ec - Peli Acceptance Mask register 1"]
    pub group11_amr1_p: GROUP11_AMR1_P,
    #[doc = "0x1f0 - Peli Acceptance Mask register 2"]
    pub group11_amr2_p: GROUP11_AMR2_P,
    #[doc = "0x1f4 - Peli Acceptance Mask register 3"]
    pub group11_amr3_p: GROUP11_AMR3_P,
    _reserved_122_group12_acr: [u8; 0x04],
    #[doc = "0x1fc - Peli Acceptance Code register1"]
    pub group12_acr1_p: GROUP12_ACR1_P,
    #[doc = "0x200 - Peli Acceptance Code register2"]
    pub group12_acr2_p: GROUP12_ACR2_P,
    #[doc = "0x204 - Peli Acceptance Code register3"]
    pub group12_acr3_p: GROUP12_ACR3_P,
    _reserved_126_group12_amr: [u8; 0x04],
    #[doc = "0x20c - Peli Acceptance Mask register 1"]
    pub group12_amr1_p: GROUP12_AMR1_P,
    #[doc = "0x210 - Peli Acceptance Mask register 2"]
    pub group12_amr2_p: GROUP12_AMR2_P,
    #[doc = "0x214 - Peli Acceptance Mask register 3"]
    pub group12_amr3_p: GROUP12_AMR3_P,
    _reserved_130_group13_acr: [u8; 0x04],
    #[doc = "0x21c - Peli Acceptance Code register1"]
    pub group13_acr1_p: GROUP13_ACR1_P,
    #[doc = "0x220 - Peli Acceptance Code register2"]
    pub group13_acr2_p: GROUP13_ACR2_P,
    #[doc = "0x224 - Peli Acceptance Code register3"]
    pub group13_acr3_p: GROUP13_ACR3_P,
    _reserved_134_group13_amr: [u8; 0x04],
    #[doc = "0x22c - Peli Acceptance Mask register 1"]
    pub group13_amr1_p: GROUP13_AMR1_P,
    #[doc = "0x230 - Peli Acceptance Mask register 2"]
    pub group13_amr2_p: GROUP13_AMR2_P,
    #[doc = "0x234 - Peli Acceptance Mask register 3"]
    pub group13_amr3_p: GROUP13_AMR3_P,
    _reserved_138_group14_acr: [u8; 0x04],
    #[doc = "0x23c - Peli Acceptance Code register1"]
    pub group14_acr1_p: GROUP14_ACR1_P,
    #[doc = "0x240 - Peli Acceptance Code register2"]
    pub group14_acr2_p: GROUP14_ACR2_P,
    #[doc = "0x244 - Peli Acceptance Code register3"]
    pub group14_acr3_p: GROUP14_ACR3_P,
    _reserved_142_group14_amr: [u8; 0x04],
    #[doc = "0x24c - Peli Acceptance Mask register 1"]
    pub group14_amr1_p: GROUP14_AMR1_P,
    #[doc = "0x250 - Peli Acceptance Mask register 2"]
    pub group14_amr2_p: GROUP14_AMR2_P,
    #[doc = "0x254 - Peli Acceptance Mask register 3"]
    pub group14_amr3_p: GROUP14_AMR3_P,
    _reserved_146_group15_acr: [u8; 0x04],
    #[doc = "0x25c - Peli Acceptance Code register1"]
    pub group15_acr1_p: GROUP15_ACR1_P,
    #[doc = "0x260 - Peli Acceptance Code register2"]
    pub group15_acr2_p: GROUP15_ACR2_P,
    #[doc = "0x264 - Peli Acceptance Code register3"]
    pub group15_acr3_p: GROUP15_ACR3_P,
    _reserved_150_group15_amr: [u8; 0x04],
    #[doc = "0x26c - Peli Acceptance Mask register 1"]
    pub group15_amr1_p: GROUP15_AMR1_P,
    #[doc = "0x270 - Peli Acceptance Mask register 2"]
    pub group15_amr2_p: GROUP15_AMR2_P,
    #[doc = "0x274 - Peli Acceptance Mask register 3"]
    pub group15_amr3_p: GROUP15_AMR3_P,
    _reserved_154_group16_acr: [u8; 0x04],
    #[doc = "0x27c - Peli Acceptance Code register1"]
    pub group16_acr1_p: GROUP16_ACR1_P,
    #[doc = "0x280 - Peli Acceptance Code register2"]
    pub group16_acr2_p: GROUP16_ACR2_P,
    #[doc = "0x284 - Peli Acceptance Code register3"]
    pub group16_acr3_p: GROUP16_ACR3_P,
    _reserved_158_group16_amr: [u8; 0x04],
    #[doc = "0x28c - Peli Acceptance Mask register 1"]
    pub group16_amr1_p: GROUP16_AMR1_P,
    #[doc = "0x290 - Peli Acceptance Mask register 2"]
    pub group16_amr2_p: GROUP16_AMR2_P,
    #[doc = "0x294 - Peli Acceptance Mask register 3"]
    pub group16_amr3_p: GROUP16_AMR3_P,
    _reserved_162_group17_acr: [u8; 0x04],
    #[doc = "0x29c - Peli Acceptance Code register1"]
    pub group17_acr1_p: GROUP17_ACR1_P,
    #[doc = "0x2a0 - Peli Acceptance Code register2"]
    pub group17_acr2_p: GROUP17_ACR2_P,
    #[doc = "0x2a4 - Peli Acceptance Code register3"]
    pub group17_acr3_p: GROUP17_ACR3_P,
    _reserved_166_group17_amr: [u8; 0x04],
    #[doc = "0x2ac - Peli Acceptance Mask register 1"]
    pub group17_amr1_p: GROUP17_AMR1_P,
    #[doc = "0x2b0 - Peli Acceptance Mask register 2"]
    pub group17_amr2_p: GROUP17_AMR2_P,
    #[doc = "0x2b4 - Peli Acceptance Mask register 3"]
    pub group17_amr3_p: GROUP17_AMR3_P,
    _reserved_170_group18_acr: [u8; 0x04],
    #[doc = "0x2bc - Peli Acceptance Code register1"]
    pub group18_acr1_p: GROUP18_ACR1_P,
    #[doc = "0x2c0 - Peli Acceptance Code register2"]
    pub group18_acr2_p: GROUP18_ACR2_P,
    #[doc = "0x2c4 - Peli Acceptance Code register3"]
    pub group18_acr3_p: GROUP18_ACR3_P,
    _reserved_174_group18_amr: [u8; 0x04],
    #[doc = "0x2cc - Peli Acceptance Mask register 1"]
    pub group18_amr1_p: GROUP18_AMR1_P,
    #[doc = "0x2d0 - Peli Acceptance Mask register 2"]
    pub group18_amr2_p: GROUP18_AMR2_P,
    #[doc = "0x2d4 - Peli Acceptance Mask register 3"]
    pub group18_amr3_p: GROUP18_AMR3_P,
    _reserved_178_group19_acr: [u8; 0x04],
    #[doc = "0x2dc - Peli Acceptance Code register1"]
    pub group19_acr1_p: GROUP19_ACR1_P,
    #[doc = "0x2e0 - Peli Acceptance Code register2"]
    pub group19_acr2_p: GROUP19_ACR2_P,
    #[doc = "0x2e4 - Peli Acceptance Code register3"]
    pub group19_acr3_p: GROUP19_ACR3_P,
    _reserved_182_group19_amr: [u8; 0x04],
    #[doc = "0x2ec - Peli Acceptance Mask register 1"]
    pub group19_amr1_p: GROUP19_AMR1_P,
    #[doc = "0x2f0 - Peli Acceptance Mask register 2"]
    pub group19_amr2_p: GROUP19_AMR2_P,
    #[doc = "0x2f4 - Peli Acceptance Mask register 3"]
    pub group19_amr3_p: GROUP19_AMR3_P,
}
impl RegisterBlock {
    #[doc = "0x00 - Peli Mode register"]
    #[inline(always)]
    pub const fn mod_p(&self) -> &MOD_P {
        unsafe { &*(self as *const Self).cast::<u8>().add(0usize).cast() }
    }
    #[doc = "0x00 - Basic control register"]
    #[inline(always)]
    pub const fn cr_b(&self) -> &CR_B {
        unsafe { &*(self as *const Self).cast::<u8>().add(0usize).cast() }
    }
    #[doc = "0x04 - Peli Command register"]
    #[inline(always)]
    pub const fn cmr_p(&self) -> &CMR_P {
        unsafe { &*(self as *const Self).cast::<u8>().add(4usize).cast() }
    }
    #[doc = "0x04 - Basic Command register"]
    #[inline(always)]
    pub const fn cmr_b(&self) -> &CMR_B {
        unsafe { &*(self as *const Self).cast::<u8>().add(4usize).cast() }
    }
    #[doc = "0x0c - Interrupt register"]
    #[inline(always)]
    pub const fn ir_p(&self) -> &IR_P {
        unsafe { &*(self as *const Self).cast::<u8>().add(12usize).cast() }
    }
    #[doc = "0x0c - Interrupt register"]
    #[inline(always)]
    pub const fn ir_b(&self) -> &IR_B {
        unsafe { &*(self as *const Self).cast::<u8>().add(12usize).cast() }
    }
    #[doc = "0x10 - Peli Interrupt Enable register"]
    #[inline(always)]
    pub const fn ier_p(&self) -> &IER_P {
        unsafe { &*(self as *const Self).cast::<u8>().add(16usize).cast() }
    }
    #[doc = "0x10 - Basic Acceptance Code register"]
    #[inline(always)]
    pub const fn group0_acr_b(&self) -> &GROUP0_ACR_B {
        unsafe { &*(self as *const Self).cast::<u8>().add(16usize).cast() }
    }
    #[doc = "0x2c - Peli Arbitration Lost Capture register"]
    #[inline(always)]
    pub const fn alc_p(&self) -> &ALC_P {
        unsafe { &*(self as *const Self).cast::<u8>().add(44usize).cast() }
    }
    #[doc = "0x2c - Basic TX ID register 1"]
    #[inline(always)]
    pub const fn txid1_b(&self) -> &TXID1_B {
        unsafe { &*(self as *const Self).cast::<u8>().add(44usize).cast() }
    }
    #[doc = "0x30 - Peli Error Code Capture register"]
    #[inline(always)]
    pub const fn ecc_p(&self) -> &ECC_P {
        unsafe { &*(self as *const Self).cast::<u8>().add(48usize).cast() }
    }
    #[doc = "0x30 - Basic TX Data register 0"]
    #[inline(always)]
    pub const fn txdr0_b(&self) -> &TXDR0_B {
        unsafe { &*(self as *const Self).cast::<u8>().add(48usize).cast() }
    }
    #[doc = "0x34 - Peli Error Warning Limit register"]
    #[inline(always)]
    pub const fn ewlr_p(&self) -> &EWLR_P {
        unsafe { &*(self as *const Self).cast::<u8>().add(52usize).cast() }
    }
    #[doc = "0x34 - Basic TX Data register 1"]
    #[inline(always)]
    pub const fn txdr1_b(&self) -> &TXDR1_B {
        unsafe { &*(self as *const Self).cast::<u8>().add(52usize).cast() }
    }
    #[doc = "0x38 - Peli RX Error Counter register"]
    #[inline(always)]
    pub const fn rxerr_p(&self) -> &RXERR_P {
        unsafe { &*(self as *const Self).cast::<u8>().add(56usize).cast() }
    }
    #[doc = "0x38 - Basic Send Data register 2"]
    #[inline(always)]
    pub const fn txdr2_b(&self) -> &TXDR2_B {
        unsafe { &*(self as *const Self).cast::<u8>().add(56usize).cast() }
    }
    #[doc = "0x3c - Peli TX Error Counter register"]
    #[inline(always)]
    pub const fn txerr_p(&self) -> &TXERR_P {
        unsafe { &*(self as *const Self).cast::<u8>().add(60usize).cast() }
    }
    #[doc = "0x3c - Basic TX Data register 3"]
    #[inline(always)]
    pub const fn txdr3_b(&self) -> &TXDR3_B {
        unsafe { &*(self as *const Self).cast::<u8>().add(60usize).cast() }
    }
    #[doc = "0x40 - Peli Acceptance Code register 0"]
    #[inline(always)]
    pub const fn group0_acr0_p(&self) -> &GROUP0_ACR0_P {
        unsafe { &*(self as *const Self).cast::<u8>().add(64usize).cast() }
    }
    #[doc = "0x40 - Peli Send Frame Format register"]
    #[inline(always)]
    pub const fn sff_p(&self) -> &SFF_P {
        unsafe { &*(self as *const Self).cast::<u8>().add(64usize).cast() }
    }
    #[doc = "0x40 - Basic TX Data register 4"]
    #[inline(always)]
    pub const fn txdr4_b(&self) -> &TXDR4_B {
        unsafe { &*(self as *const Self).cast::<u8>().add(64usize).cast() }
    }
    #[doc = "0x44 - Peli Acceptance Code register 1"]
    #[inline(always)]
    pub const fn group0_acr1_p(&self) -> &GROUP0_ACR1_P {
        unsafe { &*(self as *const Self).cast::<u8>().add(68usize).cast() }
    }
    #[doc = "0x44 - Peli TX ID register 0"]
    #[inline(always)]
    pub const fn txid0_p(&self) -> &TXID0_P {
        unsafe { &*(self as *const Self).cast::<u8>().add(68usize).cast() }
    }
    #[doc = "0x44 - Basic TX Data register 5"]
    #[inline(always)]
    pub const fn txdr5_b(&self) -> &TXDR5_B {
        unsafe { &*(self as *const Self).cast::<u8>().add(68usize).cast() }
    }
    #[doc = "0x48 - Peli Acceptance Code register 2"]
    #[inline(always)]
    pub const fn group0_acr2_p(&self) -> &GROUP0_ACR2_P {
        unsafe { &*(self as *const Self).cast::<u8>().add(72usize).cast() }
    }
    #[doc = "0x48 - Peli TX ID register 1"]
    #[inline(always)]
    pub const fn txid1_p(&self) -> &TXID1_P {
        unsafe { &*(self as *const Self).cast::<u8>().add(72usize).cast() }
    }
    #[doc = "0x48 - Basic TX Data register 6"]
    #[inline(always)]
    pub const fn txdr6_b(&self) -> &TXDR6_B {
        unsafe { &*(self as *const Self).cast::<u8>().add(72usize).cast() }
    }
    #[doc = "0x4c - Peli Acceptance Code register 3"]
    #[inline(always)]
    pub const fn group0_acr3_p(&self) -> &GROUP0_ACR3_P {
        unsafe { &*(self as *const Self).cast::<u8>().add(76usize).cast() }
    }
    #[doc = "0x4c - Peli TX Data register 0"]
    #[inline(always)]
    pub const fn txdata0_p(&self) -> &TXDATA0_P {
        unsafe { &*(self as *const Self).cast::<u8>().add(76usize).cast() }
    }
    #[doc = "0x4c - Basic TX Data register 7"]
    #[inline(always)]
    pub const fn txdr7_b(&self) -> &TXDR7_B {
        unsafe { &*(self as *const Self).cast::<u8>().add(76usize).cast() }
    }
    #[doc = "0x50 - Peli Acceptance Mask register 0"]
    #[inline(always)]
    pub const fn group0_amr0_p(&self) -> &GROUP0_AMR0_P {
        unsafe { &*(self as *const Self).cast::<u8>().add(80usize).cast() }
    }
    #[doc = "0x50 - Peli TX Data register 1"]
    #[inline(always)]
    pub const fn txdata1_p(&self) -> &TXDATA1_P {
        unsafe { &*(self as *const Self).cast::<u8>().add(80usize).cast() }
    }
    #[doc = "0x54 - Peli Acceptance Mask register 1"]
    #[inline(always)]
    pub const fn group0_amr1_p(&self) -> &GROUP0_AMR1_P {
        unsafe { &*(self as *const Self).cast::<u8>().add(84usize).cast() }
    }
    #[doc = "0x54 - Peli TX Data register 2"]
    #[inline(always)]
    pub const fn txdata2_p(&self) -> &TXDATA2_P {
        unsafe { &*(self as *const Self).cast::<u8>().add(84usize).cast() }
    }
    #[doc = "0x58 - Peli Acceptance Mask register 2"]
    #[inline(always)]
    pub const fn group0_amr2_p(&self) -> &GROUP0_AMR2_P {
        unsafe { &*(self as *const Self).cast::<u8>().add(88usize).cast() }
    }
    #[doc = "0x58 - Peli TX Data register 3"]
    #[inline(always)]
    pub const fn txdata3_p(&self) -> &TXDATA3_P {
        unsafe { &*(self as *const Self).cast::<u8>().add(88usize).cast() }
    }
    #[doc = "0x5c - Peli Acceptance Mask register 3"]
    #[inline(always)]
    pub const fn group0_amr3_p(&self) -> &GROUP0_AMR3_P {
        unsafe { &*(self as *const Self).cast::<u8>().add(92usize).cast() }
    }
    #[doc = "0x5c - Peli TX Data register 4"]
    #[inline(always)]
    pub const fn txdata4_p(&self) -> &TXDATA4_P {
        unsafe { &*(self as *const Self).cast::<u8>().add(92usize).cast() }
    }
    #[doc = "0x98 - Peli Acceptance Code register0"]
    #[inline(always)]
    pub const fn group1_acr0_p(&self) -> &GROUP1_ACR0_P {
        unsafe { &*(self as *const Self).cast::<u8>().add(152usize).cast() }
    }
    #[doc = "0x98 - Basic Acceptance Code register"]
    #[inline(always)]
    pub const fn group1_acr_b(&self) -> &GROUP1_ACR_B {
        unsafe { &*(self as *const Self).cast::<u8>().add(152usize).cast() }
    }
    #[doc = "0xa8 - Peli Acceptance Mask register 0"]
    #[inline(always)]
    pub const fn group1_amr0_p(&self) -> &GROUP1_AMR0_P {
        unsafe { &*(self as *const Self).cast::<u8>().add(168usize).cast() }
    }
    #[doc = "0xa8 - Basic Acceptance Mask register"]
    #[inline(always)]
    pub const fn group1_amr_b(&self) -> &GROUP1_AMR_B {
        unsafe { &*(self as *const Self).cast::<u8>().add(168usize).cast() }
    }
    #[doc = "0xb8 - Peli Acceptance Code register"]
    #[inline(always)]
    pub const fn group2_acr0_p(&self) -> &GROUP2_ACR0_P {
        unsafe { &*(self as *const Self).cast::<u8>().add(184usize).cast() }
    }
    #[doc = "0xb8 - Basic Acceptance Code register"]
    #[inline(always)]
    pub const fn group2_acr_b(&self) -> &GROUP2_ACR_B {
        unsafe { &*(self as *const Self).cast::<u8>().add(184usize).cast() }
    }
    #[doc = "0xc8 - Peli Acceptance Mask register 0"]
    #[inline(always)]
    pub const fn group2_amr0_p(&self) -> &GROUP2_AMR0_P {
        unsafe { &*(self as *const Self).cast::<u8>().add(200usize).cast() }
    }
    #[doc = "0xc8 - Basic Acceptance Mask register"]
    #[inline(always)]
    pub const fn group2_amr_b(&self) -> &GROUP2_AMR_B {
        unsafe { &*(self as *const Self).cast::<u8>().add(200usize).cast() }
    }
    #[doc = "0xd8 - Peli Acceptance Code register0"]
    #[inline(always)]
    pub const fn group3_acr0_p(&self) -> &GROUP3_ACR0_P {
        unsafe { &*(self as *const Self).cast::<u8>().add(216usize).cast() }
    }
    #[doc = "0xd8 - Basic Acceptance Code register"]
    #[inline(always)]
    pub const fn group3_acr_b(&self) -> &GROUP3_ACR_B {
        unsafe { &*(self as *const Self).cast::<u8>().add(216usize).cast() }
    }
    #[doc = "0xe8 - Peli Acceptance Mask register 0"]
    #[inline(always)]
    pub const fn group3_amr0_p(&self) -> &GROUP3_AMR0_P {
        unsafe { &*(self as *const Self).cast::<u8>().add(232usize).cast() }
    }
    #[doc = "0xe8 - Basic Acceptance Mask register"]
    #[inline(always)]
    pub const fn group3_amr_b(&self) -> &GROUP3_AMR_B {
        unsafe { &*(self as *const Self).cast::<u8>().add(232usize).cast() }
    }
    #[doc = "0xf8 - Peli Acceptance Code register"]
    #[inline(always)]
    pub const fn group4_acr0_p(&self) -> &GROUP4_ACR0_P {
        unsafe { &*(self as *const Self).cast::<u8>().add(248usize).cast() }
    }
    #[doc = "0xf8 - Basic Acceptance Code register"]
    #[inline(always)]
    pub const fn group4_acr_b(&self) -> &GROUP4_ACR_B {
        unsafe { &*(self as *const Self).cast::<u8>().add(248usize).cast() }
    }
    #[doc = "0x108 - Peli Acceptance Mask register 0"]
    #[inline(always)]
    pub const fn group4_amr0_p(&self) -> &GROUP4_AMR0_P {
        unsafe { &*(self as *const Self).cast::<u8>().add(264usize).cast() }
    }
    #[doc = "0x108 - Basic Acceptance Mask register"]
    #[inline(always)]
    pub const fn group4_amr_b(&self) -> &GROUP4_AMR_B {
        unsafe { &*(self as *const Self).cast::<u8>().add(264usize).cast() }
    }
    #[doc = "0x118 - Peli Acceptance Code register0"]
    #[inline(always)]
    pub const fn group5_acr0_p(&self) -> &GROUP5_ACR0_P {
        unsafe { &*(self as *const Self).cast::<u8>().add(280usize).cast() }
    }
    #[doc = "0x118 - Basic Acceptance Code register"]
    #[inline(always)]
    pub const fn group5_acr_b(&self) -> &GROUP5_ACR_B {
        unsafe { &*(self as *const Self).cast::<u8>().add(280usize).cast() }
    }
    #[doc = "0x128 - Peli Acceptance Mask register 0"]
    #[inline(always)]
    pub const fn group5_amr0_p(&self) -> &GROUP5_AMR0_P {
        unsafe { &*(self as *const Self).cast::<u8>().add(296usize).cast() }
    }
    #[doc = "0x128 - Basic Acceptance Mask register"]
    #[inline(always)]
    pub const fn group5_amr_b(&self) -> &GROUP5_AMR_B {
        unsafe { &*(self as *const Self).cast::<u8>().add(296usize).cast() }
    }
    #[doc = "0x138 - Peli Acceptance Code register"]
    #[inline(always)]
    pub const fn group6_acr0_p(&self) -> &GROUP6_ACR0_P {
        unsafe { &*(self as *const Self).cast::<u8>().add(312usize).cast() }
    }
    #[doc = "0x138 - Basic Acceptance Code register"]
    #[inline(always)]
    pub const fn group6_acr_b(&self) -> &GROUP6_ACR_B {
        unsafe { &*(self as *const Self).cast::<u8>().add(312usize).cast() }
    }
    #[doc = "0x148 - Peli Acceptance Mask register 0"]
    #[inline(always)]
    pub const fn group6_amr0_p(&self) -> &GROUP6_AMR0_P {
        unsafe { &*(self as *const Self).cast::<u8>().add(328usize).cast() }
    }
    #[doc = "0x148 - Basic Acceptance Mask register"]
    #[inline(always)]
    pub const fn group6_amr_b(&self) -> &GROUP6_AMR_B {
        unsafe { &*(self as *const Self).cast::<u8>().add(328usize).cast() }
    }
    #[doc = "0x158 - Peli Acceptance Code register0"]
    #[inline(always)]
    pub const fn group7_acr0_p(&self) -> &GROUP7_ACR0_P {
        unsafe { &*(self as *const Self).cast::<u8>().add(344usize).cast() }
    }
    #[doc = "0x158 - Basic Acceptance Code register"]
    #[inline(always)]
    pub const fn group7_acr_b(&self) -> &GROUP7_ACR_B {
        unsafe { &*(self as *const Self).cast::<u8>().add(344usize).cast() }
    }
    #[doc = "0x168 - Peli Acceptance Mask register 0"]
    #[inline(always)]
    pub const fn group7_amr0_p(&self) -> &GROUP7_AMR0_P {
        unsafe { &*(self as *const Self).cast::<u8>().add(360usize).cast() }
    }
    #[doc = "0x168 - Basic Acceptance Mask register"]
    #[inline(always)]
    pub const fn group7_amr_b(&self) -> &GROUP7_AMR_B {
        unsafe { &*(self as *const Self).cast::<u8>().add(360usize).cast() }
    }
    #[doc = "0x178 - Peli Acceptance Code register"]
    #[inline(always)]
    pub const fn group8_acr0_p(&self) -> &GROUP8_ACR0_P {
        unsafe { &*(self as *const Self).cast::<u8>().add(376usize).cast() }
    }
    #[doc = "0x178 - Basic Acceptance Code register"]
    #[inline(always)]
    pub const fn group8_acr_b(&self) -> &GROUP8_ACR_B {
        unsafe { &*(self as *const Self).cast::<u8>().add(376usize).cast() }
    }
    #[doc = "0x188 - Peli Acceptance Mask register 0"]
    #[inline(always)]
    pub const fn group8_amr0_p(&self) -> &GROUP8_AMR0_P {
        unsafe { &*(self as *const Self).cast::<u8>().add(392usize).cast() }
    }
    #[doc = "0x188 - Basic Acceptance Mask register"]
    #[inline(always)]
    pub const fn group8_amr_b(&self) -> &GROUP8_AMR_B {
        unsafe { &*(self as *const Self).cast::<u8>().add(392usize).cast() }
    }
    #[doc = "0x198 - Peli Acceptance Code register0"]
    #[inline(always)]
    pub const fn group9_acr0_p(&self) -> &GROUP9_ACR0_P {
        unsafe { &*(self as *const Self).cast::<u8>().add(408usize).cast() }
    }
    #[doc = "0x198 - Basic Acceptance Code register"]
    #[inline(always)]
    pub const fn group9_acr_b(&self) -> &GROUP9_ACR_B {
        unsafe { &*(self as *const Self).cast::<u8>().add(408usize).cast() }
    }
    #[doc = "0x1a8 - Peli Acceptance Mask register 0"]
    #[inline(always)]
    pub const fn group9_amr0_p(&self) -> &GROUP9_AMR0_P {
        unsafe { &*(self as *const Self).cast::<u8>().add(424usize).cast() }
    }
    #[doc = "0x1a8 - Basic Acceptance Mask register"]
    #[inline(always)]
    pub const fn group9_amr_b(&self) -> &GROUP9_AMR_B {
        unsafe { &*(self as *const Self).cast::<u8>().add(424usize).cast() }
    }
    #[doc = "0x1b8 - Peli Acceptance Code register"]
    #[inline(always)]
    pub const fn group10_acr0_p(&self) -> &GROUP10_ACR0_P {
        unsafe { &*(self as *const Self).cast::<u8>().add(440usize).cast() }
    }
    #[doc = "0x1b8 - Basic Acceptance Code register"]
    #[inline(always)]
    pub const fn group10_acr_b(&self) -> &GROUP10_ACR_B {
        unsafe { &*(self as *const Self).cast::<u8>().add(440usize).cast() }
    }
    #[doc = "0x1c8 - Peli Acceptance Mask register 0"]
    #[inline(always)]
    pub const fn group10_amr0_p(&self) -> &GROUP10_AMR0_P {
        unsafe { &*(self as *const Self).cast::<u8>().add(456usize).cast() }
    }
    #[doc = "0x1c8 - Basic Acceptance Mask register"]
    #[inline(always)]
    pub const fn group10_amr_b(&self) -> &GROUP10_AMR_B {
        unsafe { &*(self as *const Self).cast::<u8>().add(456usize).cast() }
    }
    #[doc = "0x1d8 - Peli Acceptance Code register0"]
    #[inline(always)]
    pub const fn group11_acr0_p(&self) -> &GROUP11_ACR0_P {
        unsafe { &*(self as *const Self).cast::<u8>().add(472usize).cast() }
    }
    #[doc = "0x1d8 - Basic Acceptance Code register"]
    #[inline(always)]
    pub const fn group11_acr_b(&self) -> &GROUP11_ACR_B {
        unsafe { &*(self as *const Self).cast::<u8>().add(472usize).cast() }
    }
    #[doc = "0x1e8 - Peli Acceptance Mask register 0"]
    #[inline(always)]
    pub const fn group11_amr0_p(&self) -> &GROUP11_AMR0_P {
        unsafe { &*(self as *const Self).cast::<u8>().add(488usize).cast() }
    }
    #[doc = "0x1e8 - Basic Acceptance Mask register"]
    #[inline(always)]
    pub const fn group11_amr_b(&self) -> &GROUP11_AMR_B {
        unsafe { &*(self as *const Self).cast::<u8>().add(488usize).cast() }
    }
    #[doc = "0x1f8 - Peli Acceptance Code register"]
    #[inline(always)]
    pub const fn group12_acr0_p(&self) -> &GROUP12_ACR0_P {
        unsafe { &*(self as *const Self).cast::<u8>().add(504usize).cast() }
    }
    #[doc = "0x1f8 - Basic Acceptance Code register"]
    #[inline(always)]
    pub const fn group12_acr_b(&self) -> &GROUP12_ACR_B {
        unsafe { &*(self as *const Self).cast::<u8>().add(504usize).cast() }
    }
    #[doc = "0x208 - Peli Acceptance Mask register 0"]
    #[inline(always)]
    pub const fn group12_amr0_p(&self) -> &GROUP12_AMR0_P {
        unsafe { &*(self as *const Self).cast::<u8>().add(520usize).cast() }
    }
    #[doc = "0x208 - Basic Acceptance Mask register"]
    #[inline(always)]
    pub const fn group12_amr_b(&self) -> &GROUP12_AMR_B {
        unsafe { &*(self as *const Self).cast::<u8>().add(520usize).cast() }
    }
    #[doc = "0x218 - Peli Acceptance Code register0"]
    #[inline(always)]
    pub const fn group13_acr0_p(&self) -> &GROUP13_ACR0_P {
        unsafe { &*(self as *const Self).cast::<u8>().add(536usize).cast() }
    }
    #[doc = "0x218 - Basic Acceptance Code register"]
    #[inline(always)]
    pub const fn group13_acr_b(&self) -> &GROUP13_ACR_B {
        unsafe { &*(self as *const Self).cast::<u8>().add(536usize).cast() }
    }
    #[doc = "0x228 - Peli Acceptance Mask register 0"]
    #[inline(always)]
    pub const fn group13_amr0_p(&self) -> &GROUP13_AMR0_P {
        unsafe { &*(self as *const Self).cast::<u8>().add(552usize).cast() }
    }
    #[doc = "0x228 - Basic Acceptance Mask register"]
    #[inline(always)]
    pub const fn group13_amr_b(&self) -> &GROUP13_AMR_B {
        unsafe { &*(self as *const Self).cast::<u8>().add(552usize).cast() }
    }
    #[doc = "0x238 - Peli Acceptance Code register"]
    #[inline(always)]
    pub const fn group14_acr0_p(&self) -> &GROUP14_ACR0_P {
        unsafe { &*(self as *const Self).cast::<u8>().add(568usize).cast() }
    }
    #[doc = "0x238 - Basic Acceptance Code register"]
    #[inline(always)]
    pub const fn group14_acr_b(&self) -> &GROUP14_ACR_B {
        unsafe { &*(self as *const Self).cast::<u8>().add(568usize).cast() }
    }
    #[doc = "0x248 - Peli Acceptance Mask register 0"]
    #[inline(always)]
    pub const fn group14_amr0_p(&self) -> &GROUP14_AMR0_P {
        unsafe { &*(self as *const Self).cast::<u8>().add(584usize).cast() }
    }
    #[doc = "0x248 - Basic Acceptance Mask register"]
    #[inline(always)]
    pub const fn group14_amr_b(&self) -> &GROUP14_AMR_B {
        unsafe { &*(self as *const Self).cast::<u8>().add(584usize).cast() }
    }
    #[doc = "0x258 - Peli Acceptance Code register0"]
    #[inline(always)]
    pub const fn group15_acr0_p(&self) -> &GROUP15_ACR0_P {
        unsafe { &*(self as *const Self).cast::<u8>().add(600usize).cast() }
    }
    #[doc = "0x258 - Basic Acceptance Code register"]
    #[inline(always)]
    pub const fn group15_acr_b(&self) -> &GROUP15_ACR_B {
        unsafe { &*(self as *const Self).cast::<u8>().add(600usize).cast() }
    }
    #[doc = "0x268 - Peli Acceptance Mask register 0"]
    #[inline(always)]
    pub const fn group15_amr0_p(&self) -> &GROUP15_AMR0_P {
        unsafe { &*(self as *const Self).cast::<u8>().add(616usize).cast() }
    }
    #[doc = "0x268 - Basic Acceptance Mask register"]
    #[inline(always)]
    pub const fn group15_amr_b(&self) -> &GROUP15_AMR_B {
        unsafe { &*(self as *const Self).cast::<u8>().add(616usize).cast() }
    }
    #[doc = "0x278 - Peli Acceptance Code register"]
    #[inline(always)]
    pub const fn group16_acr0_p(&self) -> &GROUP16_ACR0_P {
        unsafe { &*(self as *const Self).cast::<u8>().add(632usize).cast() }
    }
    #[doc = "0x278 - Basic Acceptance Code register"]
    #[inline(always)]
    pub const fn group16_acr_b(&self) -> &GROUP16_ACR_B {
        unsafe { &*(self as *const Self).cast::<u8>().add(632usize).cast() }
    }
    #[doc = "0x288 - Peli Acceptance Mask register 0"]
    #[inline(always)]
    pub const fn group16_amr0_p(&self) -> &GROUP16_AMR0_P {
        unsafe { &*(self as *const Self).cast::<u8>().add(648usize).cast() }
    }
    #[doc = "0x288 - Basic Acceptance Mask register"]
    #[inline(always)]
    pub const fn group16_amr_b(&self) -> &GROUP16_AMR_B {
        unsafe { &*(self as *const Self).cast::<u8>().add(648usize).cast() }
    }
    #[doc = "0x298 - Peli Acceptance Code register0"]
    #[inline(always)]
    pub const fn group17_acr0_p(&self) -> &GROUP17_ACR0_P {
        unsafe { &*(self as *const Self).cast::<u8>().add(664usize).cast() }
    }
    #[doc = "0x298 - Basic Acceptance Code register"]
    #[inline(always)]
    pub const fn group17_acr_b(&self) -> &GROUP17_ACR_B {
        unsafe { &*(self as *const Self).cast::<u8>().add(664usize).cast() }
    }
    #[doc = "0x2a8 - Peli Acceptance Mask register 0"]
    #[inline(always)]
    pub const fn group17_amr0_p(&self) -> &GROUP17_AMR0_P {
        unsafe { &*(self as *const Self).cast::<u8>().add(680usize).cast() }
    }
    #[doc = "0x2a8 - Basic Acceptance Mask register"]
    #[inline(always)]
    pub const fn group17_amr_b(&self) -> &GROUP17_AMR_B {
        unsafe { &*(self as *const Self).cast::<u8>().add(680usize).cast() }
    }
    #[doc = "0x2b8 - Peli Acceptance Code register0"]
    #[inline(always)]
    pub const fn group18_acr0_p(&self) -> &GROUP18_ACR0_P {
        unsafe { &*(self as *const Self).cast::<u8>().add(696usize).cast() }
    }
    #[doc = "0x2b8 - Basic Acceptance Code register"]
    #[inline(always)]
    pub const fn group18_acr_b(&self) -> &GROUP18_ACR_B {
        unsafe { &*(self as *const Self).cast::<u8>().add(696usize).cast() }
    }
    #[doc = "0x2c8 - Peli Acceptance Mask register 0"]
    #[inline(always)]
    pub const fn group18_amr0_p(&self) -> &GROUP18_AMR0_P {
        unsafe { &*(self as *const Self).cast::<u8>().add(712usize).cast() }
    }
    #[doc = "0x2c8 - Basic Acceptance Mask register"]
    #[inline(always)]
    pub const fn group18_amr_b(&self) -> &GROUP18_AMR_B {
        unsafe { &*(self as *const Self).cast::<u8>().add(712usize).cast() }
    }
    #[doc = "0x2d8 - Peli Acceptance Code register0"]
    #[inline(always)]
    pub const fn group19_acr0_p(&self) -> &GROUP19_ACR0_P {
        unsafe { &*(self as *const Self).cast::<u8>().add(728usize).cast() }
    }
    #[doc = "0x2d8 - Basic Acceptance Code register"]
    #[inline(always)]
    pub const fn group19_acr_b(&self) -> &GROUP19_ACR_B {
        unsafe { &*(self as *const Self).cast::<u8>().add(728usize).cast() }
    }
    #[doc = "0x2e8 - Peli Acceptance Mask register 0"]
    #[inline(always)]
    pub const fn group19_amr0_p(&self) -> &GROUP19_AMR0_P {
        unsafe { &*(self as *const Self).cast::<u8>().add(744usize).cast() }
    }
    #[doc = "0x2e8 - Basic Acceptance Mask register"]
    #[inline(always)]
    pub const fn group19_amr_b(&self) -> &GROUP19_AMR_B {
        unsafe { &*(self as *const Self).cast::<u8>().add(744usize).cast() }
    }
}
#[doc = "CR_B (rw) register accessor: an alias for `Reg<CR_B_SPEC>`"]
pub type CR_B = crate::Reg<cr_b::CR_B_SPEC>;
#[doc = "Basic control register"]
pub mod cr_b;
#[doc = "MOD_P (rw) register accessor: an alias for `Reg<MOD_P_SPEC>`"]
pub type MOD_P = crate::Reg<mod_p::MOD_P_SPEC>;
#[doc = "Peli Mode register"]
pub mod mod_p;
#[doc = "CMR_B (w) register accessor: an alias for `Reg<CMR_B_SPEC>`"]
pub type CMR_B = crate::Reg<cmr_b::CMR_B_SPEC>;
#[doc = "Basic Command register"]
pub mod cmr_b;
#[doc = "CMR_P (w) register accessor: an alias for `Reg<CMR_P_SPEC>`"]
pub type CMR_P = crate::Reg<cmr_p::CMR_P_SPEC>;
#[doc = "Peli Command register"]
pub mod cmr_p;
#[doc = "SR (rw) register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "Status register"]
pub mod sr;
#[doc = "IR_B (r) register accessor: an alias for `Reg<IR_B_SPEC>`"]
pub type IR_B = crate::Reg<ir_b::IR_B_SPEC>;
#[doc = "Interrupt register"]
pub mod ir_b;
#[doc = "IR_P (r) register accessor: an alias for `Reg<IR_P_SPEC>`"]
pub type IR_P = crate::Reg<ir_p::IR_P_SPEC>;
#[doc = "Interrupt register"]
pub mod ir_p;
#[doc = "GROUP0_ACR_B (rw) register accessor: an alias for `Reg<GROUP0_ACR_B_SPEC>`"]
pub type GROUP0_ACR_B = crate::Reg<group0_acr_b::GROUP0_ACR_B_SPEC>;
#[doc = "Basic Acceptance Code register"]
pub mod group0_acr_b;
#[doc = "IER_P (rw) register accessor: an alias for `Reg<IER_P_SPEC>`"]
pub type IER_P = crate::Reg<ier_p::IER_P_SPEC>;
#[doc = "Peli Interrupt Enable register"]
pub mod ier_p;
#[doc = "GROUP0_AMR_B (rw) register accessor: an alias for `Reg<GROUP0_AMR_B_SPEC>`"]
pub type GROUP0_AMR_B = crate::Reg<group0_amr_b::GROUP0_AMR_B_SPEC>;
#[doc = "Basic Acceptance Mask register"]
pub mod group0_amr_b;
#[doc = "BTR0 (rw) register accessor: an alias for `Reg<BTR0_SPEC>`"]
pub type BTR0 = crate::Reg<btr0::BTR0_SPEC>;
#[doc = "Bus Timing register 0"]
pub mod btr0;
#[doc = "BTR1 (rw) register accessor: an alias for `Reg<BTR1_SPEC>`"]
pub type BTR1 = crate::Reg<btr1::BTR1_SPEC>;
#[doc = "Bus Timing register 1"]
pub mod btr1;
#[doc = "TXID0_B (rw) register accessor: an alias for `Reg<TXID0_B_SPEC>`"]
pub type TXID0_B = crate::Reg<txid0_b::TXID0_B_SPEC>;
#[doc = "Basic TX ID register 0"]
pub mod txid0_b;
#[doc = "TXID1_B (rw) register accessor: an alias for `Reg<TXID1_B_SPEC>`"]
pub type TXID1_B = crate::Reg<txid1_b::TXID1_B_SPEC>;
#[doc = "Basic TX ID register 1"]
pub mod txid1_b;
#[doc = "ALC_P (rw) register accessor: an alias for `Reg<ALC_P_SPEC>`"]
pub type ALC_P = crate::Reg<alc_p::ALC_P_SPEC>;
#[doc = "Peli Arbitration Lost Capture register"]
pub mod alc_p;
#[doc = "TXDR0_B (rw) register accessor: an alias for `Reg<TXDR0_B_SPEC>`"]
pub type TXDR0_B = crate::Reg<txdr0_b::TXDR0_B_SPEC>;
#[doc = "Basic TX Data register 0"]
pub mod txdr0_b;
#[doc = "ECC_P (r) register accessor: an alias for `Reg<ECC_P_SPEC>`"]
pub type ECC_P = crate::Reg<ecc_p::ECC_P_SPEC>;
#[doc = "Peli Error Code Capture register"]
pub mod ecc_p;
#[doc = "TXDR1_B (rw) register accessor: an alias for `Reg<TXDR1_B_SPEC>`"]
pub type TXDR1_B = crate::Reg<txdr1_b::TXDR1_B_SPEC>;
#[doc = "Basic TX Data register 1"]
pub mod txdr1_b;
#[doc = "EWLR_P (rw) register accessor: an alias for `Reg<EWLR_P_SPEC>`"]
pub type EWLR_P = crate::Reg<ewlr_p::EWLR_P_SPEC>;
#[doc = "Peli Error Warning Limit register"]
pub mod ewlr_p;
#[doc = "TXDR2_B (rw) register accessor: an alias for `Reg<TXDR2_B_SPEC>`"]
pub type TXDR2_B = crate::Reg<txdr2_b::TXDR2_B_SPEC>;
#[doc = "Basic Send Data register 2"]
pub mod txdr2_b;
#[doc = "RXERR_P (rw) register accessor: an alias for `Reg<RXERR_P_SPEC>`"]
pub type RXERR_P = crate::Reg<rxerr_p::RXERR_P_SPEC>;
#[doc = "Peli RX Error Counter register"]
pub mod rxerr_p;
#[doc = "TXDR3_B (rw) register accessor: an alias for `Reg<TXDR3_B_SPEC>`"]
pub type TXDR3_B = crate::Reg<txdr3_b::TXDR3_B_SPEC>;
#[doc = "Basic TX Data register 3"]
pub mod txdr3_b;
#[doc = "TXERR_P (rw) register accessor: an alias for `Reg<TXERR_P_SPEC>`"]
pub type TXERR_P = crate::Reg<txerr_p::TXERR_P_SPEC>;
#[doc = "Peli TX Error Counter register"]
pub mod txerr_p;
#[doc = "TXDR4_B (rw) register accessor: an alias for `Reg<TXDR4_B_SPEC>`"]
pub type TXDR4_B = crate::Reg<txdr4_b::TXDR4_B_SPEC>;
#[doc = "Basic TX Data register 4"]
pub mod txdr4_b;
#[doc = "SFF_P (rw) register accessor: an alias for `Reg<SFF_P_SPEC>`"]
pub type SFF_P = crate::Reg<sff_p::SFF_P_SPEC>;
#[doc = "Peli Send Frame Format register"]
pub mod sff_p;
#[doc = "GROUP0_ACR0_P (rw) register accessor: an alias for `Reg<GROUP0_ACR0_P_SPEC>`"]
pub type GROUP0_ACR0_P = crate::Reg<group0_acr0_p::GROUP0_ACR0_P_SPEC>;
#[doc = "Peli Acceptance Code register 0"]
pub mod group0_acr0_p;
#[doc = "TXDR5_B (rw) register accessor: an alias for `Reg<TXDR5_B_SPEC>`"]
pub type TXDR5_B = crate::Reg<txdr5_b::TXDR5_B_SPEC>;
#[doc = "Basic TX Data register 5"]
pub mod txdr5_b;
#[doc = "TXID0_P (rw) register accessor: an alias for `Reg<TXID0_P_SPEC>`"]
pub type TXID0_P = crate::Reg<txid0_p::TXID0_P_SPEC>;
#[doc = "Peli TX ID register 0"]
pub mod txid0_p;
#[doc = "GROUP0_ACR1_P (rw) register accessor: an alias for `Reg<GROUP0_ACR1_P_SPEC>`"]
pub type GROUP0_ACR1_P = crate::Reg<group0_acr1_p::GROUP0_ACR1_P_SPEC>;
#[doc = "Peli Acceptance Code register 1"]
pub mod group0_acr1_p;
#[doc = "TXDR6_B (rw) register accessor: an alias for `Reg<TXDR6_B_SPEC>`"]
pub type TXDR6_B = crate::Reg<txdr6_b::TXDR6_B_SPEC>;
#[doc = "Basic TX Data register 6"]
pub mod txdr6_b;
#[doc = "TXID1_P (rw) register accessor: an alias for `Reg<TXID1_P_SPEC>`"]
pub type TXID1_P = crate::Reg<txid1_p::TXID1_P_SPEC>;
#[doc = "Peli TX ID register 1"]
pub mod txid1_p;
#[doc = "GROUP0_ACR2_P (rw) register accessor: an alias for `Reg<GROUP0_ACR2_P_SPEC>`"]
pub type GROUP0_ACR2_P = crate::Reg<group0_acr2_p::GROUP0_ACR2_P_SPEC>;
#[doc = "Peli Acceptance Code register 2"]
pub mod group0_acr2_p;
#[doc = "TXDR7_B (rw) register accessor: an alias for `Reg<TXDR7_B_SPEC>`"]
pub type TXDR7_B = crate::Reg<txdr7_b::TXDR7_B_SPEC>;
#[doc = "Basic TX Data register 7"]
pub mod txdr7_b;
#[doc = "TXDATA0_P (rw) register accessor: an alias for `Reg<TXDATA0_P_SPEC>`"]
pub type TXDATA0_P = crate::Reg<txdata0_p::TXDATA0_P_SPEC>;
#[doc = "Peli TX Data register 0"]
pub mod txdata0_p;
#[doc = "GROUP0_ACR3_P (rw) register accessor: an alias for `Reg<GROUP0_ACR3_P_SPEC>`"]
pub type GROUP0_ACR3_P = crate::Reg<group0_acr3_p::GROUP0_ACR3_P_SPEC>;
#[doc = "Peli Acceptance Code register 3"]
pub mod group0_acr3_p;
#[doc = "TXDATA1_P (rw) register accessor: an alias for `Reg<TXDATA1_P_SPEC>`"]
pub type TXDATA1_P = crate::Reg<txdata1_p::TXDATA1_P_SPEC>;
#[doc = "Peli TX Data register 1"]
pub mod txdata1_p;
#[doc = "GROUP0_AMR0_P (rw) register accessor: an alias for `Reg<GROUP0_AMR0_P_SPEC>`"]
pub type GROUP0_AMR0_P = crate::Reg<group0_amr0_p::GROUP0_AMR0_P_SPEC>;
#[doc = "Peli Acceptance Mask register 0"]
pub mod group0_amr0_p;
#[doc = "TXDATA2_P (rw) register accessor: an alias for `Reg<TXDATA2_P_SPEC>`"]
pub type TXDATA2_P = crate::Reg<txdata2_p::TXDATA2_P_SPEC>;
#[doc = "Peli TX Data register 2"]
pub mod txdata2_p;
#[doc = "GROUP0_AMR1_P (rw) register accessor: an alias for `Reg<GROUP0_AMR1_P_SPEC>`"]
pub type GROUP0_AMR1_P = crate::Reg<group0_amr1_p::GROUP0_AMR1_P_SPEC>;
#[doc = "Peli Acceptance Mask register 1"]
pub mod group0_amr1_p;
#[doc = "TXDATA3_P (rw) register accessor: an alias for `Reg<TXDATA3_P_SPEC>`"]
pub type TXDATA3_P = crate::Reg<txdata3_p::TXDATA3_P_SPEC>;
#[doc = "Peli TX Data register 3"]
pub mod txdata3_p;
#[doc = "GROUP0_AMR2_P (rw) register accessor: an alias for `Reg<GROUP0_AMR2_P_SPEC>`"]
pub type GROUP0_AMR2_P = crate::Reg<group0_amr2_p::GROUP0_AMR2_P_SPEC>;
#[doc = "Peli Acceptance Mask register 2"]
pub mod group0_amr2_p;
#[doc = "TXDATA4_P (rw) register accessor: an alias for `Reg<TXDATA4_P_SPEC>`"]
pub type TXDATA4_P = crate::Reg<txdata4_p::TXDATA4_P_SPEC>;
#[doc = "Peli TX Data register 4"]
pub mod txdata4_p;
#[doc = "GROUP0_AMR3_P (rw) register accessor: an alias for `Reg<GROUP0_AMR3_P_SPEC>`"]
pub type GROUP0_AMR3_P = crate::Reg<group0_amr3_p::GROUP0_AMR3_P_SPEC>;
#[doc = "Peli Acceptance Mask register 3"]
pub mod group0_amr3_p;
#[doc = "TXDATA5_P (rw) register accessor: an alias for `Reg<TXDATA5_P_SPEC>`"]
pub type TXDATA5_P = crate::Reg<txdata5_p::TXDATA5_P_SPEC>;
#[doc = "Peli TX Data register 5"]
pub mod txdata5_p;
#[doc = "TXDATA6_P (rw) register accessor: an alias for `Reg<TXDATA6_P_SPEC>`"]
pub type TXDATA6_P = crate::Reg<txdata6_p::TXDATA6_P_SPEC>;
#[doc = "Peli TX Data register 6"]
pub mod txdata6_p;
#[doc = "TXDATA7_P (rw) register accessor: an alias for `Reg<TXDATA7_P_SPEC>`"]
pub type TXDATA7_P = crate::Reg<txdata7_p::TXDATA7_P_SPEC>;
#[doc = "Peli TX Data register 7"]
pub mod txdata7_p;
#[doc = "TXDATA8_P (rw) register accessor: an alias for `Reg<TXDATA8_P_SPEC>`"]
pub type TXDATA8_P = crate::Reg<txdata8_p::TXDATA8_P_SPEC>;
#[doc = "Peli TX Data register 8"]
pub mod txdata8_p;
#[doc = "TXDATA9_P (rw) register accessor: an alias for `Reg<TXDATA9_P_SPEC>`"]
pub type TXDATA9_P = crate::Reg<txdata9_p::TXDATA9_P_SPEC>;
#[doc = "Peli TX Data register 9"]
pub mod txdata9_p;
#[doc = "CDR (rw) register accessor: an alias for `Reg<CDR_SPEC>`"]
pub type CDR = crate::Reg<cdr::CDR_SPEC>;
#[doc = "Clock Divider register"]
pub mod cdr;
#[doc = "AFM0 (rw) register accessor: an alias for `Reg<AFM0_SPEC>`"]
pub type AFM0 = crate::Reg<afm0::AFM0_SPEC>;
#[doc = "Filter Mode register 0"]
pub mod afm0;
#[doc = "AFM1 (rw) register accessor: an alias for `Reg<AFM1_SPEC>`"]
pub type AFM1 = crate::Reg<afm1::AFM1_SPEC>;
#[doc = "Filter Mode register 1"]
pub mod afm1;
#[doc = "AFM2 (rw) register accessor: an alias for `Reg<AFM2_SPEC>`"]
pub type AFM2 = crate::Reg<afm2::AFM2_SPEC>;
#[doc = "Filter Mode register 2"]
pub mod afm2;
#[doc = "FGA0 (rw) register accessor: an alias for `Reg<FGA0_SPEC>`"]
pub type FGA0 = crate::Reg<fga0::FGA0_SPEC>;
#[doc = "Filter Group Enable Register 0"]
pub mod fga0;
#[doc = "FGA1 (rw) register accessor: an alias for `Reg<FGA1_SPEC>`"]
pub type FGA1 = crate::Reg<fga1::FGA1_SPEC>;
#[doc = "Filter Group Enable Register 1"]
pub mod fga1;
#[doc = "FGA2 (rw) register accessor: an alias for `Reg<FGA2_SPEC>`"]
pub type FGA2 = crate::Reg<fga2::FGA2_SPEC>;
#[doc = "Filter Group Enable Register 2"]
pub mod fga2;
#[doc = "GROUP1_ACR_B (rw) register accessor: an alias for `Reg<GROUP1_ACR_B_SPEC>`"]
pub type GROUP1_ACR_B = crate::Reg<group1_acr_b::GROUP1_ACR_B_SPEC>;
#[doc = "Basic Acceptance Code register"]
pub mod group1_acr_b;
#[doc = "GROUP1_ACR0_P (rw) register accessor: an alias for `Reg<GROUP1_ACR0_P_SPEC>`"]
pub type GROUP1_ACR0_P = crate::Reg<group1_acr0_p::GROUP1_ACR0_P_SPEC>;
#[doc = "Peli Acceptance Code register0"]
pub mod group1_acr0_p;
#[doc = "GROUP1_ACR1_P (rw) register accessor: an alias for `Reg<GROUP1_ACR1_P_SPEC>`"]
pub type GROUP1_ACR1_P = crate::Reg<group1_acr1_p::GROUP1_ACR1_P_SPEC>;
#[doc = "Peli Acceptance Code register1"]
pub mod group1_acr1_p;
#[doc = "GROUP1_ACR2_P (rw) register accessor: an alias for `Reg<GROUP1_ACR2_P_SPEC>`"]
pub type GROUP1_ACR2_P = crate::Reg<group1_acr2_p::GROUP1_ACR2_P_SPEC>;
#[doc = "Peli Acceptance Code register2"]
pub mod group1_acr2_p;
#[doc = "GROUP1_ACR3_P (rw) register accessor: an alias for `Reg<GROUP1_ACR3_P_SPEC>`"]
pub type GROUP1_ACR3_P = crate::Reg<group1_acr3_p::GROUP1_ACR3_P_SPEC>;
#[doc = "Peli Acceptance Code register3"]
pub mod group1_acr3_p;
#[doc = "GROUP1_AMR_B (rw) register accessor: an alias for `Reg<GROUP1_AMR_B_SPEC>`"]
pub type GROUP1_AMR_B = crate::Reg<group1_amr_b::GROUP1_AMR_B_SPEC>;
#[doc = "Basic Acceptance Mask register"]
pub mod group1_amr_b;
#[doc = "GROUP1_AMR0_P (rw) register accessor: an alias for `Reg<GROUP1_AMR0_P_SPEC>`"]
pub type GROUP1_AMR0_P = crate::Reg<group1_amr0_p::GROUP1_AMR0_P_SPEC>;
#[doc = "Peli Acceptance Mask register 0"]
pub mod group1_amr0_p;
#[doc = "GROUP1_AMR1_P (rw) register accessor: an alias for `Reg<GROUP1_AMR1_P_SPEC>`"]
pub type GROUP1_AMR1_P = crate::Reg<group1_amr1_p::GROUP1_AMR1_P_SPEC>;
#[doc = "Peli Acceptance Mask register 1"]
pub mod group1_amr1_p;
#[doc = "GROUP1_AMR2_P (rw) register accessor: an alias for `Reg<GROUP1_AMR2_P_SPEC>`"]
pub type GROUP1_AMR2_P = crate::Reg<group1_amr2_p::GROUP1_AMR2_P_SPEC>;
#[doc = "Peli Acceptance Mask register 2"]
pub mod group1_amr2_p;
#[doc = "GROUP1_AMR3_P (rw) register accessor: an alias for `Reg<GROUP1_AMR3_P_SPEC>`"]
pub type GROUP1_AMR3_P = crate::Reg<group1_amr3_p::GROUP1_AMR3_P_SPEC>;
#[doc = "Peli Acceptance Mask register 3"]
pub mod group1_amr3_p;
#[doc = "GROUP2_ACR_B (rw) register accessor: an alias for `Reg<GROUP2_ACR_B_SPEC>`"]
pub type GROUP2_ACR_B = crate::Reg<group2_acr_b::GROUP2_ACR_B_SPEC>;
#[doc = "Basic Acceptance Code register"]
pub mod group2_acr_b;
#[doc = "GROUP2_ACR0_P (rw) register accessor: an alias for `Reg<GROUP2_ACR0_P_SPEC>`"]
pub type GROUP2_ACR0_P = crate::Reg<group2_acr0_p::GROUP2_ACR0_P_SPEC>;
#[doc = "Peli Acceptance Code register"]
pub mod group2_acr0_p;
#[doc = "GROUP2_ACR1_P (rw) register accessor: an alias for `Reg<GROUP2_ACR1_P_SPEC>`"]
pub type GROUP2_ACR1_P = crate::Reg<group2_acr1_p::GROUP2_ACR1_P_SPEC>;
#[doc = "Peli Acceptance Code register1"]
pub mod group2_acr1_p;
#[doc = "GROUP2_ACR2_P (rw) register accessor: an alias for `Reg<GROUP2_ACR2_P_SPEC>`"]
pub type GROUP2_ACR2_P = crate::Reg<group2_acr2_p::GROUP2_ACR2_P_SPEC>;
#[doc = "Peli Acceptance Code register2"]
pub mod group2_acr2_p;
#[doc = "GROUP2_ACR3_P (rw) register accessor: an alias for `Reg<GROUP2_ACR3_P_SPEC>`"]
pub type GROUP2_ACR3_P = crate::Reg<group2_acr3_p::GROUP2_ACR3_P_SPEC>;
#[doc = "Peli Acceptance Code register3"]
pub mod group2_acr3_p;
#[doc = "GROUP2_AMR_B (rw) register accessor: an alias for `Reg<GROUP2_AMR_B_SPEC>`"]
pub type GROUP2_AMR_B = crate::Reg<group2_amr_b::GROUP2_AMR_B_SPEC>;
#[doc = "Basic Acceptance Mask register"]
pub mod group2_amr_b;
#[doc = "GROUP2_AMR0_P (rw) register accessor: an alias for `Reg<GROUP2_AMR0_P_SPEC>`"]
pub type GROUP2_AMR0_P = crate::Reg<group2_amr0_p::GROUP2_AMR0_P_SPEC>;
#[doc = "Peli Acceptance Mask register 0"]
pub mod group2_amr0_p;
#[doc = "GROUP2_AMR1_P (rw) register accessor: an alias for `Reg<GROUP2_AMR1_P_SPEC>`"]
pub type GROUP2_AMR1_P = crate::Reg<group2_amr1_p::GROUP2_AMR1_P_SPEC>;
#[doc = "Peli Acceptance Mask register 1"]
pub mod group2_amr1_p;
#[doc = "GROUP2_AMR2_P (rw) register accessor: an alias for `Reg<GROUP2_AMR2_P_SPEC>`"]
pub type GROUP2_AMR2_P = crate::Reg<group2_amr2_p::GROUP2_AMR2_P_SPEC>;
#[doc = "Peli Acceptance Mask register 2"]
pub mod group2_amr2_p;
#[doc = "GROUP2_AMR3_P (rw) register accessor: an alias for `Reg<GROUP2_AMR3_P_SPEC>`"]
pub type GROUP2_AMR3_P = crate::Reg<group2_amr3_p::GROUP2_AMR3_P_SPEC>;
#[doc = "Peli Acceptance Mask register 3"]
pub mod group2_amr3_p;
#[doc = "GROUP3_ACR_B (rw) register accessor: an alias for `Reg<GROUP3_ACR_B_SPEC>`"]
pub type GROUP3_ACR_B = crate::Reg<group3_acr_b::GROUP3_ACR_B_SPEC>;
#[doc = "Basic Acceptance Code register"]
pub mod group3_acr_b;
#[doc = "GROUP3_ACR0_P (rw) register accessor: an alias for `Reg<GROUP3_ACR0_P_SPEC>`"]
pub type GROUP3_ACR0_P = crate::Reg<group3_acr0_p::GROUP3_ACR0_P_SPEC>;
#[doc = "Peli Acceptance Code register0"]
pub mod group3_acr0_p;
#[doc = "GROUP3_ACR1_P (rw) register accessor: an alias for `Reg<GROUP3_ACR1_P_SPEC>`"]
pub type GROUP3_ACR1_P = crate::Reg<group3_acr1_p::GROUP3_ACR1_P_SPEC>;
#[doc = "Peli Acceptance Code register1"]
pub mod group3_acr1_p;
#[doc = "GROUP3_ACR2_P (rw) register accessor: an alias for `Reg<GROUP3_ACR2_P_SPEC>`"]
pub type GROUP3_ACR2_P = crate::Reg<group3_acr2_p::GROUP3_ACR2_P_SPEC>;
#[doc = "Peli Acceptance Code register2"]
pub mod group3_acr2_p;
#[doc = "GROUP3_ACR3_P (rw) register accessor: an alias for `Reg<GROUP3_ACR3_P_SPEC>`"]
pub type GROUP3_ACR3_P = crate::Reg<group3_acr3_p::GROUP3_ACR3_P_SPEC>;
#[doc = "Peli Acceptance Code register3"]
pub mod group3_acr3_p;
#[doc = "GROUP3_AMR_B (rw) register accessor: an alias for `Reg<GROUP3_AMR_B_SPEC>`"]
pub type GROUP3_AMR_B = crate::Reg<group3_amr_b::GROUP3_AMR_B_SPEC>;
#[doc = "Basic Acceptance Mask register"]
pub mod group3_amr_b;
#[doc = "GROUP3_AMR0_P (rw) register accessor: an alias for `Reg<GROUP3_AMR0_P_SPEC>`"]
pub type GROUP3_AMR0_P = crate::Reg<group3_amr0_p::GROUP3_AMR0_P_SPEC>;
#[doc = "Peli Acceptance Mask register 0"]
pub mod group3_amr0_p;
#[doc = "GROUP3_AMR1_P (rw) register accessor: an alias for `Reg<GROUP3_AMR1_P_SPEC>`"]
pub type GROUP3_AMR1_P = crate::Reg<group3_amr1_p::GROUP3_AMR1_P_SPEC>;
#[doc = "Peli Acceptance Mask register 1"]
pub mod group3_amr1_p;
#[doc = "GROUP3_AMR2_P (rw) register accessor: an alias for `Reg<GROUP3_AMR2_P_SPEC>`"]
pub type GROUP3_AMR2_P = crate::Reg<group3_amr2_p::GROUP3_AMR2_P_SPEC>;
#[doc = "Peli Acceptance Mask register 2"]
pub mod group3_amr2_p;
#[doc = "GROUP3_AMR3_P (rw) register accessor: an alias for `Reg<GROUP3_AMR3_P_SPEC>`"]
pub type GROUP3_AMR3_P = crate::Reg<group3_amr3_p::GROUP3_AMR3_P_SPEC>;
#[doc = "Peli Acceptance Mask register 3"]
pub mod group3_amr3_p;
#[doc = "GROUP4_ACR_B (rw) register accessor: an alias for `Reg<GROUP4_ACR_B_SPEC>`"]
pub type GROUP4_ACR_B = crate::Reg<group4_acr_b::GROUP4_ACR_B_SPEC>;
#[doc = "Basic Acceptance Code register"]
pub mod group4_acr_b;
#[doc = "GROUP4_ACR0_P (rw) register accessor: an alias for `Reg<GROUP4_ACR0_P_SPEC>`"]
pub type GROUP4_ACR0_P = crate::Reg<group4_acr0_p::GROUP4_ACR0_P_SPEC>;
#[doc = "Peli Acceptance Code register"]
pub mod group4_acr0_p;
#[doc = "GROUP4_ACR1_P (rw) register accessor: an alias for `Reg<GROUP4_ACR1_P_SPEC>`"]
pub type GROUP4_ACR1_P = crate::Reg<group4_acr1_p::GROUP4_ACR1_P_SPEC>;
#[doc = "Peli Acceptance Code register1"]
pub mod group4_acr1_p;
#[doc = "GROUP4_ACR2_P (rw) register accessor: an alias for `Reg<GROUP4_ACR2_P_SPEC>`"]
pub type GROUP4_ACR2_P = crate::Reg<group4_acr2_p::GROUP4_ACR2_P_SPEC>;
#[doc = "Peli Acceptance Code register2"]
pub mod group4_acr2_p;
#[doc = "GROUP4_ACR3_P (rw) register accessor: an alias for `Reg<GROUP4_ACR3_P_SPEC>`"]
pub type GROUP4_ACR3_P = crate::Reg<group4_acr3_p::GROUP4_ACR3_P_SPEC>;
#[doc = "Peli Acceptance Code register3"]
pub mod group4_acr3_p;
#[doc = "GROUP4_AMR_B (rw) register accessor: an alias for `Reg<GROUP4_AMR_B_SPEC>`"]
pub type GROUP4_AMR_B = crate::Reg<group4_amr_b::GROUP4_AMR_B_SPEC>;
#[doc = "Basic Acceptance Mask register"]
pub mod group4_amr_b;
#[doc = "GROUP4_AMR0_P (rw) register accessor: an alias for `Reg<GROUP4_AMR0_P_SPEC>`"]
pub type GROUP4_AMR0_P = crate::Reg<group4_amr0_p::GROUP4_AMR0_P_SPEC>;
#[doc = "Peli Acceptance Mask register 0"]
pub mod group4_amr0_p;
#[doc = "GROUP4_AMR1_P (rw) register accessor: an alias for `Reg<GROUP4_AMR1_P_SPEC>`"]
pub type GROUP4_AMR1_P = crate::Reg<group4_amr1_p::GROUP4_AMR1_P_SPEC>;
#[doc = "Peli Acceptance Mask register 1"]
pub mod group4_amr1_p;
#[doc = "GROUP4_AMR2_P (rw) register accessor: an alias for `Reg<GROUP4_AMR2_P_SPEC>`"]
pub type GROUP4_AMR2_P = crate::Reg<group4_amr2_p::GROUP4_AMR2_P_SPEC>;
#[doc = "Peli Acceptance Mask register 2"]
pub mod group4_amr2_p;
#[doc = "GROUP4_AMR3_P (rw) register accessor: an alias for `Reg<GROUP4_AMR3_P_SPEC>`"]
pub type GROUP4_AMR3_P = crate::Reg<group4_amr3_p::GROUP4_AMR3_P_SPEC>;
#[doc = "Peli Acceptance Mask register 3"]
pub mod group4_amr3_p;
#[doc = "GROUP5_ACR_B (rw) register accessor: an alias for `Reg<GROUP5_ACR_B_SPEC>`"]
pub type GROUP5_ACR_B = crate::Reg<group5_acr_b::GROUP5_ACR_B_SPEC>;
#[doc = "Basic Acceptance Code register"]
pub mod group5_acr_b;
#[doc = "GROUP5_ACR0_P (rw) register accessor: an alias for `Reg<GROUP5_ACR0_P_SPEC>`"]
pub type GROUP5_ACR0_P = crate::Reg<group5_acr0_p::GROUP5_ACR0_P_SPEC>;
#[doc = "Peli Acceptance Code register0"]
pub mod group5_acr0_p;
#[doc = "GROUP5_ACR1_P (rw) register accessor: an alias for `Reg<GROUP5_ACR1_P_SPEC>`"]
pub type GROUP5_ACR1_P = crate::Reg<group5_acr1_p::GROUP5_ACR1_P_SPEC>;
#[doc = "Peli Acceptance Code register1"]
pub mod group5_acr1_p;
#[doc = "GROUP5_ACR2_P (rw) register accessor: an alias for `Reg<GROUP5_ACR2_P_SPEC>`"]
pub type GROUP5_ACR2_P = crate::Reg<group5_acr2_p::GROUP5_ACR2_P_SPEC>;
#[doc = "Peli Acceptance Code register2"]
pub mod group5_acr2_p;
#[doc = "GROUP5_ACR3_P (rw) register accessor: an alias for `Reg<GROUP5_ACR3_P_SPEC>`"]
pub type GROUP5_ACR3_P = crate::Reg<group5_acr3_p::GROUP5_ACR3_P_SPEC>;
#[doc = "Peli Acceptance Code register3"]
pub mod group5_acr3_p;
#[doc = "GROUP5_AMR_B (rw) register accessor: an alias for `Reg<GROUP5_AMR_B_SPEC>`"]
pub type GROUP5_AMR_B = crate::Reg<group5_amr_b::GROUP5_AMR_B_SPEC>;
#[doc = "Basic Acceptance Mask register"]
pub mod group5_amr_b;
#[doc = "GROUP5_AMR0_P (rw) register accessor: an alias for `Reg<GROUP5_AMR0_P_SPEC>`"]
pub type GROUP5_AMR0_P = crate::Reg<group5_amr0_p::GROUP5_AMR0_P_SPEC>;
#[doc = "Peli Acceptance Mask register 0"]
pub mod group5_amr0_p;
#[doc = "GROUP5_AMR1_P (rw) register accessor: an alias for `Reg<GROUP5_AMR1_P_SPEC>`"]
pub type GROUP5_AMR1_P = crate::Reg<group5_amr1_p::GROUP5_AMR1_P_SPEC>;
#[doc = "Peli Acceptance Mask register 1"]
pub mod group5_amr1_p;
#[doc = "GROUP5_AMR2_P (rw) register accessor: an alias for `Reg<GROUP5_AMR2_P_SPEC>`"]
pub type GROUP5_AMR2_P = crate::Reg<group5_amr2_p::GROUP5_AMR2_P_SPEC>;
#[doc = "Peli Acceptance Mask register 2"]
pub mod group5_amr2_p;
#[doc = "GROUP5_AMR3_P (rw) register accessor: an alias for `Reg<GROUP5_AMR3_P_SPEC>`"]
pub type GROUP5_AMR3_P = crate::Reg<group5_amr3_p::GROUP5_AMR3_P_SPEC>;
#[doc = "Peli Acceptance Mask register 3"]
pub mod group5_amr3_p;
#[doc = "GROUP6_ACR_B (rw) register accessor: an alias for `Reg<GROUP6_ACR_B_SPEC>`"]
pub type GROUP6_ACR_B = crate::Reg<group6_acr_b::GROUP6_ACR_B_SPEC>;
#[doc = "Basic Acceptance Code register"]
pub mod group6_acr_b;
#[doc = "GROUP6_ACR0_P (rw) register accessor: an alias for `Reg<GROUP6_ACR0_P_SPEC>`"]
pub type GROUP6_ACR0_P = crate::Reg<group6_acr0_p::GROUP6_ACR0_P_SPEC>;
#[doc = "Peli Acceptance Code register"]
pub mod group6_acr0_p;
#[doc = "GROUP6_ACR1_P (rw) register accessor: an alias for `Reg<GROUP6_ACR1_P_SPEC>`"]
pub type GROUP6_ACR1_P = crate::Reg<group6_acr1_p::GROUP6_ACR1_P_SPEC>;
#[doc = "Peli Acceptance Code register1"]
pub mod group6_acr1_p;
#[doc = "GROUP6_ACR2_P (rw) register accessor: an alias for `Reg<GROUP6_ACR2_P_SPEC>`"]
pub type GROUP6_ACR2_P = crate::Reg<group6_acr2_p::GROUP6_ACR2_P_SPEC>;
#[doc = "Peli Acceptance Code register2"]
pub mod group6_acr2_p;
#[doc = "GROUP6_ACR3_P (rw) register accessor: an alias for `Reg<GROUP6_ACR3_P_SPEC>`"]
pub type GROUP6_ACR3_P = crate::Reg<group6_acr3_p::GROUP6_ACR3_P_SPEC>;
#[doc = "Peli Acceptance Code register3"]
pub mod group6_acr3_p;
#[doc = "GROUP6_AMR_B (rw) register accessor: an alias for `Reg<GROUP6_AMR_B_SPEC>`"]
pub type GROUP6_AMR_B = crate::Reg<group6_amr_b::GROUP6_AMR_B_SPEC>;
#[doc = "Basic Acceptance Mask register"]
pub mod group6_amr_b;
#[doc = "GROUP6_AMR0_P (rw) register accessor: an alias for `Reg<GROUP6_AMR0_P_SPEC>`"]
pub type GROUP6_AMR0_P = crate::Reg<group6_amr0_p::GROUP6_AMR0_P_SPEC>;
#[doc = "Peli Acceptance Mask register 0"]
pub mod group6_amr0_p;
#[doc = "GROUP6_AMR1_P (rw) register accessor: an alias for `Reg<GROUP6_AMR1_P_SPEC>`"]
pub type GROUP6_AMR1_P = crate::Reg<group6_amr1_p::GROUP6_AMR1_P_SPEC>;
#[doc = "Peli Acceptance Mask register 1"]
pub mod group6_amr1_p;
#[doc = "GROUP6_AMR2_P (rw) register accessor: an alias for `Reg<GROUP6_AMR2_P_SPEC>`"]
pub type GROUP6_AMR2_P = crate::Reg<group6_amr2_p::GROUP6_AMR2_P_SPEC>;
#[doc = "Peli Acceptance Mask register 2"]
pub mod group6_amr2_p;
#[doc = "GROUP6_AMR3_P (rw) register accessor: an alias for `Reg<GROUP6_AMR3_P_SPEC>`"]
pub type GROUP6_AMR3_P = crate::Reg<group6_amr3_p::GROUP6_AMR3_P_SPEC>;
#[doc = "Peli Acceptance Mask register 3"]
pub mod group6_amr3_p;
#[doc = "GROUP7_ACR_B (rw) register accessor: an alias for `Reg<GROUP7_ACR_B_SPEC>`"]
pub type GROUP7_ACR_B = crate::Reg<group7_acr_b::GROUP7_ACR_B_SPEC>;
#[doc = "Basic Acceptance Code register"]
pub mod group7_acr_b;
#[doc = "GROUP7_ACR0_P (rw) register accessor: an alias for `Reg<GROUP7_ACR0_P_SPEC>`"]
pub type GROUP7_ACR0_P = crate::Reg<group7_acr0_p::GROUP7_ACR0_P_SPEC>;
#[doc = "Peli Acceptance Code register0"]
pub mod group7_acr0_p;
#[doc = "GROUP7_ACR1_P (rw) register accessor: an alias for `Reg<GROUP7_ACR1_P_SPEC>`"]
pub type GROUP7_ACR1_P = crate::Reg<group7_acr1_p::GROUP7_ACR1_P_SPEC>;
#[doc = "Peli Acceptance Code register1"]
pub mod group7_acr1_p;
#[doc = "GROUP7_ACR2_P (rw) register accessor: an alias for `Reg<GROUP7_ACR2_P_SPEC>`"]
pub type GROUP7_ACR2_P = crate::Reg<group7_acr2_p::GROUP7_ACR2_P_SPEC>;
#[doc = "Peli Acceptance Code register2"]
pub mod group7_acr2_p;
#[doc = "GROUP7_ACR3_P (rw) register accessor: an alias for `Reg<GROUP7_ACR3_P_SPEC>`"]
pub type GROUP7_ACR3_P = crate::Reg<group7_acr3_p::GROUP7_ACR3_P_SPEC>;
#[doc = "Peli Acceptance Code register3"]
pub mod group7_acr3_p;
#[doc = "GROUP7_AMR_B (rw) register accessor: an alias for `Reg<GROUP7_AMR_B_SPEC>`"]
pub type GROUP7_AMR_B = crate::Reg<group7_amr_b::GROUP7_AMR_B_SPEC>;
#[doc = "Basic Acceptance Mask register"]
pub mod group7_amr_b;
#[doc = "GROUP7_AMR0_P (rw) register accessor: an alias for `Reg<GROUP7_AMR0_P_SPEC>`"]
pub type GROUP7_AMR0_P = crate::Reg<group7_amr0_p::GROUP7_AMR0_P_SPEC>;
#[doc = "Peli Acceptance Mask register 0"]
pub mod group7_amr0_p;
#[doc = "GROUP7_AMR1_P (rw) register accessor: an alias for `Reg<GROUP7_AMR1_P_SPEC>`"]
pub type GROUP7_AMR1_P = crate::Reg<group7_amr1_p::GROUP7_AMR1_P_SPEC>;
#[doc = "Peli Acceptance Mask register 1"]
pub mod group7_amr1_p;
#[doc = "GROUP7_AMR2_P (rw) register accessor: an alias for `Reg<GROUP7_AMR2_P_SPEC>`"]
pub type GROUP7_AMR2_P = crate::Reg<group7_amr2_p::GROUP7_AMR2_P_SPEC>;
#[doc = "Peli Acceptance Mask register 2"]
pub mod group7_amr2_p;
#[doc = "GROUP7_AMR3_P (rw) register accessor: an alias for `Reg<GROUP7_AMR3_P_SPEC>`"]
pub type GROUP7_AMR3_P = crate::Reg<group7_amr3_p::GROUP7_AMR3_P_SPEC>;
#[doc = "Peli Acceptance Mask register 3"]
pub mod group7_amr3_p;
#[doc = "GROUP8_ACR_B (rw) register accessor: an alias for `Reg<GROUP8_ACR_B_SPEC>`"]
pub type GROUP8_ACR_B = crate::Reg<group8_acr_b::GROUP8_ACR_B_SPEC>;
#[doc = "Basic Acceptance Code register"]
pub mod group8_acr_b;
#[doc = "GROUP8_ACR0_P (rw) register accessor: an alias for `Reg<GROUP8_ACR0_P_SPEC>`"]
pub type GROUP8_ACR0_P = crate::Reg<group8_acr0_p::GROUP8_ACR0_P_SPEC>;
#[doc = "Peli Acceptance Code register"]
pub mod group8_acr0_p;
#[doc = "GROUP8_ACR1_P (rw) register accessor: an alias for `Reg<GROUP8_ACR1_P_SPEC>`"]
pub type GROUP8_ACR1_P = crate::Reg<group8_acr1_p::GROUP8_ACR1_P_SPEC>;
#[doc = "Peli Acceptance Code register1"]
pub mod group8_acr1_p;
#[doc = "GROUP8_ACR2_P (rw) register accessor: an alias for `Reg<GROUP8_ACR2_P_SPEC>`"]
pub type GROUP8_ACR2_P = crate::Reg<group8_acr2_p::GROUP8_ACR2_P_SPEC>;
#[doc = "Peli Acceptance Code register2"]
pub mod group8_acr2_p;
#[doc = "GROUP8_ACR3_P (rw) register accessor: an alias for `Reg<GROUP8_ACR3_P_SPEC>`"]
pub type GROUP8_ACR3_P = crate::Reg<group8_acr3_p::GROUP8_ACR3_P_SPEC>;
#[doc = "Peli Acceptance Code register3"]
pub mod group8_acr3_p;
#[doc = "GROUP8_AMR_B (rw) register accessor: an alias for `Reg<GROUP8_AMR_B_SPEC>`"]
pub type GROUP8_AMR_B = crate::Reg<group8_amr_b::GROUP8_AMR_B_SPEC>;
#[doc = "Basic Acceptance Mask register"]
pub mod group8_amr_b;
#[doc = "GROUP8_AMR0_P (rw) register accessor: an alias for `Reg<GROUP8_AMR0_P_SPEC>`"]
pub type GROUP8_AMR0_P = crate::Reg<group8_amr0_p::GROUP8_AMR0_P_SPEC>;
#[doc = "Peli Acceptance Mask register 0"]
pub mod group8_amr0_p;
#[doc = "GROUP8_AMR1_P (rw) register accessor: an alias for `Reg<GROUP8_AMR1_P_SPEC>`"]
pub type GROUP8_AMR1_P = crate::Reg<group8_amr1_p::GROUP8_AMR1_P_SPEC>;
#[doc = "Peli Acceptance Mask register 1"]
pub mod group8_amr1_p;
#[doc = "GROUP8_AMR2_P (rw) register accessor: an alias for `Reg<GROUP8_AMR2_P_SPEC>`"]
pub type GROUP8_AMR2_P = crate::Reg<group8_amr2_p::GROUP8_AMR2_P_SPEC>;
#[doc = "Peli Acceptance Mask register 2"]
pub mod group8_amr2_p;
#[doc = "GROUP8_AMR3_P (rw) register accessor: an alias for `Reg<GROUP8_AMR3_P_SPEC>`"]
pub type GROUP8_AMR3_P = crate::Reg<group8_amr3_p::GROUP8_AMR3_P_SPEC>;
#[doc = "Peli Acceptance Mask register 3"]
pub mod group8_amr3_p;
#[doc = "GROUP9_ACR_B (rw) register accessor: an alias for `Reg<GROUP9_ACR_B_SPEC>`"]
pub type GROUP9_ACR_B = crate::Reg<group9_acr_b::GROUP9_ACR_B_SPEC>;
#[doc = "Basic Acceptance Code register"]
pub mod group9_acr_b;
#[doc = "GROUP9_ACR0_P (rw) register accessor: an alias for `Reg<GROUP9_ACR0_P_SPEC>`"]
pub type GROUP9_ACR0_P = crate::Reg<group9_acr0_p::GROUP9_ACR0_P_SPEC>;
#[doc = "Peli Acceptance Code register0"]
pub mod group9_acr0_p;
#[doc = "GROUP9_ACR1_P (rw) register accessor: an alias for `Reg<GROUP9_ACR1_P_SPEC>`"]
pub type GROUP9_ACR1_P = crate::Reg<group9_acr1_p::GROUP9_ACR1_P_SPEC>;
#[doc = "Peli Acceptance Code register1"]
pub mod group9_acr1_p;
#[doc = "GROUP9_ACR2_P (rw) register accessor: an alias for `Reg<GROUP9_ACR2_P_SPEC>`"]
pub type GROUP9_ACR2_P = crate::Reg<group9_acr2_p::GROUP9_ACR2_P_SPEC>;
#[doc = "Peli Acceptance Code register2"]
pub mod group9_acr2_p;
#[doc = "GROUP9_ACR3_P (rw) register accessor: an alias for `Reg<GROUP9_ACR3_P_SPEC>`"]
pub type GROUP9_ACR3_P = crate::Reg<group9_acr3_p::GROUP9_ACR3_P_SPEC>;
#[doc = "Peli Acceptance Code register3"]
pub mod group9_acr3_p;
#[doc = "GROUP9_AMR_B (rw) register accessor: an alias for `Reg<GROUP9_AMR_B_SPEC>`"]
pub type GROUP9_AMR_B = crate::Reg<group9_amr_b::GROUP9_AMR_B_SPEC>;
#[doc = "Basic Acceptance Mask register"]
pub mod group9_amr_b;
#[doc = "GROUP9_AMR0_P (rw) register accessor: an alias for `Reg<GROUP9_AMR0_P_SPEC>`"]
pub type GROUP9_AMR0_P = crate::Reg<group9_amr0_p::GROUP9_AMR0_P_SPEC>;
#[doc = "Peli Acceptance Mask register 0"]
pub mod group9_amr0_p;
#[doc = "GROUP9_AMR1_P (rw) register accessor: an alias for `Reg<GROUP9_AMR1_P_SPEC>`"]
pub type GROUP9_AMR1_P = crate::Reg<group9_amr1_p::GROUP9_AMR1_P_SPEC>;
#[doc = "Peli Acceptance Mask register 1"]
pub mod group9_amr1_p;
#[doc = "GROUP9_AMR2_P (rw) register accessor: an alias for `Reg<GROUP9_AMR2_P_SPEC>`"]
pub type GROUP9_AMR2_P = crate::Reg<group9_amr2_p::GROUP9_AMR2_P_SPEC>;
#[doc = "Peli Acceptance Mask register 2"]
pub mod group9_amr2_p;
#[doc = "GROUP9_AMR3_P (rw) register accessor: an alias for `Reg<GROUP9_AMR3_P_SPEC>`"]
pub type GROUP9_AMR3_P = crate::Reg<group9_amr3_p::GROUP9_AMR3_P_SPEC>;
#[doc = "Peli Acceptance Mask register 3"]
pub mod group9_amr3_p;
#[doc = "GROUP10_ACR_B (rw) register accessor: an alias for `Reg<GROUP10_ACR_B_SPEC>`"]
pub type GROUP10_ACR_B = crate::Reg<group10_acr_b::GROUP10_ACR_B_SPEC>;
#[doc = "Basic Acceptance Code register"]
pub mod group10_acr_b;
#[doc = "GROUP10_ACR0_P (rw) register accessor: an alias for `Reg<GROUP10_ACR0_P_SPEC>`"]
pub type GROUP10_ACR0_P = crate::Reg<group10_acr0_p::GROUP10_ACR0_P_SPEC>;
#[doc = "Peli Acceptance Code register"]
pub mod group10_acr0_p;
#[doc = "GROUP10_ACR1_P (rw) register accessor: an alias for `Reg<GROUP10_ACR1_P_SPEC>`"]
pub type GROUP10_ACR1_P = crate::Reg<group10_acr1_p::GROUP10_ACR1_P_SPEC>;
#[doc = "Peli Acceptance Code register1"]
pub mod group10_acr1_p;
#[doc = "GROUP10_ACR2_P (rw) register accessor: an alias for `Reg<GROUP10_ACR2_P_SPEC>`"]
pub type GROUP10_ACR2_P = crate::Reg<group10_acr2_p::GROUP10_ACR2_P_SPEC>;
#[doc = "Peli Acceptance Code register2"]
pub mod group10_acr2_p;
#[doc = "GROUP10_ACR3_P (rw) register accessor: an alias for `Reg<GROUP10_ACR3_P_SPEC>`"]
pub type GROUP10_ACR3_P = crate::Reg<group10_acr3_p::GROUP10_ACR3_P_SPEC>;
#[doc = "Peli Acceptance Code register3"]
pub mod group10_acr3_p;
#[doc = "GROUP10_AMR_B (rw) register accessor: an alias for `Reg<GROUP10_AMR_B_SPEC>`"]
pub type GROUP10_AMR_B = crate::Reg<group10_amr_b::GROUP10_AMR_B_SPEC>;
#[doc = "Basic Acceptance Mask register"]
pub mod group10_amr_b;
#[doc = "GROUP10_AMR0_P (rw) register accessor: an alias for `Reg<GROUP10_AMR0_P_SPEC>`"]
pub type GROUP10_AMR0_P = crate::Reg<group10_amr0_p::GROUP10_AMR0_P_SPEC>;
#[doc = "Peli Acceptance Mask register 0"]
pub mod group10_amr0_p;
#[doc = "GROUP10_AMR1_P (rw) register accessor: an alias for `Reg<GROUP10_AMR1_P_SPEC>`"]
pub type GROUP10_AMR1_P = crate::Reg<group10_amr1_p::GROUP10_AMR1_P_SPEC>;
#[doc = "Peli Acceptance Mask register 1"]
pub mod group10_amr1_p;
#[doc = "GROUP10_AMR2_P (rw) register accessor: an alias for `Reg<GROUP10_AMR2_P_SPEC>`"]
pub type GROUP10_AMR2_P = crate::Reg<group10_amr2_p::GROUP10_AMR2_P_SPEC>;
#[doc = "Peli Acceptance Mask register 2"]
pub mod group10_amr2_p;
#[doc = "GROUP10_AMR3_P (rw) register accessor: an alias for `Reg<GROUP10_AMR3_P_SPEC>`"]
pub type GROUP10_AMR3_P = crate::Reg<group10_amr3_p::GROUP10_AMR3_P_SPEC>;
#[doc = "Peli Acceptance Mask register 3"]
pub mod group10_amr3_p;
#[doc = "GROUP11_ACR_B (rw) register accessor: an alias for `Reg<GROUP11_ACR_B_SPEC>`"]
pub type GROUP11_ACR_B = crate::Reg<group11_acr_b::GROUP11_ACR_B_SPEC>;
#[doc = "Basic Acceptance Code register"]
pub mod group11_acr_b;
#[doc = "GROUP11_ACR0_P (rw) register accessor: an alias for `Reg<GROUP11_ACR0_P_SPEC>`"]
pub type GROUP11_ACR0_P = crate::Reg<group11_acr0_p::GROUP11_ACR0_P_SPEC>;
#[doc = "Peli Acceptance Code register0"]
pub mod group11_acr0_p;
#[doc = "GROUP11_ACR1_P (rw) register accessor: an alias for `Reg<GROUP11_ACR1_P_SPEC>`"]
pub type GROUP11_ACR1_P = crate::Reg<group11_acr1_p::GROUP11_ACR1_P_SPEC>;
#[doc = "Peli Acceptance Code register1"]
pub mod group11_acr1_p;
#[doc = "GROUP11_ACR2_P (rw) register accessor: an alias for `Reg<GROUP11_ACR2_P_SPEC>`"]
pub type GROUP11_ACR2_P = crate::Reg<group11_acr2_p::GROUP11_ACR2_P_SPEC>;
#[doc = "Peli Acceptance Code register2"]
pub mod group11_acr2_p;
#[doc = "GROUP11_ACR3_P (rw) register accessor: an alias for `Reg<GROUP11_ACR3_P_SPEC>`"]
pub type GROUP11_ACR3_P = crate::Reg<group11_acr3_p::GROUP11_ACR3_P_SPEC>;
#[doc = "Peli Acceptance Code register3"]
pub mod group11_acr3_p;
#[doc = "GROUP11_AMR_B (rw) register accessor: an alias for `Reg<GROUP11_AMR_B_SPEC>`"]
pub type GROUP11_AMR_B = crate::Reg<group11_amr_b::GROUP11_AMR_B_SPEC>;
#[doc = "Basic Acceptance Mask register"]
pub mod group11_amr_b;
#[doc = "GROUP11_AMR0_P (rw) register accessor: an alias for `Reg<GROUP11_AMR0_P_SPEC>`"]
pub type GROUP11_AMR0_P = crate::Reg<group11_amr0_p::GROUP11_AMR0_P_SPEC>;
#[doc = "Peli Acceptance Mask register 0"]
pub mod group11_amr0_p;
#[doc = "GROUP11_AMR1_P (rw) register accessor: an alias for `Reg<GROUP11_AMR1_P_SPEC>`"]
pub type GROUP11_AMR1_P = crate::Reg<group11_amr1_p::GROUP11_AMR1_P_SPEC>;
#[doc = "Peli Acceptance Mask register 1"]
pub mod group11_amr1_p;
#[doc = "GROUP11_AMR2_P (rw) register accessor: an alias for `Reg<GROUP11_AMR2_P_SPEC>`"]
pub type GROUP11_AMR2_P = crate::Reg<group11_amr2_p::GROUP11_AMR2_P_SPEC>;
#[doc = "Peli Acceptance Mask register 2"]
pub mod group11_amr2_p;
#[doc = "GROUP11_AMR3_P (rw) register accessor: an alias for `Reg<GROUP11_AMR3_P_SPEC>`"]
pub type GROUP11_AMR3_P = crate::Reg<group11_amr3_p::GROUP11_AMR3_P_SPEC>;
#[doc = "Peli Acceptance Mask register 3"]
pub mod group11_amr3_p;
#[doc = "GROUP12_ACR_B (rw) register accessor: an alias for `Reg<GROUP12_ACR_B_SPEC>`"]
pub type GROUP12_ACR_B = crate::Reg<group12_acr_b::GROUP12_ACR_B_SPEC>;
#[doc = "Basic Acceptance Code register"]
pub mod group12_acr_b;
#[doc = "GROUP12_ACR0_P (rw) register accessor: an alias for `Reg<GROUP12_ACR0_P_SPEC>`"]
pub type GROUP12_ACR0_P = crate::Reg<group12_acr0_p::GROUP12_ACR0_P_SPEC>;
#[doc = "Peli Acceptance Code register"]
pub mod group12_acr0_p;
#[doc = "GROUP12_ACR1_P (rw) register accessor: an alias for `Reg<GROUP12_ACR1_P_SPEC>`"]
pub type GROUP12_ACR1_P = crate::Reg<group12_acr1_p::GROUP12_ACR1_P_SPEC>;
#[doc = "Peli Acceptance Code register1"]
pub mod group12_acr1_p;
#[doc = "GROUP12_ACR2_P (rw) register accessor: an alias for `Reg<GROUP12_ACR2_P_SPEC>`"]
pub type GROUP12_ACR2_P = crate::Reg<group12_acr2_p::GROUP12_ACR2_P_SPEC>;
#[doc = "Peli Acceptance Code register2"]
pub mod group12_acr2_p;
#[doc = "GROUP12_ACR3_P (rw) register accessor: an alias for `Reg<GROUP12_ACR3_P_SPEC>`"]
pub type GROUP12_ACR3_P = crate::Reg<group12_acr3_p::GROUP12_ACR3_P_SPEC>;
#[doc = "Peli Acceptance Code register3"]
pub mod group12_acr3_p;
#[doc = "GROUP12_AMR_B (rw) register accessor: an alias for `Reg<GROUP12_AMR_B_SPEC>`"]
pub type GROUP12_AMR_B = crate::Reg<group12_amr_b::GROUP12_AMR_B_SPEC>;
#[doc = "Basic Acceptance Mask register"]
pub mod group12_amr_b;
#[doc = "GROUP12_AMR0_P (rw) register accessor: an alias for `Reg<GROUP12_AMR0_P_SPEC>`"]
pub type GROUP12_AMR0_P = crate::Reg<group12_amr0_p::GROUP12_AMR0_P_SPEC>;
#[doc = "Peli Acceptance Mask register 0"]
pub mod group12_amr0_p;
#[doc = "GROUP12_AMR1_P (rw) register accessor: an alias for `Reg<GROUP12_AMR1_P_SPEC>`"]
pub type GROUP12_AMR1_P = crate::Reg<group12_amr1_p::GROUP12_AMR1_P_SPEC>;
#[doc = "Peli Acceptance Mask register 1"]
pub mod group12_amr1_p;
#[doc = "GROUP12_AMR2_P (rw) register accessor: an alias for `Reg<GROUP12_AMR2_P_SPEC>`"]
pub type GROUP12_AMR2_P = crate::Reg<group12_amr2_p::GROUP12_AMR2_P_SPEC>;
#[doc = "Peli Acceptance Mask register 2"]
pub mod group12_amr2_p;
#[doc = "GROUP12_AMR3_P (rw) register accessor: an alias for `Reg<GROUP12_AMR3_P_SPEC>`"]
pub type GROUP12_AMR3_P = crate::Reg<group12_amr3_p::GROUP12_AMR3_P_SPEC>;
#[doc = "Peli Acceptance Mask register 3"]
pub mod group12_amr3_p;
#[doc = "GROUP13_ACR_B (rw) register accessor: an alias for `Reg<GROUP13_ACR_B_SPEC>`"]
pub type GROUP13_ACR_B = crate::Reg<group13_acr_b::GROUP13_ACR_B_SPEC>;
#[doc = "Basic Acceptance Code register"]
pub mod group13_acr_b;
#[doc = "GROUP13_ACR0_P (rw) register accessor: an alias for `Reg<GROUP13_ACR0_P_SPEC>`"]
pub type GROUP13_ACR0_P = crate::Reg<group13_acr0_p::GROUP13_ACR0_P_SPEC>;
#[doc = "Peli Acceptance Code register0"]
pub mod group13_acr0_p;
#[doc = "GROUP13_ACR1_P (rw) register accessor: an alias for `Reg<GROUP13_ACR1_P_SPEC>`"]
pub type GROUP13_ACR1_P = crate::Reg<group13_acr1_p::GROUP13_ACR1_P_SPEC>;
#[doc = "Peli Acceptance Code register1"]
pub mod group13_acr1_p;
#[doc = "GROUP13_ACR2_P (rw) register accessor: an alias for `Reg<GROUP13_ACR2_P_SPEC>`"]
pub type GROUP13_ACR2_P = crate::Reg<group13_acr2_p::GROUP13_ACR2_P_SPEC>;
#[doc = "Peli Acceptance Code register2"]
pub mod group13_acr2_p;
#[doc = "GROUP13_ACR3_P (rw) register accessor: an alias for `Reg<GROUP13_ACR3_P_SPEC>`"]
pub type GROUP13_ACR3_P = crate::Reg<group13_acr3_p::GROUP13_ACR3_P_SPEC>;
#[doc = "Peli Acceptance Code register3"]
pub mod group13_acr3_p;
#[doc = "GROUP13_AMR_B (rw) register accessor: an alias for `Reg<GROUP13_AMR_B_SPEC>`"]
pub type GROUP13_AMR_B = crate::Reg<group13_amr_b::GROUP13_AMR_B_SPEC>;
#[doc = "Basic Acceptance Mask register"]
pub mod group13_amr_b;
#[doc = "GROUP13_AMR0_P (rw) register accessor: an alias for `Reg<GROUP13_AMR0_P_SPEC>`"]
pub type GROUP13_AMR0_P = crate::Reg<group13_amr0_p::GROUP13_AMR0_P_SPEC>;
#[doc = "Peli Acceptance Mask register 0"]
pub mod group13_amr0_p;
#[doc = "GROUP13_AMR1_P (rw) register accessor: an alias for `Reg<GROUP13_AMR1_P_SPEC>`"]
pub type GROUP13_AMR1_P = crate::Reg<group13_amr1_p::GROUP13_AMR1_P_SPEC>;
#[doc = "Peli Acceptance Mask register 1"]
pub mod group13_amr1_p;
#[doc = "GROUP13_AMR2_P (rw) register accessor: an alias for `Reg<GROUP13_AMR2_P_SPEC>`"]
pub type GROUP13_AMR2_P = crate::Reg<group13_amr2_p::GROUP13_AMR2_P_SPEC>;
#[doc = "Peli Acceptance Mask register 2"]
pub mod group13_amr2_p;
#[doc = "GROUP13_AMR3_P (rw) register accessor: an alias for `Reg<GROUP13_AMR3_P_SPEC>`"]
pub type GROUP13_AMR3_P = crate::Reg<group13_amr3_p::GROUP13_AMR3_P_SPEC>;
#[doc = "Peli Acceptance Mask register 3"]
pub mod group13_amr3_p;
#[doc = "GROUP14_ACR_B (rw) register accessor: an alias for `Reg<GROUP14_ACR_B_SPEC>`"]
pub type GROUP14_ACR_B = crate::Reg<group14_acr_b::GROUP14_ACR_B_SPEC>;
#[doc = "Basic Acceptance Code register"]
pub mod group14_acr_b;
#[doc = "GROUP14_ACR0_P (rw) register accessor: an alias for `Reg<GROUP14_ACR0_P_SPEC>`"]
pub type GROUP14_ACR0_P = crate::Reg<group14_acr0_p::GROUP14_ACR0_P_SPEC>;
#[doc = "Peli Acceptance Code register"]
pub mod group14_acr0_p;
#[doc = "GROUP14_ACR1_P (rw) register accessor: an alias for `Reg<GROUP14_ACR1_P_SPEC>`"]
pub type GROUP14_ACR1_P = crate::Reg<group14_acr1_p::GROUP14_ACR1_P_SPEC>;
#[doc = "Peli Acceptance Code register1"]
pub mod group14_acr1_p;
#[doc = "GROUP14_ACR2_P (rw) register accessor: an alias for `Reg<GROUP14_ACR2_P_SPEC>`"]
pub type GROUP14_ACR2_P = crate::Reg<group14_acr2_p::GROUP14_ACR2_P_SPEC>;
#[doc = "Peli Acceptance Code register2"]
pub mod group14_acr2_p;
#[doc = "GROUP14_ACR3_P (rw) register accessor: an alias for `Reg<GROUP14_ACR3_P_SPEC>`"]
pub type GROUP14_ACR3_P = crate::Reg<group14_acr3_p::GROUP14_ACR3_P_SPEC>;
#[doc = "Peli Acceptance Code register3"]
pub mod group14_acr3_p;
#[doc = "GROUP14_AMR_B (rw) register accessor: an alias for `Reg<GROUP14_AMR_B_SPEC>`"]
pub type GROUP14_AMR_B = crate::Reg<group14_amr_b::GROUP14_AMR_B_SPEC>;
#[doc = "Basic Acceptance Mask register"]
pub mod group14_amr_b;
#[doc = "GROUP14_AMR0_P (rw) register accessor: an alias for `Reg<GROUP14_AMR0_P_SPEC>`"]
pub type GROUP14_AMR0_P = crate::Reg<group14_amr0_p::GROUP14_AMR0_P_SPEC>;
#[doc = "Peli Acceptance Mask register 0"]
pub mod group14_amr0_p;
#[doc = "GROUP14_AMR1_P (rw) register accessor: an alias for `Reg<GROUP14_AMR1_P_SPEC>`"]
pub type GROUP14_AMR1_P = crate::Reg<group14_amr1_p::GROUP14_AMR1_P_SPEC>;
#[doc = "Peli Acceptance Mask register 1"]
pub mod group14_amr1_p;
#[doc = "GROUP14_AMR2_P (rw) register accessor: an alias for `Reg<GROUP14_AMR2_P_SPEC>`"]
pub type GROUP14_AMR2_P = crate::Reg<group14_amr2_p::GROUP14_AMR2_P_SPEC>;
#[doc = "Peli Acceptance Mask register 2"]
pub mod group14_amr2_p;
#[doc = "GROUP14_AMR3_P (rw) register accessor: an alias for `Reg<GROUP14_AMR3_P_SPEC>`"]
pub type GROUP14_AMR3_P = crate::Reg<group14_amr3_p::GROUP14_AMR3_P_SPEC>;
#[doc = "Peli Acceptance Mask register 3"]
pub mod group14_amr3_p;
#[doc = "GROUP15_ACR_B (rw) register accessor: an alias for `Reg<GROUP15_ACR_B_SPEC>`"]
pub type GROUP15_ACR_B = crate::Reg<group15_acr_b::GROUP15_ACR_B_SPEC>;
#[doc = "Basic Acceptance Code register"]
pub mod group15_acr_b;
#[doc = "GROUP15_ACR0_P (rw) register accessor: an alias for `Reg<GROUP15_ACR0_P_SPEC>`"]
pub type GROUP15_ACR0_P = crate::Reg<group15_acr0_p::GROUP15_ACR0_P_SPEC>;
#[doc = "Peli Acceptance Code register0"]
pub mod group15_acr0_p;
#[doc = "GROUP15_ACR1_P (rw) register accessor: an alias for `Reg<GROUP15_ACR1_P_SPEC>`"]
pub type GROUP15_ACR1_P = crate::Reg<group15_acr1_p::GROUP15_ACR1_P_SPEC>;
#[doc = "Peli Acceptance Code register1"]
pub mod group15_acr1_p;
#[doc = "GROUP15_ACR2_P (rw) register accessor: an alias for `Reg<GROUP15_ACR2_P_SPEC>`"]
pub type GROUP15_ACR2_P = crate::Reg<group15_acr2_p::GROUP15_ACR2_P_SPEC>;
#[doc = "Peli Acceptance Code register2"]
pub mod group15_acr2_p;
#[doc = "GROUP15_ACR3_P (rw) register accessor: an alias for `Reg<GROUP15_ACR3_P_SPEC>`"]
pub type GROUP15_ACR3_P = crate::Reg<group15_acr3_p::GROUP15_ACR3_P_SPEC>;
#[doc = "Peli Acceptance Code register3"]
pub mod group15_acr3_p;
#[doc = "GROUP15_AMR_B (rw) register accessor: an alias for `Reg<GROUP15_AMR_B_SPEC>`"]
pub type GROUP15_AMR_B = crate::Reg<group15_amr_b::GROUP15_AMR_B_SPEC>;
#[doc = "Basic Acceptance Mask register"]
pub mod group15_amr_b;
#[doc = "GROUP15_AMR0_P (rw) register accessor: an alias for `Reg<GROUP15_AMR0_P_SPEC>`"]
pub type GROUP15_AMR0_P = crate::Reg<group15_amr0_p::GROUP15_AMR0_P_SPEC>;
#[doc = "Peli Acceptance Mask register 0"]
pub mod group15_amr0_p;
#[doc = "GROUP15_AMR1_P (rw) register accessor: an alias for `Reg<GROUP15_AMR1_P_SPEC>`"]
pub type GROUP15_AMR1_P = crate::Reg<group15_amr1_p::GROUP15_AMR1_P_SPEC>;
#[doc = "Peli Acceptance Mask register 1"]
pub mod group15_amr1_p;
#[doc = "GROUP15_AMR2_P (rw) register accessor: an alias for `Reg<GROUP15_AMR2_P_SPEC>`"]
pub type GROUP15_AMR2_P = crate::Reg<group15_amr2_p::GROUP15_AMR2_P_SPEC>;
#[doc = "Peli Acceptance Mask register 2"]
pub mod group15_amr2_p;
#[doc = "GROUP15_AMR3_P (rw) register accessor: an alias for `Reg<GROUP15_AMR3_P_SPEC>`"]
pub type GROUP15_AMR3_P = crate::Reg<group15_amr3_p::GROUP15_AMR3_P_SPEC>;
#[doc = "Peli Acceptance Mask register 3"]
pub mod group15_amr3_p;
#[doc = "GROUP16_ACR_B (rw) register accessor: an alias for `Reg<GROUP16_ACR_B_SPEC>`"]
pub type GROUP16_ACR_B = crate::Reg<group16_acr_b::GROUP16_ACR_B_SPEC>;
#[doc = "Basic Acceptance Code register"]
pub mod group16_acr_b;
#[doc = "GROUP16_ACR0_P (rw) register accessor: an alias for `Reg<GROUP16_ACR0_P_SPEC>`"]
pub type GROUP16_ACR0_P = crate::Reg<group16_acr0_p::GROUP16_ACR0_P_SPEC>;
#[doc = "Peli Acceptance Code register"]
pub mod group16_acr0_p;
#[doc = "GROUP16_ACR1_P (rw) register accessor: an alias for `Reg<GROUP16_ACR1_P_SPEC>`"]
pub type GROUP16_ACR1_P = crate::Reg<group16_acr1_p::GROUP16_ACR1_P_SPEC>;
#[doc = "Peli Acceptance Code register1"]
pub mod group16_acr1_p;
#[doc = "GROUP16_ACR2_P (rw) register accessor: an alias for `Reg<GROUP16_ACR2_P_SPEC>`"]
pub type GROUP16_ACR2_P = crate::Reg<group16_acr2_p::GROUP16_ACR2_P_SPEC>;
#[doc = "Peli Acceptance Code register2"]
pub mod group16_acr2_p;
#[doc = "GROUP16_ACR3_P (rw) register accessor: an alias for `Reg<GROUP16_ACR3_P_SPEC>`"]
pub type GROUP16_ACR3_P = crate::Reg<group16_acr3_p::GROUP16_ACR3_P_SPEC>;
#[doc = "Peli Acceptance Code register3"]
pub mod group16_acr3_p;
#[doc = "GROUP16_AMR_B (rw) register accessor: an alias for `Reg<GROUP16_AMR_B_SPEC>`"]
pub type GROUP16_AMR_B = crate::Reg<group16_amr_b::GROUP16_AMR_B_SPEC>;
#[doc = "Basic Acceptance Mask register"]
pub mod group16_amr_b;
#[doc = "GROUP16_AMR0_P (rw) register accessor: an alias for `Reg<GROUP16_AMR0_P_SPEC>`"]
pub type GROUP16_AMR0_P = crate::Reg<group16_amr0_p::GROUP16_AMR0_P_SPEC>;
#[doc = "Peli Acceptance Mask register 0"]
pub mod group16_amr0_p;
#[doc = "GROUP16_AMR1_P (rw) register accessor: an alias for `Reg<GROUP16_AMR1_P_SPEC>`"]
pub type GROUP16_AMR1_P = crate::Reg<group16_amr1_p::GROUP16_AMR1_P_SPEC>;
#[doc = "Peli Acceptance Mask register 1"]
pub mod group16_amr1_p;
#[doc = "GROUP16_AMR2_P (rw) register accessor: an alias for `Reg<GROUP16_AMR2_P_SPEC>`"]
pub type GROUP16_AMR2_P = crate::Reg<group16_amr2_p::GROUP16_AMR2_P_SPEC>;
#[doc = "Peli Acceptance Mask register 2"]
pub mod group16_amr2_p;
#[doc = "GROUP16_AMR3_P (rw) register accessor: an alias for `Reg<GROUP16_AMR3_P_SPEC>`"]
pub type GROUP16_AMR3_P = crate::Reg<group16_amr3_p::GROUP16_AMR3_P_SPEC>;
#[doc = "Peli Acceptance Mask register 3"]
pub mod group16_amr3_p;
#[doc = "GROUP17_ACR_B (rw) register accessor: an alias for `Reg<GROUP17_ACR_B_SPEC>`"]
pub type GROUP17_ACR_B = crate::Reg<group17_acr_b::GROUP17_ACR_B_SPEC>;
#[doc = "Basic Acceptance Code register"]
pub mod group17_acr_b;
#[doc = "GROUP17_ACR0_P (rw) register accessor: an alias for `Reg<GROUP17_ACR0_P_SPEC>`"]
pub type GROUP17_ACR0_P = crate::Reg<group17_acr0_p::GROUP17_ACR0_P_SPEC>;
#[doc = "Peli Acceptance Code register0"]
pub mod group17_acr0_p;
#[doc = "GROUP17_ACR1_P (rw) register accessor: an alias for `Reg<GROUP17_ACR1_P_SPEC>`"]
pub type GROUP17_ACR1_P = crate::Reg<group17_acr1_p::GROUP17_ACR1_P_SPEC>;
#[doc = "Peli Acceptance Code register1"]
pub mod group17_acr1_p;
#[doc = "GROUP17_ACR2_P (rw) register accessor: an alias for `Reg<GROUP17_ACR2_P_SPEC>`"]
pub type GROUP17_ACR2_P = crate::Reg<group17_acr2_p::GROUP17_ACR2_P_SPEC>;
#[doc = "Peli Acceptance Code register2"]
pub mod group17_acr2_p;
#[doc = "GROUP17_ACR3_P (rw) register accessor: an alias for `Reg<GROUP17_ACR3_P_SPEC>`"]
pub type GROUP17_ACR3_P = crate::Reg<group17_acr3_p::GROUP17_ACR3_P_SPEC>;
#[doc = "Peli Acceptance Code register3"]
pub mod group17_acr3_p;
#[doc = "GROUP17_AMR_B (rw) register accessor: an alias for `Reg<GROUP17_AMR_B_SPEC>`"]
pub type GROUP17_AMR_B = crate::Reg<group17_amr_b::GROUP17_AMR_B_SPEC>;
#[doc = "Basic Acceptance Mask register"]
pub mod group17_amr_b;
#[doc = "GROUP17_AMR0_P (rw) register accessor: an alias for `Reg<GROUP17_AMR0_P_SPEC>`"]
pub type GROUP17_AMR0_P = crate::Reg<group17_amr0_p::GROUP17_AMR0_P_SPEC>;
#[doc = "Peli Acceptance Mask register 0"]
pub mod group17_amr0_p;
#[doc = "GROUP17_AMR1_P (rw) register accessor: an alias for `Reg<GROUP17_AMR1_P_SPEC>`"]
pub type GROUP17_AMR1_P = crate::Reg<group17_amr1_p::GROUP17_AMR1_P_SPEC>;
#[doc = "Peli Acceptance Mask register 1"]
pub mod group17_amr1_p;
#[doc = "GROUP17_AMR2_P (rw) register accessor: an alias for `Reg<GROUP17_AMR2_P_SPEC>`"]
pub type GROUP17_AMR2_P = crate::Reg<group17_amr2_p::GROUP17_AMR2_P_SPEC>;
#[doc = "Peli Acceptance Mask register 2"]
pub mod group17_amr2_p;
#[doc = "GROUP17_AMR3_P (rw) register accessor: an alias for `Reg<GROUP17_AMR3_P_SPEC>`"]
pub type GROUP17_AMR3_P = crate::Reg<group17_amr3_p::GROUP17_AMR3_P_SPEC>;
#[doc = "Peli Acceptance Mask register 3"]
pub mod group17_amr3_p;
#[doc = "GROUP18_ACR_B (rw) register accessor: an alias for `Reg<GROUP18_ACR_B_SPEC>`"]
pub type GROUP18_ACR_B = crate::Reg<group18_acr_b::GROUP18_ACR_B_SPEC>;
#[doc = "Basic Acceptance Code register"]
pub mod group18_acr_b;
#[doc = "GROUP18_ACR0_P (rw) register accessor: an alias for `Reg<GROUP18_ACR0_P_SPEC>`"]
pub type GROUP18_ACR0_P = crate::Reg<group18_acr0_p::GROUP18_ACR0_P_SPEC>;
#[doc = "Peli Acceptance Code register0"]
pub mod group18_acr0_p;
#[doc = "GROUP18_ACR1_P (rw) register accessor: an alias for `Reg<GROUP18_ACR1_P_SPEC>`"]
pub type GROUP18_ACR1_P = crate::Reg<group18_acr1_p::GROUP18_ACR1_P_SPEC>;
#[doc = "Peli Acceptance Code register1"]
pub mod group18_acr1_p;
#[doc = "GROUP18_ACR2_P (rw) register accessor: an alias for `Reg<GROUP18_ACR2_P_SPEC>`"]
pub type GROUP18_ACR2_P = crate::Reg<group18_acr2_p::GROUP18_ACR2_P_SPEC>;
#[doc = "Peli Acceptance Code register2"]
pub mod group18_acr2_p;
#[doc = "GROUP18_ACR3_P (rw) register accessor: an alias for `Reg<GROUP18_ACR3_P_SPEC>`"]
pub type GROUP18_ACR3_P = crate::Reg<group18_acr3_p::GROUP18_ACR3_P_SPEC>;
#[doc = "Peli Acceptance Code register3"]
pub mod group18_acr3_p;
#[doc = "GROUP18_AMR_B (rw) register accessor: an alias for `Reg<GROUP18_AMR_B_SPEC>`"]
pub type GROUP18_AMR_B = crate::Reg<group18_amr_b::GROUP18_AMR_B_SPEC>;
#[doc = "Basic Acceptance Mask register"]
pub mod group18_amr_b;
#[doc = "GROUP18_AMR0_P (rw) register accessor: an alias for `Reg<GROUP18_AMR0_P_SPEC>`"]
pub type GROUP18_AMR0_P = crate::Reg<group18_amr0_p::GROUP18_AMR0_P_SPEC>;
#[doc = "Peli Acceptance Mask register 0"]
pub mod group18_amr0_p;
#[doc = "GROUP18_AMR1_P (rw) register accessor: an alias for `Reg<GROUP18_AMR1_P_SPEC>`"]
pub type GROUP18_AMR1_P = crate::Reg<group18_amr1_p::GROUP18_AMR1_P_SPEC>;
#[doc = "Peli Acceptance Mask register 1"]
pub mod group18_amr1_p;
#[doc = "GROUP18_AMR2_P (rw) register accessor: an alias for `Reg<GROUP18_AMR2_P_SPEC>`"]
pub type GROUP18_AMR2_P = crate::Reg<group18_amr2_p::GROUP18_AMR2_P_SPEC>;
#[doc = "Peli Acceptance Mask register 2"]
pub mod group18_amr2_p;
#[doc = "GROUP18_AMR3_P (rw) register accessor: an alias for `Reg<GROUP18_AMR3_P_SPEC>`"]
pub type GROUP18_AMR3_P = crate::Reg<group18_amr3_p::GROUP18_AMR3_P_SPEC>;
#[doc = "Peli Acceptance Mask register 3"]
pub mod group18_amr3_p;
#[doc = "GROUP19_ACR_B (rw) register accessor: an alias for `Reg<GROUP19_ACR_B_SPEC>`"]
pub type GROUP19_ACR_B = crate::Reg<group19_acr_b::GROUP19_ACR_B_SPEC>;
#[doc = "Basic Acceptance Code register"]
pub mod group19_acr_b;
#[doc = "GROUP19_ACR0_P (rw) register accessor: an alias for `Reg<GROUP19_ACR0_P_SPEC>`"]
pub type GROUP19_ACR0_P = crate::Reg<group19_acr0_p::GROUP19_ACR0_P_SPEC>;
#[doc = "Peli Acceptance Code register0"]
pub mod group19_acr0_p;
#[doc = "GROUP19_ACR1_P (rw) register accessor: an alias for `Reg<GROUP19_ACR1_P_SPEC>`"]
pub type GROUP19_ACR1_P = crate::Reg<group19_acr1_p::GROUP19_ACR1_P_SPEC>;
#[doc = "Peli Acceptance Code register1"]
pub mod group19_acr1_p;
#[doc = "GROUP19_ACR2_P (rw) register accessor: an alias for `Reg<GROUP19_ACR2_P_SPEC>`"]
pub type GROUP19_ACR2_P = crate::Reg<group19_acr2_p::GROUP19_ACR2_P_SPEC>;
#[doc = "Peli Acceptance Code register2"]
pub mod group19_acr2_p;
#[doc = "GROUP19_ACR3_P (rw) register accessor: an alias for `Reg<GROUP19_ACR3_P_SPEC>`"]
pub type GROUP19_ACR3_P = crate::Reg<group19_acr3_p::GROUP19_ACR3_P_SPEC>;
#[doc = "Peli Acceptance Code register3"]
pub mod group19_acr3_p;
#[doc = "GROUP19_AMR_B (rw) register accessor: an alias for `Reg<GROUP19_AMR_B_SPEC>`"]
pub type GROUP19_AMR_B = crate::Reg<group19_amr_b::GROUP19_AMR_B_SPEC>;
#[doc = "Basic Acceptance Mask register"]
pub mod group19_amr_b;
#[doc = "GROUP19_AMR0_P (rw) register accessor: an alias for `Reg<GROUP19_AMR0_P_SPEC>`"]
pub type GROUP19_AMR0_P = crate::Reg<group19_amr0_p::GROUP19_AMR0_P_SPEC>;
#[doc = "Peli Acceptance Mask register 0"]
pub mod group19_amr0_p;
#[doc = "GROUP19_AMR1_P (rw) register accessor: an alias for `Reg<GROUP19_AMR1_P_SPEC>`"]
pub type GROUP19_AMR1_P = crate::Reg<group19_amr1_p::GROUP19_AMR1_P_SPEC>;
#[doc = "Peli Acceptance Mask register 1"]
pub mod group19_amr1_p;
#[doc = "GROUP19_AMR2_P (rw) register accessor: an alias for `Reg<GROUP19_AMR2_P_SPEC>`"]
pub type GROUP19_AMR2_P = crate::Reg<group19_amr2_p::GROUP19_AMR2_P_SPEC>;
#[doc = "Peli Acceptance Mask register 2"]
pub mod group19_amr2_p;
#[doc = "GROUP19_AMR3_P (rw) register accessor: an alias for `Reg<GROUP19_AMR3_P_SPEC>`"]
pub type GROUP19_AMR3_P = crate::Reg<group19_amr3_p::GROUP19_AMR3_P_SPEC>;
#[doc = "Peli Acceptance Mask register 3"]
pub mod group19_amr3_p;
