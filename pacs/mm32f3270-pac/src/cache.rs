#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Configuration and control register"]
    pub ccr: CCR,
    #[doc = "0x04 - status register"]
    pub sr: SR,
    #[doc = "0x08 - Interrupt mask register"]
    pub imr: IMR,
    #[doc = "0x0c - Interrupt status register"]
    pub isr: ISR,
    _reserved4: [u8; 0x04],
    #[doc = "0x14 - Cache statistics hit register"]
    pub cshr: CSHR,
    #[doc = "0x18 - Cache statistics miss register"]
    pub csmr: CSMR,
}
#[doc = "CCR (rw) register accessor: an alias for `Reg<CCR_SPEC>`"]
pub type CCR = crate::Reg<ccr::CCR_SPEC>;
#[doc = "Configuration and control register"]
pub mod ccr;
#[doc = "SR (r) register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "status register"]
pub mod sr;
#[doc = "IMR (rw) register accessor: an alias for `Reg<IMR_SPEC>`"]
pub type IMR = crate::Reg<imr::IMR_SPEC>;
#[doc = "Interrupt mask register"]
pub mod imr;
#[doc = "ISR (rw) register accessor: an alias for `Reg<ISR_SPEC>`"]
pub type ISR = crate::Reg<isr::ISR_SPEC>;
#[doc = "Interrupt status register"]
pub mod isr;
#[doc = "CSHR (rw) register accessor: an alias for `Reg<CSHR_SPEC>`"]
pub type CSHR = crate::Reg<cshr::CSHR_SPEC>;
#[doc = "Cache statistics hit register"]
pub mod cshr;
#[doc = "CSMR (rw) register accessor: an alias for `Reg<CSMR_SPEC>`"]
pub type CSMR = crate::Reg<csmr::CSMR_SPEC>;
#[doc = "Cache statistics miss register"]
pub mod csmr;
