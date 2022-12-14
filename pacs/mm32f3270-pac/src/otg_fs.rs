#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x10],
    #[doc = "0x10 - OTG Interrupt Status Register"]
    pub otg_istat: OTG_ISTAT,
    #[doc = "0x14 - OTG Interrupt Control Register"]
    pub otg_ictrl: OTG_ICTRL,
    #[doc = "0x18 - OTG Status Register"]
    pub otg_stat: OTG_STAT,
    #[doc = "0x1c - OTG Control register"]
    pub otg_ctrl: OTG_CTRL,
    _reserved4: [u8; 0x60],
    #[doc = "0x80 - Interrupt status register"]
    pub int_stat: INT_STAT,
    #[doc = "0x84 - Interrupt enable register"]
    pub int_enb: INT_ENB,
    #[doc = "0x88 - Error interrupt status register"]
    pub err_stat: ERR_STAT,
    #[doc = "0x8c - Error interrupt enable register"]
    pub err_enb: ERR_ENB,
    #[doc = "0x90 - Status register"]
    pub stat: STAT,
    #[doc = "0x94 - Control register"]
    pub ctl: CTL,
    #[doc = "0x98 - Address register"]
    pub addr: ADDR,
    #[doc = "0x9c - BDT page register 1"]
    pub bdt_page_01: BDT_PAGE_01,
    #[doc = "0xa0 - Frame number register"]
    pub frm_numl: FRM_NUML,
    #[doc = "0xa4 - Frame number register"]
    pub frm_numh: FRM_NUMH,
    #[doc = "0xa8 - Token register"]
    pub token: TOKEN,
    #[doc = "0xac - SOF threshold register"]
    pub sof_thld: SOF_THLD,
    #[doc = "0xb0 - BDT page register 2"]
    pub bdt_page_02: BDT_PAGE_02,
    #[doc = "0xb4 - BDT page register 3"]
    pub bdt_page_03: BDT_PAGE_03,
    _reserved18: [u8; 0x08],
    #[doc = "0xc0 - Endpoint control register"]
    pub ep_ctl0: EP_CTL0,
    #[doc = "0xc4 - Endpoint control register"]
    pub ep_ctl1: EP_CTL1,
    #[doc = "0xc8 - Endpoint control register"]
    pub ep_ctl2: EP_CTL2,
    #[doc = "0xcc - Endpoint control register"]
    pub ep_ctl3: EP_CTL3,
    #[doc = "0xd0 - Endpoint control register"]
    pub ep_ctl4: EP_CTL4,
    #[doc = "0xd4 - Endpoint control register"]
    pub ep_ctl5: EP_CTL5,
    #[doc = "0xd8 - Endpoint control register"]
    pub ep_ctl6: EP_CTL6,
    #[doc = "0xdc - Endpoint control register"]
    pub ep_ctl7: EP_CTL7,
    #[doc = "0xe0 - Endpoint control register"]
    pub ep_ctl8: EP_CTL8,
    #[doc = "0xe4 - Endpoint control register"]
    pub ep_ctl9: EP_CTL9,
    #[doc = "0xe8 - Endpoint control register"]
    pub ep_ctl10: EP_CTL10,
    #[doc = "0xec - Endpoint control register"]
    pub ep_ctl11: EP_CTL11,
    #[doc = "0xf0 - Endpoint control register"]
    pub ep_ctl12: EP_CTL12,
    #[doc = "0xf4 - Endpoint control register"]
    pub ep_ctl13: EP_CTL13,
    #[doc = "0xf8 - Endpoint control register"]
    pub ep_ctl14: EP_CTL14,
    #[doc = "0xfc - Endpoint control register"]
    pub ep_ctl15: EP_CTL15,
}
#[doc = "OTG_ISTAT (rw) register accessor: an alias for `Reg<OTG_ISTAT_SPEC>`"]
pub type OTG_ISTAT = crate::Reg<otg_istat::OTG_ISTAT_SPEC>;
#[doc = "OTG Interrupt Status Register"]
pub mod otg_istat;
#[doc = "OTG_ICTRL (rw) register accessor: an alias for `Reg<OTG_ICTRL_SPEC>`"]
pub type OTG_ICTRL = crate::Reg<otg_ictrl::OTG_ICTRL_SPEC>;
#[doc = "OTG Interrupt Control Register"]
pub mod otg_ictrl;
#[doc = "OTG_STAT (rw) register accessor: an alias for `Reg<OTG_STAT_SPEC>`"]
pub type OTG_STAT = crate::Reg<otg_stat::OTG_STAT_SPEC>;
#[doc = "OTG Status Register"]
pub mod otg_stat;
#[doc = "OTG_CTRL (rw) register accessor: an alias for `Reg<OTG_CTRL_SPEC>`"]
pub type OTG_CTRL = crate::Reg<otg_ctrl::OTG_CTRL_SPEC>;
#[doc = "OTG Control register"]
pub mod otg_ctrl;
#[doc = "INT_STAT (rw) register accessor: an alias for `Reg<INT_STAT_SPEC>`"]
pub type INT_STAT = crate::Reg<int_stat::INT_STAT_SPEC>;
#[doc = "Interrupt status register"]
pub mod int_stat;
#[doc = "INT_ENB (rw) register accessor: an alias for `Reg<INT_ENB_SPEC>`"]
pub type INT_ENB = crate::Reg<int_enb::INT_ENB_SPEC>;
#[doc = "Interrupt enable register"]
pub mod int_enb;
#[doc = "ERR_STAT (rw) register accessor: an alias for `Reg<ERR_STAT_SPEC>`"]
pub type ERR_STAT = crate::Reg<err_stat::ERR_STAT_SPEC>;
#[doc = "Error interrupt status register"]
pub mod err_stat;
#[doc = "ERR_ENB (rw) register accessor: an alias for `Reg<ERR_ENB_SPEC>`"]
pub type ERR_ENB = crate::Reg<err_enb::ERR_ENB_SPEC>;
#[doc = "Error interrupt enable register"]
pub mod err_enb;
#[doc = "STAT (r) register accessor: an alias for `Reg<STAT_SPEC>`"]
pub type STAT = crate::Reg<stat::STAT_SPEC>;
#[doc = "Status register"]
pub mod stat;
#[doc = "CTL (rw) register accessor: an alias for `Reg<CTL_SPEC>`"]
pub type CTL = crate::Reg<ctl::CTL_SPEC>;
#[doc = "Control register"]
pub mod ctl;
#[doc = "ADDR (rw) register accessor: an alias for `Reg<ADDR_SPEC>`"]
pub type ADDR = crate::Reg<addr::ADDR_SPEC>;
#[doc = "Address register"]
pub mod addr;
#[doc = "BDT_PAGE_01 (rw) register accessor: an alias for `Reg<BDT_PAGE_01_SPEC>`"]
pub type BDT_PAGE_01 = crate::Reg<bdt_page_01::BDT_PAGE_01_SPEC>;
#[doc = "BDT page register 1"]
pub mod bdt_page_01;
#[doc = "FRM_NUML (r) register accessor: an alias for `Reg<FRM_NUML_SPEC>`"]
pub type FRM_NUML = crate::Reg<frm_numl::FRM_NUML_SPEC>;
#[doc = "Frame number register"]
pub mod frm_numl;
#[doc = "FRM_NUMH (r) register accessor: an alias for `Reg<FRM_NUMH_SPEC>`"]
pub type FRM_NUMH = crate::Reg<frm_numh::FRM_NUMH_SPEC>;
#[doc = "Frame number register"]
pub mod frm_numh;
#[doc = "TOKEN (rw) register accessor: an alias for `Reg<TOKEN_SPEC>`"]
pub type TOKEN = crate::Reg<token::TOKEN_SPEC>;
#[doc = "Token register"]
pub mod token;
#[doc = "SOF_THLD (rw) register accessor: an alias for `Reg<SOF_THLD_SPEC>`"]
pub type SOF_THLD = crate::Reg<sof_thld::SOF_THLD_SPEC>;
#[doc = "SOF threshold register"]
pub mod sof_thld;
#[doc = "BDT_PAGE_02 (rw) register accessor: an alias for `Reg<BDT_PAGE_02_SPEC>`"]
pub type BDT_PAGE_02 = crate::Reg<bdt_page_02::BDT_PAGE_02_SPEC>;
#[doc = "BDT page register 2"]
pub mod bdt_page_02;
#[doc = "BDT_PAGE_03 (rw) register accessor: an alias for `Reg<BDT_PAGE_03_SPEC>`"]
pub type BDT_PAGE_03 = crate::Reg<bdt_page_03::BDT_PAGE_03_SPEC>;
#[doc = "BDT page register 3"]
pub mod bdt_page_03;
#[doc = "EP_CTL0 (rw) register accessor: an alias for `Reg<EP_CTL0_SPEC>`"]
pub type EP_CTL0 = crate::Reg<ep_ctl0::EP_CTL0_SPEC>;
#[doc = "Endpoint control register"]
pub mod ep_ctl0;
#[doc = "EP_CTL1 (rw) register accessor: an alias for `Reg<EP_CTL1_SPEC>`"]
pub type EP_CTL1 = crate::Reg<ep_ctl1::EP_CTL1_SPEC>;
#[doc = "Endpoint control register"]
pub mod ep_ctl1;
#[doc = "EP_CTL2 (rw) register accessor: an alias for `Reg<EP_CTL2_SPEC>`"]
pub type EP_CTL2 = crate::Reg<ep_ctl2::EP_CTL2_SPEC>;
#[doc = "Endpoint control register"]
pub mod ep_ctl2;
#[doc = "EP_CTL3 (rw) register accessor: an alias for `Reg<EP_CTL3_SPEC>`"]
pub type EP_CTL3 = crate::Reg<ep_ctl3::EP_CTL3_SPEC>;
#[doc = "Endpoint control register"]
pub mod ep_ctl3;
#[doc = "EP_CTL4 (rw) register accessor: an alias for `Reg<EP_CTL4_SPEC>`"]
pub type EP_CTL4 = crate::Reg<ep_ctl4::EP_CTL4_SPEC>;
#[doc = "Endpoint control register"]
pub mod ep_ctl4;
#[doc = "EP_CTL5 (rw) register accessor: an alias for `Reg<EP_CTL5_SPEC>`"]
pub type EP_CTL5 = crate::Reg<ep_ctl5::EP_CTL5_SPEC>;
#[doc = "Endpoint control register"]
pub mod ep_ctl5;
#[doc = "EP_CTL6 (rw) register accessor: an alias for `Reg<EP_CTL6_SPEC>`"]
pub type EP_CTL6 = crate::Reg<ep_ctl6::EP_CTL6_SPEC>;
#[doc = "Endpoint control register"]
pub mod ep_ctl6;
#[doc = "EP_CTL7 (rw) register accessor: an alias for `Reg<EP_CTL7_SPEC>`"]
pub type EP_CTL7 = crate::Reg<ep_ctl7::EP_CTL7_SPEC>;
#[doc = "Endpoint control register"]
pub mod ep_ctl7;
#[doc = "EP_CTL8 (rw) register accessor: an alias for `Reg<EP_CTL8_SPEC>`"]
pub type EP_CTL8 = crate::Reg<ep_ctl8::EP_CTL8_SPEC>;
#[doc = "Endpoint control register"]
pub mod ep_ctl8;
#[doc = "EP_CTL9 (rw) register accessor: an alias for `Reg<EP_CTL9_SPEC>`"]
pub type EP_CTL9 = crate::Reg<ep_ctl9::EP_CTL9_SPEC>;
#[doc = "Endpoint control register"]
pub mod ep_ctl9;
#[doc = "EP_CTL10 (rw) register accessor: an alias for `Reg<EP_CTL10_SPEC>`"]
pub type EP_CTL10 = crate::Reg<ep_ctl10::EP_CTL10_SPEC>;
#[doc = "Endpoint control register"]
pub mod ep_ctl10;
#[doc = "EP_CTL11 (rw) register accessor: an alias for `Reg<EP_CTL11_SPEC>`"]
pub type EP_CTL11 = crate::Reg<ep_ctl11::EP_CTL11_SPEC>;
#[doc = "Endpoint control register"]
pub mod ep_ctl11;
#[doc = "EP_CTL12 (rw) register accessor: an alias for `Reg<EP_CTL12_SPEC>`"]
pub type EP_CTL12 = crate::Reg<ep_ctl12::EP_CTL12_SPEC>;
#[doc = "Endpoint control register"]
pub mod ep_ctl12;
#[doc = "EP_CTL13 (rw) register accessor: an alias for `Reg<EP_CTL13_SPEC>`"]
pub type EP_CTL13 = crate::Reg<ep_ctl13::EP_CTL13_SPEC>;
#[doc = "Endpoint control register"]
pub mod ep_ctl13;
#[doc = "EP_CTL14 (rw) register accessor: an alias for `Reg<EP_CTL14_SPEC>`"]
pub type EP_CTL14 = crate::Reg<ep_ctl14::EP_CTL14_SPEC>;
#[doc = "Endpoint control register"]
pub mod ep_ctl14;
#[doc = "EP_CTL15 (rw) register accessor: an alias for `Reg<EP_CTL15_SPEC>`"]
pub type EP_CTL15 = crate::Reg<ep_ctl15::EP_CTL15_SPEC>;
#[doc = "Endpoint control register"]
pub mod ep_ctl15;
