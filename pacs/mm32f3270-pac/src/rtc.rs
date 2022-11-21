#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - configuration register"]
    pub cr: CR,
    _reserved1: [u8; 0x02],
    #[doc = "0x04 - CSR"]
    pub csr: CSR,
    _reserved2: [u8; 0x02],
    #[doc = "0x08 - Prescaler load high register"]
    pub prlh: PRLH,
    _reserved3: [u8; 0x02],
    #[doc = "0x0c - Prescaler load low register"]
    pub prll: PRLL,
    _reserved4: [u8; 0x02],
    #[doc = "0x10 - Prescaler divider factor high register"]
    pub divh: DIVH,
    _reserved5: [u8; 0x02],
    #[doc = "0x14 - Prescaler divider factor low register"]
    pub divl: DIVL,
    _reserved6: [u8; 0x02],
    #[doc = "0x18 - Counter high register"]
    pub cnth: CNTH,
    _reserved7: [u8; 0x02],
    #[doc = "0x1c - Counter low register"]
    pub cntl: CNTL,
    #[doc = "0x20 - Alarm high register"]
    pub alrh: ALRH,
    _reserved9: [u8; 0x02],
    #[doc = "0x24 - Alarm low register"]
    pub alrl: ALRL,
    #[doc = "0x28 - RTC millisecond alarm high register"]
    pub rtc_msrh: RTC_MSRH,
    _reserved11: [u8; 0x02],
    #[doc = "0x2c - RTC millisecond alarm low register"]
    pub rtc_msrl: RTC_MSRL,
}
#[doc = "CR (rw) register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "configuration register"]
pub mod cr;
#[doc = "CSR (rw) register accessor: an alias for `Reg<CSR_SPEC>`"]
pub type CSR = crate::Reg<csr::CSR_SPEC>;
#[doc = "CSR"]
pub mod csr;
#[doc = "PRLH (w) register accessor: an alias for `Reg<PRLH_SPEC>`"]
pub type PRLH = crate::Reg<prlh::PRLH_SPEC>;
#[doc = "Prescaler load high register"]
pub mod prlh;
#[doc = "PRLL (w) register accessor: an alias for `Reg<PRLL_SPEC>`"]
pub type PRLL = crate::Reg<prll::PRLL_SPEC>;
#[doc = "Prescaler load low register"]
pub mod prll;
#[doc = "DIVH (r) register accessor: an alias for `Reg<DIVH_SPEC>`"]
pub type DIVH = crate::Reg<divh::DIVH_SPEC>;
#[doc = "Prescaler divider factor high register"]
pub mod divh;
#[doc = "DIVL (r) register accessor: an alias for `Reg<DIVL_SPEC>`"]
pub type DIVL = crate::Reg<divl::DIVL_SPEC>;
#[doc = "Prescaler divider factor low register"]
pub mod divl;
#[doc = "CNTH (rw) register accessor: an alias for `Reg<CNTH_SPEC>`"]
pub type CNTH = crate::Reg<cnth::CNTH_SPEC>;
#[doc = "Counter high register"]
pub mod cnth;
#[doc = "CNTL (rw) register accessor: an alias for `Reg<CNTL_SPEC>`"]
pub type CNTL = crate::Reg<cntl::CNTL_SPEC>;
#[doc = "Counter low register"]
pub mod cntl;
#[doc = "ALRH (rw) register accessor: an alias for `Reg<ALRH_SPEC>`"]
pub type ALRH = crate::Reg<alrh::ALRH_SPEC>;
#[doc = "Alarm high register"]
pub mod alrh;
#[doc = "ALRL (rw) register accessor: an alias for `Reg<ALRL_SPEC>`"]
pub type ALRL = crate::Reg<alrl::ALRL_SPEC>;
#[doc = "Alarm low register"]
pub mod alrl;
#[doc = "RTC_MSRH (rw) register accessor: an alias for `Reg<RTC_MSRH_SPEC>`"]
pub type RTC_MSRH = crate::Reg<rtc_msrh::RTC_MSRH_SPEC>;
#[doc = "RTC millisecond alarm high register"]
pub mod rtc_msrh;
#[doc = "RTC_MSRL (rw) register accessor: an alias for `Reg<RTC_MSRL_SPEC>`"]
pub type RTC_MSRL = crate::Reg<rtc_msrl::RTC_MSRL_SPEC>;
#[doc = "RTC millisecond alarm low register"]
pub mod rtc_msrl;
