#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Flash access control register"]
    pub acr: ACR,
    #[doc = "0x04 - Flash key"]
    pub keyr: KEYR,
    #[doc = "0x08 - Option byte key"]
    pub optkeyr: OPTKEYR,
    #[doc = "0x0c - Flash status register"]
    pub sr: SR,
    #[doc = "0x10 - Flash control register"]
    pub cr: CR,
    #[doc = "0x14 - Flash address register"]
    pub ar: AR,
    _reserved6: [u8; 0x04],
    #[doc = "0x1c - Option byte register"]
    pub obr: OBR,
    #[doc = "0x20 - Write protect register"]
    pub wrpr1: WRPR1,
    #[doc = "0x24 - Write protect register"]
    pub wrpr2: WRPR2,
    #[doc = "0x28 - Write protect register"]
    pub wrpr3: WRPR3,
    #[doc = "0x2c - Write protect register"]
    pub wrpr4: WRPR4,
}
#[doc = "ACR (rw) register accessor: an alias for `Reg<ACR_SPEC>`"]
pub type ACR = crate::Reg<acr::ACR_SPEC>;
#[doc = "Flash access control register"]
pub mod acr;
#[doc = "KEYR (w) register accessor: an alias for `Reg<KEYR_SPEC>`"]
pub type KEYR = crate::Reg<keyr::KEYR_SPEC>;
#[doc = "Flash key"]
pub mod keyr;
#[doc = "OPTKEYR (w) register accessor: an alias for `Reg<OPTKEYR_SPEC>`"]
pub type OPTKEYR = crate::Reg<optkeyr::OPTKEYR_SPEC>;
#[doc = "Option byte key"]
pub mod optkeyr;
#[doc = "SR (rw) register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "Flash status register"]
pub mod sr;
#[doc = "CR (rw) register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "Flash control register"]
pub mod cr;
#[doc = "AR (w) register accessor: an alias for `Reg<AR_SPEC>`"]
pub type AR = crate::Reg<ar::AR_SPEC>;
#[doc = "Flash address register"]
pub mod ar;
#[doc = "OBR (r) register accessor: an alias for `Reg<OBR_SPEC>`"]
pub type OBR = crate::Reg<obr::OBR_SPEC>;
#[doc = "Option byte register"]
pub mod obr;
#[doc = "WRPR1 (r) register accessor: an alias for `Reg<WRPR1_SPEC>`"]
pub type WRPR1 = crate::Reg<wrpr1::WRPR1_SPEC>;
#[doc = "Write protect register"]
pub mod wrpr1;
#[doc = "WRPR2 (r) register accessor: an alias for `Reg<WRPR2_SPEC>`"]
pub type WRPR2 = crate::Reg<wrpr2::WRPR2_SPEC>;
#[doc = "Write protect register"]
pub mod wrpr2;
#[doc = "WRPR3 (r) register accessor: an alias for `Reg<WRPR3_SPEC>`"]
pub type WRPR3 = crate::Reg<wrpr3::WRPR3_SPEC>;
#[doc = "Write protect register"]
pub mod wrpr3;
#[doc = "WRPR4 (r) register accessor: an alias for `Reg<WRPR4_SPEC>`"]
pub type WRPR4 = crate::Reg<wrpr4::WRPR4_SPEC>;
#[doc = "Write protect register"]
pub mod wrpr4;
