#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - RTC clock calibration register"]
    pub rtccr: RTCCR,
    _reserved1: [u8; 0x02],
    #[doc = "0x04 - Backup control register"]
    pub cr: CR,
    _reserved2: [u8; 0x02],
    #[doc = "0x08 - BKP control/status register"]
    pub csr: CSR,
    _reserved3: [u8; 0x06],
    #[doc = "0x10 - Backup data register(BKP_DR)"]
    pub dr1: DR1,
    #[doc = "0x14 - Backup data register(BKP_DR)"]
    pub dr2: DR2,
    #[doc = "0x18 - Backup data register(BKP_DR)"]
    pub dr3: DR3,
    #[doc = "0x1c - Backup data register(BKP_DR)"]
    pub dr4: DR4,
    #[doc = "0x20 - Backup data register(BKP_DR)"]
    pub dr5: DR5,
    #[doc = "0x24 - Backup data register(BKP_DR)"]
    pub dr6: DR6,
    #[doc = "0x28 - Backup data register(BKP_DR)"]
    pub dr7: DR7,
    #[doc = "0x2c - Backup data register(BKP_DR)"]
    pub dr8: DR8,
    #[doc = "0x30 - Backup data register(BKP_DR)"]
    pub dr9: DR9,
    #[doc = "0x34 - Backup data register(BKP_DR)"]
    pub dr10: DR10,
    #[doc = "0x38 - Backup data register(BKP_DR)"]
    pub dr11: DR11,
    #[doc = "0x3c - Backup data register(BKP_DR)"]
    pub dr12: DR12,
    #[doc = "0x40 - Backup data register(BKP_DR)"]
    pub dr13: DR13,
    #[doc = "0x44 - Backup data register(BKP_DR)"]
    pub dr14: DR14,
    #[doc = "0x48 - Backup data register(BKP_DR)"]
    pub dr15: DR15,
    #[doc = "0x4c - Backup data register(BKP_DR)"]
    pub dr16: DR16,
    #[doc = "0x50 - Backup data register(BKP_DR)"]
    pub dr17: DR17,
    #[doc = "0x54 - Backup data register(BKP_DR)"]
    pub dr18: DR18,
    #[doc = "0x58 - Backup data register(BKP_DR)"]
    pub dr19: DR19,
    #[doc = "0x5c - Backup data register(BKP_DR)"]
    pub dr20: DR20,
}
#[doc = "RTCCR (rw) register accessor: an alias for `Reg<RTCCR_SPEC>`"]
pub type RTCCR = crate::Reg<rtccr::RTCCR_SPEC>;
#[doc = "RTC clock calibration register"]
pub mod rtccr;
#[doc = "CR (rw) register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "Backup control register"]
pub mod cr;
#[doc = "CSR (rw) register accessor: an alias for `Reg<CSR_SPEC>`"]
pub type CSR = crate::Reg<csr::CSR_SPEC>;
#[doc = "BKP control/status register"]
pub mod csr;
#[doc = "DR1 (rw) register accessor: an alias for `Reg<DR1_SPEC>`"]
pub type DR1 = crate::Reg<dr1::DR1_SPEC>;
#[doc = "Backup data register(BKP_DR)"]
pub mod dr1;
#[doc = "DR2 (rw) register accessor: an alias for `Reg<DR2_SPEC>`"]
pub type DR2 = crate::Reg<dr2::DR2_SPEC>;
#[doc = "Backup data register(BKP_DR)"]
pub mod dr2;
#[doc = "DR3 (rw) register accessor: an alias for `Reg<DR3_SPEC>`"]
pub type DR3 = crate::Reg<dr3::DR3_SPEC>;
#[doc = "Backup data register(BKP_DR)"]
pub mod dr3;
#[doc = "DR4 (rw) register accessor: an alias for `Reg<DR4_SPEC>`"]
pub type DR4 = crate::Reg<dr4::DR4_SPEC>;
#[doc = "Backup data register(BKP_DR)"]
pub mod dr4;
#[doc = "DR5 (rw) register accessor: an alias for `Reg<DR5_SPEC>`"]
pub type DR5 = crate::Reg<dr5::DR5_SPEC>;
#[doc = "Backup data register(BKP_DR)"]
pub mod dr5;
#[doc = "DR6 (rw) register accessor: an alias for `Reg<DR6_SPEC>`"]
pub type DR6 = crate::Reg<dr6::DR6_SPEC>;
#[doc = "Backup data register(BKP_DR)"]
pub mod dr6;
#[doc = "DR7 (rw) register accessor: an alias for `Reg<DR7_SPEC>`"]
pub type DR7 = crate::Reg<dr7::DR7_SPEC>;
#[doc = "Backup data register(BKP_DR)"]
pub mod dr7;
#[doc = "DR8 (rw) register accessor: an alias for `Reg<DR8_SPEC>`"]
pub type DR8 = crate::Reg<dr8::DR8_SPEC>;
#[doc = "Backup data register(BKP_DR)"]
pub mod dr8;
#[doc = "DR9 (rw) register accessor: an alias for `Reg<DR9_SPEC>`"]
pub type DR9 = crate::Reg<dr9::DR9_SPEC>;
#[doc = "Backup data register(BKP_DR)"]
pub mod dr9;
#[doc = "DR10 (rw) register accessor: an alias for `Reg<DR10_SPEC>`"]
pub type DR10 = crate::Reg<dr10::DR10_SPEC>;
#[doc = "Backup data register(BKP_DR)"]
pub mod dr10;
#[doc = "DR11 (rw) register accessor: an alias for `Reg<DR11_SPEC>`"]
pub type DR11 = crate::Reg<dr11::DR11_SPEC>;
#[doc = "Backup data register(BKP_DR)"]
pub mod dr11;
#[doc = "DR12 (rw) register accessor: an alias for `Reg<DR12_SPEC>`"]
pub type DR12 = crate::Reg<dr12::DR12_SPEC>;
#[doc = "Backup data register(BKP_DR)"]
pub mod dr12;
#[doc = "DR13 (rw) register accessor: an alias for `Reg<DR13_SPEC>`"]
pub type DR13 = crate::Reg<dr13::DR13_SPEC>;
#[doc = "Backup data register(BKP_DR)"]
pub mod dr13;
#[doc = "DR14 (rw) register accessor: an alias for `Reg<DR14_SPEC>`"]
pub type DR14 = crate::Reg<dr14::DR14_SPEC>;
#[doc = "Backup data register(BKP_DR)"]
pub mod dr14;
#[doc = "DR15 (rw) register accessor: an alias for `Reg<DR15_SPEC>`"]
pub type DR15 = crate::Reg<dr15::DR15_SPEC>;
#[doc = "Backup data register(BKP_DR)"]
pub mod dr15;
#[doc = "DR16 (rw) register accessor: an alias for `Reg<DR16_SPEC>`"]
pub type DR16 = crate::Reg<dr16::DR16_SPEC>;
#[doc = "Backup data register(BKP_DR)"]
pub mod dr16;
#[doc = "DR17 (rw) register accessor: an alias for `Reg<DR17_SPEC>`"]
pub type DR17 = crate::Reg<dr17::DR17_SPEC>;
#[doc = "Backup data register(BKP_DR)"]
pub mod dr17;
#[doc = "DR18 (rw) register accessor: an alias for `Reg<DR18_SPEC>`"]
pub type DR18 = crate::Reg<dr18::DR18_SPEC>;
#[doc = "Backup data register(BKP_DR)"]
pub mod dr18;
#[doc = "DR19 (rw) register accessor: an alias for `Reg<DR19_SPEC>`"]
pub type DR19 = crate::Reg<dr19::DR19_SPEC>;
#[doc = "Backup data register(BKP_DR)"]
pub mod dr19;
#[doc = "DR20 (rw) register accessor: an alias for `Reg<DR20_SPEC>`"]
pub type DR20 = crate::Reg<dr20::DR20_SPEC>;
#[doc = "Backup data register(BKP_DR)"]
pub mod dr20;
