#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - configuration low register"]
    pub crl: CRL,
    #[doc = "0x04 - configuration high register"]
    pub crh: CRH,
    #[doc = "0x08 - input data register"]
    pub idr: IDR,
    #[doc = "0x0c - output data register"]
    pub odr: ODR,
    #[doc = "0x10 - bit set/reset register"]
    pub bsrr: BSRR,
    #[doc = "0x14 - bit reset register"]
    pub brr: BRR,
    #[doc = "0x18 - Port configuration lock register"]
    pub lckr: LCKR,
    #[doc = "0x1c - Port output open drain control register"]
    pub dcr: DCR,
    #[doc = "0x20 - Port Multiplexing Function Low Register"]
    pub afrl: AFRL,
    #[doc = "0x24 - Port Multiplexing Function High Register"]
    pub afrh: AFRH,
}
#[doc = "CRL (rw) register accessor: an alias for `Reg<CRL_SPEC>`"]
pub type CRL = crate::Reg<crl::CRL_SPEC>;
#[doc = "configuration low register"]
pub mod crl;
#[doc = "CRH (rw) register accessor: an alias for `Reg<CRH_SPEC>`"]
pub type CRH = crate::Reg<crh::CRH_SPEC>;
#[doc = "configuration high register"]
pub mod crh;
#[doc = "IDR (r) register accessor: an alias for `Reg<IDR_SPEC>`"]
pub type IDR = crate::Reg<idr::IDR_SPEC>;
#[doc = "input data register"]
pub mod idr;
#[doc = "ODR (rw) register accessor: an alias for `Reg<ODR_SPEC>`"]
pub type ODR = crate::Reg<odr::ODR_SPEC>;
#[doc = "output data register"]
pub mod odr;
#[doc = "BSRR (w) register accessor: an alias for `Reg<BSRR_SPEC>`"]
pub type BSRR = crate::Reg<bsrr::BSRR_SPEC>;
#[doc = "bit set/reset register"]
pub mod bsrr;
#[doc = "BRR (w) register accessor: an alias for `Reg<BRR_SPEC>`"]
pub type BRR = crate::Reg<brr::BRR_SPEC>;
#[doc = "bit reset register"]
pub mod brr;
#[doc = "LCKR (rw) register accessor: an alias for `Reg<LCKR_SPEC>`"]
pub type LCKR = crate::Reg<lckr::LCKR_SPEC>;
#[doc = "Port configuration lock register"]
pub mod lckr;
#[doc = "DCR (rw) register accessor: an alias for `Reg<DCR_SPEC>`"]
pub type DCR = crate::Reg<dcr::DCR_SPEC>;
#[doc = "Port output open drain control register"]
pub mod dcr;
#[doc = "AFRL (rw) register accessor: an alias for `Reg<AFRL_SPEC>`"]
pub type AFRL = crate::Reg<afrl::AFRL_SPEC>;
#[doc = "Port Multiplexing Function Low Register"]
pub mod afrl;
#[doc = "AFRH (rw) register accessor: an alias for `Reg<AFRH_SPEC>`"]
pub type AFRH = crate::Reg<afrh::AFRH_SPEC>;
#[doc = "Port Multiplexing Function High Register"]
pub mod afrh;
