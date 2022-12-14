#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub cr: CR,
    #[doc = "0x04 - Configuration Register"]
    pub cfgr: CFGR,
    #[doc = "0x08 - Clock Interrupt Register"]
    pub cir: CIR,
    #[doc = "0x0c - Advanced High Performance Bus 3 Reset Register"]
    pub ahb3rstr: AHB3RSTR,
    #[doc = "0x10 - Advanced High Performance Bus 2 Reset Register"]
    pub ahb2rstr: AHB2RSTR,
    #[doc = "0x14 - Advanced High Performance Bus 1 Reset Register"]
    pub ahb1rstr: AHB1RSTR,
    #[doc = "0x18 - Advanced Peripheral Bus 2 Reset Register"]
    pub apb2rstr: APB2RSTR,
    #[doc = "0x1c - Advanced Peripheral Bus 1 Reset Register"]
    pub apb1rstr: APB1RSTR,
    #[doc = "0x20 - Advanced High Performance Bus 3 Enable Register"]
    pub ahb3enr: AHB3ENR,
    #[doc = "0x24 - Advanced High Performance Bus 2 Enable Register"]
    pub ahb2enr: AHB2ENR,
    #[doc = "0x28 - Advanced High Performance Bus 1 Enable Register"]
    pub ahb1enr: AHB1ENR,
    #[doc = "0x2c - Advanced Peripheral Bus 2 Enable Register"]
    pub apb2enr: APB2ENR,
    #[doc = "0x30 - Advanced Peripheral Bus 1 Enable Register"]
    pub apb1enr: APB1ENR,
    #[doc = "0x34 - Backup Domain Control Register"]
    pub bdcr: BDCR,
    #[doc = "0x38 - Control Status Register"]
    pub csr: CSR,
    #[doc = "0x3c - System Configuration Register"]
    pub syscfg: SYSCFG,
    #[doc = "0x40 - Configure Register 2"]
    pub cfgr2: CFGR2,
    #[doc = "0x44 - Internal clock sourcec Configure Register"]
    pub icscr: ICSCR,
    #[doc = "0x48 - PLL Configure Register"]
    pub pllcfgr: PLLCFGR,
    _reserved19: [u8; 0x34],
    #[doc = "0x80 - HSI Delay Register"]
    pub hsidly: HSIDLY,
    #[doc = "0x84 - HSE Delay Register"]
    pub hsedly: HSEDLY,
    #[doc = "0x88 - PLL Delay Register"]
    pub plldly: PLLDLY,
}
#[doc = "CR (rw) register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "Control Register"]
pub mod cr;
#[doc = "CFGR (rw) register accessor: an alias for `Reg<CFGR_SPEC>`"]
pub type CFGR = crate::Reg<cfgr::CFGR_SPEC>;
#[doc = "Configuration Register"]
pub mod cfgr;
#[doc = "CIR (rw) register accessor: an alias for `Reg<CIR_SPEC>`"]
pub type CIR = crate::Reg<cir::CIR_SPEC>;
#[doc = "Clock Interrupt Register"]
pub mod cir;
#[doc = "AHB3RSTR (rw) register accessor: an alias for `Reg<AHB3RSTR_SPEC>`"]
pub type AHB3RSTR = crate::Reg<ahb3rstr::AHB3RSTR_SPEC>;
#[doc = "Advanced High Performance Bus 3 Reset Register"]
pub mod ahb3rstr;
#[doc = "AHB2RSTR (rw) register accessor: an alias for `Reg<AHB2RSTR_SPEC>`"]
pub type AHB2RSTR = crate::Reg<ahb2rstr::AHB2RSTR_SPEC>;
#[doc = "Advanced High Performance Bus 2 Reset Register"]
pub mod ahb2rstr;
#[doc = "AHB1RSTR (rw) register accessor: an alias for `Reg<AHB1RSTR_SPEC>`"]
pub type AHB1RSTR = crate::Reg<ahb1rstr::AHB1RSTR_SPEC>;
#[doc = "Advanced High Performance Bus 1 Reset Register"]
pub mod ahb1rstr;
#[doc = "APB2RSTR (rw) register accessor: an alias for `Reg<APB2RSTR_SPEC>`"]
pub type APB2RSTR = crate::Reg<apb2rstr::APB2RSTR_SPEC>;
#[doc = "Advanced Peripheral Bus 2 Reset Register"]
pub mod apb2rstr;
#[doc = "APB1RSTR (rw) register accessor: an alias for `Reg<APB1RSTR_SPEC>`"]
pub type APB1RSTR = crate::Reg<apb1rstr::APB1RSTR_SPEC>;
#[doc = "Advanced Peripheral Bus 1 Reset Register"]
pub mod apb1rstr;
#[doc = "AHB3ENR (rw) register accessor: an alias for `Reg<AHB3ENR_SPEC>`"]
pub type AHB3ENR = crate::Reg<ahb3enr::AHB3ENR_SPEC>;
#[doc = "Advanced High Performance Bus 3 Enable Register"]
pub mod ahb3enr;
#[doc = "AHB2ENR (rw) register accessor: an alias for `Reg<AHB2ENR_SPEC>`"]
pub type AHB2ENR = crate::Reg<ahb2enr::AHB2ENR_SPEC>;
#[doc = "Advanced High Performance Bus 2 Enable Register"]
pub mod ahb2enr;
#[doc = "AHB1ENR (rw) register accessor: an alias for `Reg<AHB1ENR_SPEC>`"]
pub type AHB1ENR = crate::Reg<ahb1enr::AHB1ENR_SPEC>;
#[doc = "Advanced High Performance Bus 1 Enable Register"]
pub mod ahb1enr;
#[doc = "APB2ENR (rw) register accessor: an alias for `Reg<APB2ENR_SPEC>`"]
pub type APB2ENR = crate::Reg<apb2enr::APB2ENR_SPEC>;
#[doc = "Advanced Peripheral Bus 2 Enable Register"]
pub mod apb2enr;
#[doc = "APB1ENR (rw) register accessor: an alias for `Reg<APB1ENR_SPEC>`"]
pub type APB1ENR = crate::Reg<apb1enr::APB1ENR_SPEC>;
#[doc = "Advanced Peripheral Bus 1 Enable Register"]
pub mod apb1enr;
#[doc = "BDCR (rw) register accessor: an alias for `Reg<BDCR_SPEC>`"]
pub type BDCR = crate::Reg<bdcr::BDCR_SPEC>;
#[doc = "Backup Domain Control Register"]
pub mod bdcr;
#[doc = "CSR (rw) register accessor: an alias for `Reg<CSR_SPEC>`"]
pub type CSR = crate::Reg<csr::CSR_SPEC>;
#[doc = "Control Status Register"]
pub mod csr;
#[doc = "SYSCFG (rw) register accessor: an alias for `Reg<SYSCFG_SPEC>`"]
pub type SYSCFG = crate::Reg<syscfg::SYSCFG_SPEC>;
#[doc = "System Configuration Register"]
pub mod syscfg;
#[doc = "CFGR2 (rw) register accessor: an alias for `Reg<CFGR2_SPEC>`"]
pub type CFGR2 = crate::Reg<cfgr2::CFGR2_SPEC>;
#[doc = "Configure Register 2"]
pub mod cfgr2;
#[doc = "ICSCR (rw) register accessor: an alias for `Reg<ICSCR_SPEC>`"]
pub type ICSCR = crate::Reg<icscr::ICSCR_SPEC>;
#[doc = "Internal clock sourcec Configure Register"]
pub mod icscr;
#[doc = "PLLCFGR (rw) register accessor: an alias for `Reg<PLLCFGR_SPEC>`"]
pub type PLLCFGR = crate::Reg<pllcfgr::PLLCFGR_SPEC>;
#[doc = "PLL Configure Register"]
pub mod pllcfgr;
#[doc = "HSIDLY (rw) register accessor: an alias for `Reg<HSIDLY_SPEC>`"]
pub type HSIDLY = crate::Reg<hsidly::HSIDLY_SPEC>;
#[doc = "HSI Delay Register"]
pub mod hsidly;
#[doc = "HSEDLY (rw) register accessor: an alias for `Reg<HSEDLY_SPEC>`"]
pub type HSEDLY = crate::Reg<hsedly::HSEDLY_SPEC>;
#[doc = "HSE Delay Register"]
pub mod hsedly;
#[doc = "PLLDLY (rw) register accessor: an alias for `Reg<PLLDLY_SPEC>`"]
pub type PLLDLY = crate::Reg<plldly::PLLDLY_SPEC>;
#[doc = "PLL Delay Register"]
pub mod plldly;
