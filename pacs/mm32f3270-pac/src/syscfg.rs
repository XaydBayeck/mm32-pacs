#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SYSCFG configuration register 1"]
    pub cfgr1: CFGR1,
    _reserved1: [u8; 0x04],
    #[doc = "0x08 - External interrupt configuration register 1"]
    pub exticr1: EXTICR1,
    _reserved2: [u8; 0x02],
    #[doc = "0x0c - External interrupt configuration register 2"]
    pub exticr2: EXTICR2,
    _reserved3: [u8; 0x02],
    #[doc = "0x10 - External interrupt configuration register 3"]
    pub exticr3: EXTICR3,
    _reserved4: [u8; 0x02],
    #[doc = "0x14 - External interrupt configuration register 4"]
    pub exticr4: EXTICR4,
    _reserved5: [u8; 0x02],
    #[doc = "0x18 - SYSCFG configuration register 1"]
    pub cfgr2: CFGR2,
    #[doc = "0x1c - Power detection configuration status register"]
    pub pdetcsr: PDETCSR,
}
#[doc = "CFGR1 (rw) register accessor: an alias for `Reg<CFGR1_SPEC>`"]
pub type CFGR1 = crate::Reg<cfgr1::CFGR1_SPEC>;
#[doc = "SYSCFG configuration register 1"]
pub mod cfgr1;
#[doc = "EXTICR1 (rw) register accessor: an alias for `Reg<EXTICR1_SPEC>`"]
pub type EXTICR1 = crate::Reg<exticr1::EXTICR1_SPEC>;
#[doc = "External interrupt configuration register 1"]
pub mod exticr1;
#[doc = "EXTICR2 (rw) register accessor: an alias for `Reg<EXTICR2_SPEC>`"]
pub type EXTICR2 = crate::Reg<exticr2::EXTICR2_SPEC>;
#[doc = "External interrupt configuration register 2"]
pub mod exticr2;
#[doc = "EXTICR3 (rw) register accessor: an alias for `Reg<EXTICR3_SPEC>`"]
pub type EXTICR3 = crate::Reg<exticr3::EXTICR3_SPEC>;
#[doc = "External interrupt configuration register 3"]
pub mod exticr3;
#[doc = "EXTICR4 (rw) register accessor: an alias for `Reg<EXTICR4_SPEC>`"]
pub type EXTICR4 = crate::Reg<exticr4::EXTICR4_SPEC>;
#[doc = "External interrupt configuration register 4"]
pub mod exticr4;
#[doc = "CFGR2 (rw) register accessor: an alias for `Reg<CFGR2_SPEC>`"]
pub type CFGR2 = crate::Reg<cfgr2::CFGR2_SPEC>;
#[doc = "SYSCFG configuration register 1"]
pub mod cfgr2;
#[doc = "PDETCSR (rw) register accessor: an alias for `Reg<PDETCSR_SPEC>`"]
pub type PDETCSR = crate::Reg<pdetcsr::PDETCSR_SPEC>;
#[doc = "Power detection configuration status register"]
pub mod pdetcsr;
