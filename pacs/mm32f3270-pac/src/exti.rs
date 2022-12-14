#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Interrupt mask register"]
    pub imr: IMR,
    #[doc = "0x04 - Event mask register"]
    pub emr: EMR,
    #[doc = "0x08 - Rising trigger selection register"]
    pub rtsr: RTSR,
    #[doc = "0x0c - Falling trigger selection register"]
    pub ftsr: FTSR,
    #[doc = "0x10 - Software interrupt event register"]
    pub swier: SWIER,
    #[doc = "0x14 - Pending register"]
    pub pr: PR,
}
#[doc = "IMR (rw) register accessor: an alias for `Reg<IMR_SPEC>`"]
pub type IMR = crate::Reg<imr::IMR_SPEC>;
#[doc = "Interrupt mask register"]
pub mod imr;
#[doc = "EMR (rw) register accessor: an alias for `Reg<EMR_SPEC>`"]
pub type EMR = crate::Reg<emr::EMR_SPEC>;
#[doc = "Event mask register"]
pub mod emr;
#[doc = "RTSR (rw) register accessor: an alias for `Reg<RTSR_SPEC>`"]
pub type RTSR = crate::Reg<rtsr::RTSR_SPEC>;
#[doc = "Rising trigger selection register"]
pub mod rtsr;
#[doc = "FTSR (rw) register accessor: an alias for `Reg<FTSR_SPEC>`"]
pub type FTSR = crate::Reg<ftsr::FTSR_SPEC>;
#[doc = "Falling trigger selection register"]
pub mod ftsr;
#[doc = "SWIER (rw) register accessor: an alias for `Reg<SWIER_SPEC>`"]
pub type SWIER = crate::Reg<swier::SWIER_SPEC>;
#[doc = "Software interrupt event register"]
pub mod swier;
#[doc = "PR (rw) register accessor: an alias for `Reg<PR_SPEC>`"]
pub type PR = crate::Reg<pr::PR_SPEC>;
#[doc = "Pending register"]
pub mod pr;
