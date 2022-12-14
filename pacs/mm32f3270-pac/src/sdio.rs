#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CR1"]
    pub cr1: CR1,
    #[doc = "0x04 - CR2"]
    pub cr2: CR2,
    #[doc = "0x08 - Data transfer byte count register"]
    pub bytecntr: BYTECNTR,
    #[doc = "0x0c - Data block count register"]
    pub tcblkcntr: TCBLKCNTR,
    #[doc = "0x10 - CRC control register"]
    pub crccr: CRCCR,
    #[doc = "0x14 - CMD_CRC register"]
    pub cmd_crc: CMD_CRC,
    #[doc = "0x18 - CRC low data register"]
    pub dat_crcl: DAT_CRCL,
    #[doc = "0x1c - CRC high data register"]
    pub dat_crch: DAT_CRCH,
    #[doc = "0x20 - MMC port register"]
    pub port: PORT,
    #[doc = "0x24 - Interrupt mask register"]
    pub imr: IMR,
    #[doc = "0x28 - Interrupt clear mask register"]
    pub icr: ICR,
    #[doc = "0x2c - card sell"]
    pub cardsel: CARDSEL,
    #[doc = "0x30 - Signal register"]
    pub sigr: SIGR,
    #[doc = "0x34 - Multi-block data transmission register"]
    pub mbcr: MBCR,
    #[doc = "0x38 - Data block count register"]
    pub blkcntr: BLKCNTR,
    #[doc = "0x3c - Data transfer timeout count register"]
    pub tocntr: TOCNTR,
    #[doc = "0x40 - CMD buffer register 0"]
    pub cmd_buf0: CMD_BUF0,
    #[doc = "0x44 - CMD buffer register 1"]
    pub cmd_buf1: CMD_BUF1,
    #[doc = "0x48 - CMD buffer register 2"]
    pub cmd_buf2: CMD_BUF2,
    #[doc = "0x4c - CMD buffer register 3"]
    pub cmd_buf3: CMD_BUF3,
    #[doc = "0x50 - CMD buffer register 4"]
    pub cmd_buf4: CMD_BUF4,
    #[doc = "0x54 - CMD buffer register 5"]
    pub cmd_buf5: CMD_BUF5,
    #[doc = "0x58 - CMD buffer register 6"]
    pub cmd_buf6: CMD_BUF6,
    #[doc = "0x5c - CMD buffer register 7"]
    pub cmd_buf7: CMD_BUF7,
    #[doc = "0x60 - CMD buffer register 8"]
    pub cmd_buf8: CMD_BUF8,
    #[doc = "0x64 - CMD buffer register 9"]
    pub cmd_buf9: CMD_BUF9,
    #[doc = "0x68 - CMD buffer register 10"]
    pub cmd_buf10: CMD_BUF10,
    #[doc = "0x6c - CMD buffer register 11"]
    pub cmd_buf11: CMD_BUF11,
    #[doc = "0x70 - CMD buffer register 12"]
    pub cmd_buf12: CMD_BUF12,
    #[doc = "0x74 - CMD buffer register 13"]
    pub cmd_buf13: CMD_BUF13,
    #[doc = "0x78 - CMD buffer register 14"]
    pub cmd_buf14: CMD_BUF14,
    #[doc = "0x7c - CMD buffer register 15"]
    pub cmd_buf15: CMD_BUF15,
    #[doc = "0x80 - Buffer control register"]
    pub bufcr: BUFCR,
    _reserved33: [u8; 0x7c],
    #[doc = "0x100 - Data buffer register"]
    pub dat_buf: DAT_BUF,
}
#[doc = "CR1 (rw) register accessor: an alias for `Reg<CR1_SPEC>`"]
pub type CR1 = crate::Reg<cr1::CR1_SPEC>;
#[doc = "CR1"]
pub mod cr1;
#[doc = "CR2 (rw) register accessor: an alias for `Reg<CR2_SPEC>`"]
pub type CR2 = crate::Reg<cr2::CR2_SPEC>;
#[doc = "CR2"]
pub mod cr2;
#[doc = "BYTECNTR (rw) register accessor: an alias for `Reg<BYTECNTR_SPEC>`"]
pub type BYTECNTR = crate::Reg<bytecntr::BYTECNTR_SPEC>;
#[doc = "Data transfer byte count register"]
pub mod bytecntr;
#[doc = "TCBLKCNTR (rw) register accessor: an alias for `Reg<TCBLKCNTR_SPEC>`"]
pub type TCBLKCNTR = crate::Reg<tcblkcntr::TCBLKCNTR_SPEC>;
#[doc = "Data block count register"]
pub mod tcblkcntr;
#[doc = "CRCCR (rw) register accessor: an alias for `Reg<CRCCR_SPEC>`"]
pub type CRCCR = crate::Reg<crccr::CRCCR_SPEC>;
#[doc = "CRC control register"]
pub mod crccr;
#[doc = "CMD_CRC (r) register accessor: an alias for `Reg<CMD_CRC_SPEC>`"]
pub type CMD_CRC = crate::Reg<cmd_crc::CMD_CRC_SPEC>;
#[doc = "CMD_CRC register"]
pub mod cmd_crc;
#[doc = "DAT_CRCL (r) register accessor: an alias for `Reg<DAT_CRCL_SPEC>`"]
pub type DAT_CRCL = crate::Reg<dat_crcl::DAT_CRCL_SPEC>;
#[doc = "CRC low data register"]
pub mod dat_crcl;
#[doc = "DAT_CRCH (r) register accessor: an alias for `Reg<DAT_CRCH_SPEC>`"]
pub type DAT_CRCH = crate::Reg<dat_crch::DAT_CRCH_SPEC>;
#[doc = "CRC high data register"]
pub mod dat_crch;
#[doc = "PORT (r) register accessor: an alias for `Reg<PORT_SPEC>`"]
pub type PORT = crate::Reg<port::PORT_SPEC>;
#[doc = "MMC port register"]
pub mod port;
#[doc = "IMR (rw) register accessor: an alias for `Reg<IMR_SPEC>`"]
pub type IMR = crate::Reg<imr::IMR_SPEC>;
#[doc = "Interrupt mask register"]
pub mod imr;
#[doc = "ICR (rw) register accessor: an alias for `Reg<ICR_SPEC>`"]
pub type ICR = crate::Reg<icr::ICR_SPEC>;
#[doc = "Interrupt clear mask register"]
pub mod icr;
#[doc = "CARDSEL (rw) register accessor: an alias for `Reg<CARDSEL_SPEC>`"]
pub type CARDSEL = crate::Reg<cardsel::CARDSEL_SPEC>;
#[doc = "card sell"]
pub mod cardsel;
#[doc = "SIGR (r) register accessor: an alias for `Reg<SIGR_SPEC>`"]
pub type SIGR = crate::Reg<sigr::SIGR_SPEC>;
#[doc = "Signal register"]
pub mod sigr;
#[doc = "MBCR (rw) register accessor: an alias for `Reg<MBCR_SPEC>`"]
pub type MBCR = crate::Reg<mbcr::MBCR_SPEC>;
#[doc = "Multi-block data transmission register"]
pub mod mbcr;
#[doc = "BLKCNTR (rw) register accessor: an alias for `Reg<BLKCNTR_SPEC>`"]
pub type BLKCNTR = crate::Reg<blkcntr::BLKCNTR_SPEC>;
#[doc = "Data block count register"]
pub mod blkcntr;
#[doc = "TOCNTR (rw) register accessor: an alias for `Reg<TOCNTR_SPEC>`"]
pub type TOCNTR = crate::Reg<tocntr::TOCNTR_SPEC>;
#[doc = "Data transfer timeout count register"]
pub mod tocntr;
#[doc = "CMD_BUF0 (rw) register accessor: an alias for `Reg<CMD_BUF0_SPEC>`"]
pub type CMD_BUF0 = crate::Reg<cmd_buf0::CMD_BUF0_SPEC>;
#[doc = "CMD buffer register 0"]
pub mod cmd_buf0;
#[doc = "CMD_BUF1 (rw) register accessor: an alias for `Reg<CMD_BUF1_SPEC>`"]
pub type CMD_BUF1 = crate::Reg<cmd_buf1::CMD_BUF1_SPEC>;
#[doc = "CMD buffer register 1"]
pub mod cmd_buf1;
#[doc = "CMD_BUF2 (rw) register accessor: an alias for `Reg<CMD_BUF2_SPEC>`"]
pub type CMD_BUF2 = crate::Reg<cmd_buf2::CMD_BUF2_SPEC>;
#[doc = "CMD buffer register 2"]
pub mod cmd_buf2;
#[doc = "CMD_BUF3 (rw) register accessor: an alias for `Reg<CMD_BUF3_SPEC>`"]
pub type CMD_BUF3 = crate::Reg<cmd_buf3::CMD_BUF3_SPEC>;
#[doc = "CMD buffer register 3"]
pub mod cmd_buf3;
#[doc = "CMD_BUF4 (rw) register accessor: an alias for `Reg<CMD_BUF4_SPEC>`"]
pub type CMD_BUF4 = crate::Reg<cmd_buf4::CMD_BUF4_SPEC>;
#[doc = "CMD buffer register 4"]
pub mod cmd_buf4;
#[doc = "CMD_BUF5 (rw) register accessor: an alias for `Reg<CMD_BUF5_SPEC>`"]
pub type CMD_BUF5 = crate::Reg<cmd_buf5::CMD_BUF5_SPEC>;
#[doc = "CMD buffer register 5"]
pub mod cmd_buf5;
#[doc = "CMD_BUF6 (rw) register accessor: an alias for `Reg<CMD_BUF6_SPEC>`"]
pub type CMD_BUF6 = crate::Reg<cmd_buf6::CMD_BUF6_SPEC>;
#[doc = "CMD buffer register 6"]
pub mod cmd_buf6;
#[doc = "CMD_BUF7 (rw) register accessor: an alias for `Reg<CMD_BUF7_SPEC>`"]
pub type CMD_BUF7 = crate::Reg<cmd_buf7::CMD_BUF7_SPEC>;
#[doc = "CMD buffer register 7"]
pub mod cmd_buf7;
#[doc = "CMD_BUF8 (rw) register accessor: an alias for `Reg<CMD_BUF8_SPEC>`"]
pub type CMD_BUF8 = crate::Reg<cmd_buf8::CMD_BUF8_SPEC>;
#[doc = "CMD buffer register 8"]
pub mod cmd_buf8;
#[doc = "CMD_BUF9 (rw) register accessor: an alias for `Reg<CMD_BUF9_SPEC>`"]
pub type CMD_BUF9 = crate::Reg<cmd_buf9::CMD_BUF9_SPEC>;
#[doc = "CMD buffer register 9"]
pub mod cmd_buf9;
#[doc = "CMD_BUF10 (rw) register accessor: an alias for `Reg<CMD_BUF10_SPEC>`"]
pub type CMD_BUF10 = crate::Reg<cmd_buf10::CMD_BUF10_SPEC>;
#[doc = "CMD buffer register 10"]
pub mod cmd_buf10;
#[doc = "CMD_BUF11 (rw) register accessor: an alias for `Reg<CMD_BUF11_SPEC>`"]
pub type CMD_BUF11 = crate::Reg<cmd_buf11::CMD_BUF11_SPEC>;
#[doc = "CMD buffer register 11"]
pub mod cmd_buf11;
#[doc = "CMD_BUF12 (rw) register accessor: an alias for `Reg<CMD_BUF12_SPEC>`"]
pub type CMD_BUF12 = crate::Reg<cmd_buf12::CMD_BUF12_SPEC>;
#[doc = "CMD buffer register 12"]
pub mod cmd_buf12;
#[doc = "CMD_BUF13 (rw) register accessor: an alias for `Reg<CMD_BUF13_SPEC>`"]
pub type CMD_BUF13 = crate::Reg<cmd_buf13::CMD_BUF13_SPEC>;
#[doc = "CMD buffer register 13"]
pub mod cmd_buf13;
#[doc = "CMD_BUF14 (rw) register accessor: an alias for `Reg<CMD_BUF14_SPEC>`"]
pub type CMD_BUF14 = crate::Reg<cmd_buf14::CMD_BUF14_SPEC>;
#[doc = "CMD buffer register 14"]
pub mod cmd_buf14;
#[doc = "CMD_BUF15 (rw) register accessor: an alias for `Reg<CMD_BUF15_SPEC>`"]
pub type CMD_BUF15 = crate::Reg<cmd_buf15::CMD_BUF15_SPEC>;
#[doc = "CMD buffer register 15"]
pub mod cmd_buf15;
#[doc = "BUFCR (rw) register accessor: an alias for `Reg<BUFCR_SPEC>`"]
pub type BUFCR = crate::Reg<bufcr::BUFCR_SPEC>;
#[doc = "Buffer control register"]
pub mod bufcr;
#[doc = "DAT_BUF (rw) register accessor: an alias for `Reg<DAT_BUF_SPEC>`"]
pub type DAT_BUF = crate::Reg<dat_buf::DAT_BUF_SPEC>;
#[doc = "Data buffer register"]
pub mod dat_buf;
