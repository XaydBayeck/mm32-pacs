#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Ethernet MAC configuration register"]
    pub cr: CR,
    #[doc = "0x04 - Ethernet MAC frame filter register"]
    pub ffr: FFR,
    #[doc = "0x08 - Ethernet MAC hash table high register"]
    pub hthr: HTHR,
    #[doc = "0x0c - Ethernet MAC hash table low register"]
    pub htlr: HTLR,
    #[doc = "0x10 - Ethernet MAC MII address register"]
    pub miiar: MIIAR,
    #[doc = "0x14 - Ethernet MAC MII data register"]
    pub miidr: MIIDR,
    #[doc = "0x18 - Ethernet MAC flow control register"]
    pub fcr: FCR,
    #[doc = "0x1c - Ethernet MAC VLAN tag register"]
    pub vlantr: VLANTR,
    _reserved8: [u8; 0x04],
    #[doc = "0x24 - Ethernet MAC VLAN tag register"]
    pub sr: SR,
    _reserved9: [u8; 0x18],
    #[doc = "0x40 - Ethernet MAC address 0 high register"]
    pub maca0hr: MACA0HR,
    #[doc = "0x44 - Ethernet MAC address 0 low register"]
    pub maca0lr: MACA0LR,
    #[doc = "0x48 - Ethernet MAC address 1 high register"]
    pub maca1hr: MACA1HR,
    #[doc = "0x4c - Ethernet MAC address1 low register"]
    pub maca1lr: MACA1LR,
    #[doc = "0x50 - Ethernet MAC address 2 high register"]
    pub maca2hr: MACA2HR,
    #[doc = "0x54 - Ethernet MAC address1 low register"]
    pub maca2lr: MACA2LR,
    #[doc = "0x58 - Ethernet MAC address 3 high register"]
    pub maca3hr: MACA3HR,
    #[doc = "0x5c - Ethernet MAC address3 low register"]
    pub maca3lr: MACA3LR,
    #[doc = "0x60 - Ethernet MAC address 4 high register"]
    pub maca4hr: MACA4HR,
    #[doc = "0x64 - Ethernet MAC address4 low register"]
    pub maca4lr: MACA4LR,
    #[doc = "0x68 - Ethernet MAC address 5 high register"]
    pub maca5hr: MACA5HR,
    #[doc = "0x6c - Ethernet MAC address5 low register"]
    pub maca5lr: MACA5LR,
    #[doc = "0x70 - Ethernet MAC address 6 high register"]
    pub maca6hr: MACA6HR,
    #[doc = "0x74 - Ethernet MAC address6 low register"]
    pub maca6lr: MACA6LR,
    #[doc = "0x78 - Ethernet MAC address 7 high register"]
    pub maca7hr: MACA7HR,
    #[doc = "0x7c - Ethernet MAC address7 low register"]
    pub maca7lr: MACA7LR,
    #[doc = "0x80 - Ethernet MAC address 8 high register"]
    pub maca8hr: MACA8HR,
    #[doc = "0x84 - Ethernet MAC address8 low register"]
    pub maca8lr: MACA8LR,
    #[doc = "0x88 - Ethernet MAC address 9 high register"]
    pub maca9hr: MACA9HR,
    #[doc = "0x8c - Ethernet MAC address9 low register"]
    pub maca9lr: MACA9LR,
    #[doc = "0x90 - Ethernet MAC address 10 high register"]
    pub maca10hr: MACA10HR,
    #[doc = "0x94 - Ethernet MAC address10 low register"]
    pub maca10lr: MACA10LR,
    #[doc = "0x98 - Ethernet MAC address 11 high register"]
    pub maca11hr: MACA11HR,
    #[doc = "0x9c - Ethernet MAC address11 low register"]
    pub maca11lr: MACA11LR,
    #[doc = "0xa0 - Ethernet MAC address 12 high register"]
    pub maca12hr: MACA12HR,
    #[doc = "0xa4 - Ethernet MAC address12 low register"]
    pub maca12lr: MACA12LR,
    #[doc = "0xa8 - Ethernet MAC address 13 high register"]
    pub maca13hr: MACA13HR,
    #[doc = "0xac - Ethernet MAC address13 low register"]
    pub maca13lr: MACA13LR,
    #[doc = "0xb0 - Ethernet MAC address 14 high register"]
    pub maca14hr: MACA14HR,
    #[doc = "0xb4 - Ethernet MAC address14 low register"]
    pub maca14lr: MACA14LR,
    #[doc = "0xb8 - Ethernet MAC address 15 high register"]
    pub maca15hr: MACA15HR,
    #[doc = "0xbc - Ethernet MAC address15 low register"]
    pub maca15lr: MACA15LR,
    _reserved41: [u8; 0x0740],
    #[doc = "0x800 - Ethernet MAC address 15 high register"]
    pub maca16hr: MACA16HR,
    #[doc = "0x804 - Ethernet MAC address15 low register"]
    pub maca16lr: MACA16LR,
    #[doc = "0x808 - Ethernet MAC address 15 high register"]
    pub maca17hr: MACA17HR,
    #[doc = "0x80c - Ethernet MAC address15 low register"]
    pub maca17lr: MACA17LR,
    #[doc = "0x810 - Ethernet MAC address 15 high register"]
    pub maca18hr: MACA18HR,
    #[doc = "0x814 - Ethernet MAC address15 low register"]
    pub maca18lr: MACA18LR,
    #[doc = "0x818 - Ethernet MAC address 15 high register"]
    pub maca19hr: MACA19HR,
    #[doc = "0x81c - Ethernet MAC address15 low register"]
    pub maca19lr: MACA19LR,
    #[doc = "0x820 - Ethernet MAC address 15 high register"]
    pub maca20hr: MACA20HR,
    #[doc = "0x824 - Ethernet MAC address15 low register"]
    pub maca20lr: MACA20LR,
    #[doc = "0x828 - Ethernet MAC address 15 high register"]
    pub maca21hr: MACA21HR,
    #[doc = "0x82c - Ethernet MAC address15 low register"]
    pub maca21lr: MACA21LR,
    #[doc = "0x830 - Ethernet MAC address 15 high register"]
    pub maca22hr: MACA22HR,
    #[doc = "0x834 - Ethernet MAC address15 low register"]
    pub maca22lr: MACA22LR,
    #[doc = "0x838 - Ethernet MAC address 15 high register"]
    pub maca23hr: MACA23HR,
    #[doc = "0x83c - Ethernet MAC address15 low register"]
    pub maca23lr: MACA23LR,
    #[doc = "0x840 - Ethernet MAC address 15 high register"]
    pub maca24hr: MACA24HR,
    #[doc = "0x844 - Ethernet MAC address15 low register"]
    pub maca24lr: MACA24LR,
    #[doc = "0x848 - Ethernet MAC address 15 high register"]
    pub maca25hr: MACA25HR,
    #[doc = "0x84c - Ethernet MAC address15 low register"]
    pub maca25lr: MACA25LR,
    #[doc = "0x850 - Ethernet MAC address 15 high register"]
    pub maca26hr: MACA26HR,
    #[doc = "0x854 - Ethernet MAC address15 low register"]
    pub maca26lr: MACA26LR,
    #[doc = "0x858 - Ethernet MAC address 15 high register"]
    pub maca27hr: MACA27HR,
    #[doc = "0x85c - Ethernet MAC address15 low register"]
    pub maca27lr: MACA27LR,
    #[doc = "0x860 - Ethernet MAC address 15 high register"]
    pub maca28hr: MACA28HR,
    #[doc = "0x864 - Ethernet MAC address15 low register"]
    pub maca28lr: MACA28LR,
    #[doc = "0x868 - Ethernet MAC address 15 high register"]
    pub maca29hr: MACA29HR,
    #[doc = "0x86c - Ethernet MAC address15 low register"]
    pub maca29lr: MACA29LR,
    #[doc = "0x870 - Ethernet MAC address 15 high register"]
    pub maca30hr: MACA30HR,
    #[doc = "0x874 - Ethernet MAC address15 low register"]
    pub maca30lr: MACA30LR,
    #[doc = "0x878 - Ethernet MAC address 15 high register"]
    pub maca31hr: MACA31HR,
    #[doc = "0x87c - Ethernet MAC address15 low register"]
    pub maca31lr: MACA31LR,
}
#[doc = "CR (rw) register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "Ethernet MAC configuration register"]
pub mod cr;
#[doc = "FFR (rw) register accessor: an alias for `Reg<FFR_SPEC>`"]
pub type FFR = crate::Reg<ffr::FFR_SPEC>;
#[doc = "Ethernet MAC frame filter register"]
pub mod ffr;
#[doc = "HTHR (rw) register accessor: an alias for `Reg<HTHR_SPEC>`"]
pub type HTHR = crate::Reg<hthr::HTHR_SPEC>;
#[doc = "Ethernet MAC hash table high register"]
pub mod hthr;
#[doc = "HTLR (rw) register accessor: an alias for `Reg<HTLR_SPEC>`"]
pub type HTLR = crate::Reg<htlr::HTLR_SPEC>;
#[doc = "Ethernet MAC hash table low register"]
pub mod htlr;
#[doc = "MIIAR (rw) register accessor: an alias for `Reg<MIIAR_SPEC>`"]
pub type MIIAR = crate::Reg<miiar::MIIAR_SPEC>;
#[doc = "Ethernet MAC MII address register"]
pub mod miiar;
#[doc = "MIIDR (rw) register accessor: an alias for `Reg<MIIDR_SPEC>`"]
pub type MIIDR = crate::Reg<miidr::MIIDR_SPEC>;
#[doc = "Ethernet MAC MII data register"]
pub mod miidr;
#[doc = "FCR (rw) register accessor: an alias for `Reg<FCR_SPEC>`"]
pub type FCR = crate::Reg<fcr::FCR_SPEC>;
#[doc = "Ethernet MAC flow control register"]
pub mod fcr;
#[doc = "VLANTR (rw) register accessor: an alias for `Reg<VLANTR_SPEC>`"]
pub type VLANTR = crate::Reg<vlantr::VLANTR_SPEC>;
#[doc = "Ethernet MAC VLAN tag register"]
pub mod vlantr;
#[doc = "SR (rw) register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "Ethernet MAC VLAN tag register"]
pub mod sr;
#[doc = "MACA0HR (rw) register accessor: an alias for `Reg<MACA0HR_SPEC>`"]
pub type MACA0HR = crate::Reg<maca0hr::MACA0HR_SPEC>;
#[doc = "Ethernet MAC address 0 high register"]
pub mod maca0hr;
#[doc = "MACA0LR (rw) register accessor: an alias for `Reg<MACA0LR_SPEC>`"]
pub type MACA0LR = crate::Reg<maca0lr::MACA0LR_SPEC>;
#[doc = "Ethernet MAC address 0 low register"]
pub mod maca0lr;
#[doc = "MACA1HR (rw) register accessor: an alias for `Reg<MACA1HR_SPEC>`"]
pub type MACA1HR = crate::Reg<maca1hr::MACA1HR_SPEC>;
#[doc = "Ethernet MAC address 1 high register"]
pub mod maca1hr;
#[doc = "MACA1LR (rw) register accessor: an alias for `Reg<MACA1LR_SPEC>`"]
pub type MACA1LR = crate::Reg<maca1lr::MACA1LR_SPEC>;
#[doc = "Ethernet MAC address1 low register"]
pub mod maca1lr;
#[doc = "MACA2HR (rw) register accessor: an alias for `Reg<MACA2HR_SPEC>`"]
pub type MACA2HR = crate::Reg<maca2hr::MACA2HR_SPEC>;
#[doc = "Ethernet MAC address 2 high register"]
pub mod maca2hr;
#[doc = "MACA2LR (rw) register accessor: an alias for `Reg<MACA2LR_SPEC>`"]
pub type MACA2LR = crate::Reg<maca2lr::MACA2LR_SPEC>;
#[doc = "Ethernet MAC address1 low register"]
pub mod maca2lr;
#[doc = "MACA3HR (rw) register accessor: an alias for `Reg<MACA3HR_SPEC>`"]
pub type MACA3HR = crate::Reg<maca3hr::MACA3HR_SPEC>;
#[doc = "Ethernet MAC address 3 high register"]
pub mod maca3hr;
#[doc = "MACA3LR (rw) register accessor: an alias for `Reg<MACA3LR_SPEC>`"]
pub type MACA3LR = crate::Reg<maca3lr::MACA3LR_SPEC>;
#[doc = "Ethernet MAC address3 low register"]
pub mod maca3lr;
#[doc = "MACA4HR (rw) register accessor: an alias for `Reg<MACA4HR_SPEC>`"]
pub type MACA4HR = crate::Reg<maca4hr::MACA4HR_SPEC>;
#[doc = "Ethernet MAC address 4 high register"]
pub mod maca4hr;
#[doc = "MACA4LR (rw) register accessor: an alias for `Reg<MACA4LR_SPEC>`"]
pub type MACA4LR = crate::Reg<maca4lr::MACA4LR_SPEC>;
#[doc = "Ethernet MAC address4 low register"]
pub mod maca4lr;
#[doc = "MACA5HR (rw) register accessor: an alias for `Reg<MACA5HR_SPEC>`"]
pub type MACA5HR = crate::Reg<maca5hr::MACA5HR_SPEC>;
#[doc = "Ethernet MAC address 5 high register"]
pub mod maca5hr;
#[doc = "MACA5LR (rw) register accessor: an alias for `Reg<MACA5LR_SPEC>`"]
pub type MACA5LR = crate::Reg<maca5lr::MACA5LR_SPEC>;
#[doc = "Ethernet MAC address5 low register"]
pub mod maca5lr;
#[doc = "MACA6HR (rw) register accessor: an alias for `Reg<MACA6HR_SPEC>`"]
pub type MACA6HR = crate::Reg<maca6hr::MACA6HR_SPEC>;
#[doc = "Ethernet MAC address 6 high register"]
pub mod maca6hr;
#[doc = "MACA6LR (rw) register accessor: an alias for `Reg<MACA6LR_SPEC>`"]
pub type MACA6LR = crate::Reg<maca6lr::MACA6LR_SPEC>;
#[doc = "Ethernet MAC address6 low register"]
pub mod maca6lr;
#[doc = "MACA7HR (rw) register accessor: an alias for `Reg<MACA7HR_SPEC>`"]
pub type MACA7HR = crate::Reg<maca7hr::MACA7HR_SPEC>;
#[doc = "Ethernet MAC address 7 high register"]
pub mod maca7hr;
#[doc = "MACA7LR (rw) register accessor: an alias for `Reg<MACA7LR_SPEC>`"]
pub type MACA7LR = crate::Reg<maca7lr::MACA7LR_SPEC>;
#[doc = "Ethernet MAC address7 low register"]
pub mod maca7lr;
#[doc = "MACA8HR (rw) register accessor: an alias for `Reg<MACA8HR_SPEC>`"]
pub type MACA8HR = crate::Reg<maca8hr::MACA8HR_SPEC>;
#[doc = "Ethernet MAC address 8 high register"]
pub mod maca8hr;
#[doc = "MACA8LR (rw) register accessor: an alias for `Reg<MACA8LR_SPEC>`"]
pub type MACA8LR = crate::Reg<maca8lr::MACA8LR_SPEC>;
#[doc = "Ethernet MAC address8 low register"]
pub mod maca8lr;
#[doc = "MACA9HR (rw) register accessor: an alias for `Reg<MACA9HR_SPEC>`"]
pub type MACA9HR = crate::Reg<maca9hr::MACA9HR_SPEC>;
#[doc = "Ethernet MAC address 9 high register"]
pub mod maca9hr;
#[doc = "MACA9LR (rw) register accessor: an alias for `Reg<MACA9LR_SPEC>`"]
pub type MACA9LR = crate::Reg<maca9lr::MACA9LR_SPEC>;
#[doc = "Ethernet MAC address9 low register"]
pub mod maca9lr;
#[doc = "MACA10HR (rw) register accessor: an alias for `Reg<MACA10HR_SPEC>`"]
pub type MACA10HR = crate::Reg<maca10hr::MACA10HR_SPEC>;
#[doc = "Ethernet MAC address 10 high register"]
pub mod maca10hr;
#[doc = "MACA10LR (rw) register accessor: an alias for `Reg<MACA10LR_SPEC>`"]
pub type MACA10LR = crate::Reg<maca10lr::MACA10LR_SPEC>;
#[doc = "Ethernet MAC address10 low register"]
pub mod maca10lr;
#[doc = "MACA11HR (rw) register accessor: an alias for `Reg<MACA11HR_SPEC>`"]
pub type MACA11HR = crate::Reg<maca11hr::MACA11HR_SPEC>;
#[doc = "Ethernet MAC address 11 high register"]
pub mod maca11hr;
#[doc = "MACA11LR (rw) register accessor: an alias for `Reg<MACA11LR_SPEC>`"]
pub type MACA11LR = crate::Reg<maca11lr::MACA11LR_SPEC>;
#[doc = "Ethernet MAC address11 low register"]
pub mod maca11lr;
#[doc = "MACA12HR (rw) register accessor: an alias for `Reg<MACA12HR_SPEC>`"]
pub type MACA12HR = crate::Reg<maca12hr::MACA12HR_SPEC>;
#[doc = "Ethernet MAC address 12 high register"]
pub mod maca12hr;
#[doc = "MACA12LR (rw) register accessor: an alias for `Reg<MACA12LR_SPEC>`"]
pub type MACA12LR = crate::Reg<maca12lr::MACA12LR_SPEC>;
#[doc = "Ethernet MAC address12 low register"]
pub mod maca12lr;
#[doc = "MACA13HR (rw) register accessor: an alias for `Reg<MACA13HR_SPEC>`"]
pub type MACA13HR = crate::Reg<maca13hr::MACA13HR_SPEC>;
#[doc = "Ethernet MAC address 13 high register"]
pub mod maca13hr;
#[doc = "MACA13LR (rw) register accessor: an alias for `Reg<MACA13LR_SPEC>`"]
pub type MACA13LR = crate::Reg<maca13lr::MACA13LR_SPEC>;
#[doc = "Ethernet MAC address13 low register"]
pub mod maca13lr;
#[doc = "MACA14HR (rw) register accessor: an alias for `Reg<MACA14HR_SPEC>`"]
pub type MACA14HR = crate::Reg<maca14hr::MACA14HR_SPEC>;
#[doc = "Ethernet MAC address 14 high register"]
pub mod maca14hr;
#[doc = "MACA14LR (rw) register accessor: an alias for `Reg<MACA14LR_SPEC>`"]
pub type MACA14LR = crate::Reg<maca14lr::MACA14LR_SPEC>;
#[doc = "Ethernet MAC address14 low register"]
pub mod maca14lr;
#[doc = "MACA15HR (rw) register accessor: an alias for `Reg<MACA15HR_SPEC>`"]
pub type MACA15HR = crate::Reg<maca15hr::MACA15HR_SPEC>;
#[doc = "Ethernet MAC address 15 high register"]
pub mod maca15hr;
#[doc = "MACA15LR (rw) register accessor: an alias for `Reg<MACA15LR_SPEC>`"]
pub type MACA15LR = crate::Reg<maca15lr::MACA15LR_SPEC>;
#[doc = "Ethernet MAC address15 low register"]
pub mod maca15lr;
#[doc = "MACA16HR (rw) register accessor: an alias for `Reg<MACA16HR_SPEC>`"]
pub type MACA16HR = crate::Reg<maca16hr::MACA16HR_SPEC>;
#[doc = "Ethernet MAC address 15 high register"]
pub mod maca16hr;
#[doc = "MACA16LR (rw) register accessor: an alias for `Reg<MACA16LR_SPEC>`"]
pub type MACA16LR = crate::Reg<maca16lr::MACA16LR_SPEC>;
#[doc = "Ethernet MAC address15 low register"]
pub mod maca16lr;
#[doc = "MACA17HR (rw) register accessor: an alias for `Reg<MACA17HR_SPEC>`"]
pub type MACA17HR = crate::Reg<maca17hr::MACA17HR_SPEC>;
#[doc = "Ethernet MAC address 15 high register"]
pub mod maca17hr;
#[doc = "MACA17LR (rw) register accessor: an alias for `Reg<MACA17LR_SPEC>`"]
pub type MACA17LR = crate::Reg<maca17lr::MACA17LR_SPEC>;
#[doc = "Ethernet MAC address15 low register"]
pub mod maca17lr;
#[doc = "MACA18HR (rw) register accessor: an alias for `Reg<MACA18HR_SPEC>`"]
pub type MACA18HR = crate::Reg<maca18hr::MACA18HR_SPEC>;
#[doc = "Ethernet MAC address 15 high register"]
pub mod maca18hr;
#[doc = "MACA18LR (rw) register accessor: an alias for `Reg<MACA18LR_SPEC>`"]
pub type MACA18LR = crate::Reg<maca18lr::MACA18LR_SPEC>;
#[doc = "Ethernet MAC address15 low register"]
pub mod maca18lr;
#[doc = "MACA19HR (rw) register accessor: an alias for `Reg<MACA19HR_SPEC>`"]
pub type MACA19HR = crate::Reg<maca19hr::MACA19HR_SPEC>;
#[doc = "Ethernet MAC address 15 high register"]
pub mod maca19hr;
#[doc = "MACA19LR (rw) register accessor: an alias for `Reg<MACA19LR_SPEC>`"]
pub type MACA19LR = crate::Reg<maca19lr::MACA19LR_SPEC>;
#[doc = "Ethernet MAC address15 low register"]
pub mod maca19lr;
#[doc = "MACA20HR (rw) register accessor: an alias for `Reg<MACA20HR_SPEC>`"]
pub type MACA20HR = crate::Reg<maca20hr::MACA20HR_SPEC>;
#[doc = "Ethernet MAC address 15 high register"]
pub mod maca20hr;
#[doc = "MACA20LR (rw) register accessor: an alias for `Reg<MACA20LR_SPEC>`"]
pub type MACA20LR = crate::Reg<maca20lr::MACA20LR_SPEC>;
#[doc = "Ethernet MAC address15 low register"]
pub mod maca20lr;
#[doc = "MACA21HR (rw) register accessor: an alias for `Reg<MACA21HR_SPEC>`"]
pub type MACA21HR = crate::Reg<maca21hr::MACA21HR_SPEC>;
#[doc = "Ethernet MAC address 15 high register"]
pub mod maca21hr;
#[doc = "MACA21LR (rw) register accessor: an alias for `Reg<MACA21LR_SPEC>`"]
pub type MACA21LR = crate::Reg<maca21lr::MACA21LR_SPEC>;
#[doc = "Ethernet MAC address15 low register"]
pub mod maca21lr;
#[doc = "MACA22HR (rw) register accessor: an alias for `Reg<MACA22HR_SPEC>`"]
pub type MACA22HR = crate::Reg<maca22hr::MACA22HR_SPEC>;
#[doc = "Ethernet MAC address 15 high register"]
pub mod maca22hr;
#[doc = "MACA22LR (rw) register accessor: an alias for `Reg<MACA22LR_SPEC>`"]
pub type MACA22LR = crate::Reg<maca22lr::MACA22LR_SPEC>;
#[doc = "Ethernet MAC address15 low register"]
pub mod maca22lr;
#[doc = "MACA23HR (rw) register accessor: an alias for `Reg<MACA23HR_SPEC>`"]
pub type MACA23HR = crate::Reg<maca23hr::MACA23HR_SPEC>;
#[doc = "Ethernet MAC address 15 high register"]
pub mod maca23hr;
#[doc = "MACA23LR (rw) register accessor: an alias for `Reg<MACA23LR_SPEC>`"]
pub type MACA23LR = crate::Reg<maca23lr::MACA23LR_SPEC>;
#[doc = "Ethernet MAC address15 low register"]
pub mod maca23lr;
#[doc = "MACA24HR (rw) register accessor: an alias for `Reg<MACA24HR_SPEC>`"]
pub type MACA24HR = crate::Reg<maca24hr::MACA24HR_SPEC>;
#[doc = "Ethernet MAC address 15 high register"]
pub mod maca24hr;
#[doc = "MACA24LR (rw) register accessor: an alias for `Reg<MACA24LR_SPEC>`"]
pub type MACA24LR = crate::Reg<maca24lr::MACA24LR_SPEC>;
#[doc = "Ethernet MAC address15 low register"]
pub mod maca24lr;
#[doc = "MACA25HR (rw) register accessor: an alias for `Reg<MACA25HR_SPEC>`"]
pub type MACA25HR = crate::Reg<maca25hr::MACA25HR_SPEC>;
#[doc = "Ethernet MAC address 15 high register"]
pub mod maca25hr;
#[doc = "MACA25LR (rw) register accessor: an alias for `Reg<MACA25LR_SPEC>`"]
pub type MACA25LR = crate::Reg<maca25lr::MACA25LR_SPEC>;
#[doc = "Ethernet MAC address15 low register"]
pub mod maca25lr;
#[doc = "MACA26HR (rw) register accessor: an alias for `Reg<MACA26HR_SPEC>`"]
pub type MACA26HR = crate::Reg<maca26hr::MACA26HR_SPEC>;
#[doc = "Ethernet MAC address 15 high register"]
pub mod maca26hr;
#[doc = "MACA26LR (rw) register accessor: an alias for `Reg<MACA26LR_SPEC>`"]
pub type MACA26LR = crate::Reg<maca26lr::MACA26LR_SPEC>;
#[doc = "Ethernet MAC address15 low register"]
pub mod maca26lr;
#[doc = "MACA27HR (rw) register accessor: an alias for `Reg<MACA27HR_SPEC>`"]
pub type MACA27HR = crate::Reg<maca27hr::MACA27HR_SPEC>;
#[doc = "Ethernet MAC address 15 high register"]
pub mod maca27hr;
#[doc = "MACA27LR (rw) register accessor: an alias for `Reg<MACA27LR_SPEC>`"]
pub type MACA27LR = crate::Reg<maca27lr::MACA27LR_SPEC>;
#[doc = "Ethernet MAC address15 low register"]
pub mod maca27lr;
#[doc = "MACA28HR (rw) register accessor: an alias for `Reg<MACA28HR_SPEC>`"]
pub type MACA28HR = crate::Reg<maca28hr::MACA28HR_SPEC>;
#[doc = "Ethernet MAC address 15 high register"]
pub mod maca28hr;
#[doc = "MACA28LR (rw) register accessor: an alias for `Reg<MACA28LR_SPEC>`"]
pub type MACA28LR = crate::Reg<maca28lr::MACA28LR_SPEC>;
#[doc = "Ethernet MAC address15 low register"]
pub mod maca28lr;
#[doc = "MACA29HR (rw) register accessor: an alias for `Reg<MACA29HR_SPEC>`"]
pub type MACA29HR = crate::Reg<maca29hr::MACA29HR_SPEC>;
#[doc = "Ethernet MAC address 15 high register"]
pub mod maca29hr;
#[doc = "MACA29LR (rw) register accessor: an alias for `Reg<MACA29LR_SPEC>`"]
pub type MACA29LR = crate::Reg<maca29lr::MACA29LR_SPEC>;
#[doc = "Ethernet MAC address15 low register"]
pub mod maca29lr;
#[doc = "MACA30HR (rw) register accessor: an alias for `Reg<MACA30HR_SPEC>`"]
pub type MACA30HR = crate::Reg<maca30hr::MACA30HR_SPEC>;
#[doc = "Ethernet MAC address 15 high register"]
pub mod maca30hr;
#[doc = "MACA30LR (rw) register accessor: an alias for `Reg<MACA30LR_SPEC>`"]
pub type MACA30LR = crate::Reg<maca30lr::MACA30LR_SPEC>;
#[doc = "Ethernet MAC address15 low register"]
pub mod maca30lr;
#[doc = "MACA31HR (rw) register accessor: an alias for `Reg<MACA31HR_SPEC>`"]
pub type MACA31HR = crate::Reg<maca31hr::MACA31HR_SPEC>;
#[doc = "Ethernet MAC address 15 high register"]
pub mod maca31hr;
#[doc = "MACA31LR (rw) register accessor: an alias for `Reg<MACA31LR_SPEC>`"]
pub type MACA31LR = crate::Reg<maca31lr::MACA31LR_SPEC>;
#[doc = "Ethernet MAC address15 low register"]
pub mod maca31lr;
