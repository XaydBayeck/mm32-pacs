#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0c],
    #[doc = "0x0c - COMP1 Control State Register"]
    pub comp1_csr: COMP1_CSR,
    #[doc = "0x10 - COMP2 Control State Register"]
    pub comp2_csr: COMP2_CSR,
    _reserved2: [u8; 0x04],
    #[doc = "0x18 - COMP Extern Reference Voltage"]
    pub comp_crv: COMP_CRV,
    #[doc = "0x1c - COMP1 Polling Output Register"]
    pub comp1_poll: COMP1_POLL,
    #[doc = "0x20 - COMP2 Polling Output Register"]
    pub comp2_poll: COMP2_POLL,
}
#[doc = "COMP1_CSR (rw) register accessor: an alias for `Reg<COMP1_CSR_SPEC>`"]
pub type COMP1_CSR = crate::Reg<comp1_csr::COMP1_CSR_SPEC>;
#[doc = "COMP1 Control State Register"]
pub mod comp1_csr;
#[doc = "COMP2_CSR (rw) register accessor: an alias for `Reg<COMP2_CSR_SPEC>`"]
pub type COMP2_CSR = crate::Reg<comp2_csr::COMP2_CSR_SPEC>;
#[doc = "COMP2 Control State Register"]
pub mod comp2_csr;
#[doc = "COMP_CRV (rw) register accessor: an alias for `Reg<COMP_CRV_SPEC>`"]
pub type COMP_CRV = crate::Reg<comp_crv::COMP_CRV_SPEC>;
#[doc = "COMP Extern Reference Voltage"]
pub mod comp_crv;
#[doc = "COMP1_POLL (rw) register accessor: an alias for `Reg<COMP1_POLL_SPEC>`"]
pub type COMP1_POLL = crate::Reg<comp1_poll::COMP1_POLL_SPEC>;
#[doc = "COMP1 Polling Output Register"]
pub mod comp1_poll;
#[doc = "COMP2_POLL (rw) register accessor: an alias for `Reg<COMP2_POLL_SPEC>`"]
pub type COMP2_POLL = crate::Reg<comp2_poll::COMP2_POLL_SPEC>;
#[doc = "COMP2 Polling Output Register"]
pub mod comp2_poll;
