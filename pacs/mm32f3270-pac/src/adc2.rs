#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Data register"]
    pub dr: DR,
    #[doc = "0x04 - Configure register"]
    pub cfgr: CFGR,
    #[doc = "0x08 - Control register"]
    pub cr: CR,
    #[doc = "0x0c - Channel select register"]
    pub chsr: CHSR,
    #[doc = "0x10 - Compare register"]
    pub cmpr: CMPR,
    #[doc = "0x14 - Status register"]
    pub sr: SR,
    #[doc = "0x18 - Channel 0 data register"]
    pub ch0dr: CH0DR,
    #[doc = "0x1c - Channel 1 data register"]
    pub ch1dr: CH1DR,
    #[doc = "0x20 - Channel 2 data register"]
    pub ch2dr: CH2DR,
    #[doc = "0x24 - Channel 3 data register"]
    pub ch3dr: CH3DR,
    #[doc = "0x28 - Channel 4 data register"]
    pub ch4dr: CH4DR,
    #[doc = "0x2c - Channel 5 data register"]
    pub ch5dr: CH5DR,
    #[doc = "0x30 - Channel 6 data register"]
    pub ch6dr: CH6DR,
    #[doc = "0x34 - Channel 7 data register"]
    pub ch7dr: CH7DR,
    #[doc = "0x38 - Channel 8 data register"]
    pub ch8dr: CH8DR,
    #[doc = "0x3c - Channel 9 data register"]
    pub ch9dr: CH9DR,
    #[doc = "0x40 - Channel 10 data register"]
    pub ch10dr: CH10DR,
    #[doc = "0x44 - Channel 11 data register"]
    pub ch11dr: CH11DR,
    #[doc = "0x48 - Channel 12 data register"]
    pub ch12dr: CH12DR,
    #[doc = "0x4c - Channel 13 data register"]
    pub ch13dr: CH13DR,
    #[doc = "0x50 - Channel 14 data register"]
    pub ch14dr: CH14DR,
    #[doc = "0x54 - Channel 15 data register"]
    pub ch15dr: CH15DR,
    #[doc = "0x58 - Extend state register"]
    pub sta_ext: STA_EXT,
    #[doc = "0x5c - Arbitrary channel channel selection register 0"]
    pub chany0: CHANY0,
    #[doc = "0x60 - Arbitrary channel channel selection register 1"]
    pub chany1: CHANY1,
    #[doc = "0x64 - Arbitrary channel configuration register"]
    pub any_cfg: ANY_CFG,
    #[doc = "0x68 - Arbitrary channel control register"]
    pub any_cr: ANY_CR,
    _reserved27: [u8; 0x04],
    #[doc = "0x70 - Any channel configuration register 1"]
    pub smpr1: SMPR1,
    #[doc = "0x74 - Any channel configuration register 2"]
    pub smpr2: SMPR2,
    _reserved29: [u8; 0x04],
    #[doc = "0x7c - Injection channe 0 data compensation register"]
    pub jofr0: JOFR0,
    #[doc = "0x80 - Injection channe 1 data compensation register"]
    pub jofr1: JOFR1,
    #[doc = "0x84 - Injection channe 2 data compensation register"]
    pub jofr2: JOFR2,
    #[doc = "0x88 - Injection channe 3 data compensation register"]
    pub jofr3: JOFR3,
    #[doc = "0x8c - Injection sequence register"]
    pub jsqr: JSQR,
    #[doc = "0x90 - Injection data register"]
    pub jdata: JDATA,
    _reserved35: [u8; 0x1c],
    #[doc = "0xb0 - Injection data register"]
    pub jdr0: JDR0,
    #[doc = "0xb4 - Injection data register"]
    pub jdr1: JDR1,
    #[doc = "0xb8 - Injection data register"]
    pub jdr2: JDR2,
    #[doc = "0xbc - Injection data register"]
    pub jdr3: JDR3,
}
#[doc = "DR (r) register accessor: an alias for `Reg<DR_SPEC>`"]
pub type DR = crate::Reg<dr::DR_SPEC>;
#[doc = "Data register"]
pub mod dr;
#[doc = "CFGR (rw) register accessor: an alias for `Reg<CFGR_SPEC>`"]
pub type CFGR = crate::Reg<cfgr::CFGR_SPEC>;
#[doc = "Configure register"]
pub mod cfgr;
#[doc = "CR (rw) register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "Control register"]
pub mod cr;
#[doc = "CHSR (rw) register accessor: an alias for `Reg<CHSR_SPEC>`"]
pub type CHSR = crate::Reg<chsr::CHSR_SPEC>;
#[doc = "Channel select register"]
pub mod chsr;
#[doc = "CMPR (rw) register accessor: an alias for `Reg<CMPR_SPEC>`"]
pub type CMPR = crate::Reg<cmpr::CMPR_SPEC>;
#[doc = "Compare register"]
pub mod cmpr;
#[doc = "SR (rw) register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "Status register"]
pub mod sr;
#[doc = "CH0DR (r) register accessor: an alias for `Reg<CH0DR_SPEC>`"]
pub type CH0DR = crate::Reg<ch0dr::CH0DR_SPEC>;
#[doc = "Channel 0 data register"]
pub mod ch0dr;
#[doc = "CH1DR (r) register accessor: an alias for `Reg<CH1DR_SPEC>`"]
pub type CH1DR = crate::Reg<ch1dr::CH1DR_SPEC>;
#[doc = "Channel 1 data register"]
pub mod ch1dr;
#[doc = "CH2DR (r) register accessor: an alias for `Reg<CH2DR_SPEC>`"]
pub type CH2DR = crate::Reg<ch2dr::CH2DR_SPEC>;
#[doc = "Channel 2 data register"]
pub mod ch2dr;
#[doc = "CH3DR (r) register accessor: an alias for `Reg<CH3DR_SPEC>`"]
pub type CH3DR = crate::Reg<ch3dr::CH3DR_SPEC>;
#[doc = "Channel 3 data register"]
pub mod ch3dr;
#[doc = "CH4DR (r) register accessor: an alias for `Reg<CH4DR_SPEC>`"]
pub type CH4DR = crate::Reg<ch4dr::CH4DR_SPEC>;
#[doc = "Channel 4 data register"]
pub mod ch4dr;
#[doc = "CH5DR (r) register accessor: an alias for `Reg<CH5DR_SPEC>`"]
pub type CH5DR = crate::Reg<ch5dr::CH5DR_SPEC>;
#[doc = "Channel 5 data register"]
pub mod ch5dr;
#[doc = "CH6DR (r) register accessor: an alias for `Reg<CH6DR_SPEC>`"]
pub type CH6DR = crate::Reg<ch6dr::CH6DR_SPEC>;
#[doc = "Channel 6 data register"]
pub mod ch6dr;
#[doc = "CH7DR (r) register accessor: an alias for `Reg<CH7DR_SPEC>`"]
pub type CH7DR = crate::Reg<ch7dr::CH7DR_SPEC>;
#[doc = "Channel 7 data register"]
pub mod ch7dr;
#[doc = "CH8DR (r) register accessor: an alias for `Reg<CH8DR_SPEC>`"]
pub type CH8DR = crate::Reg<ch8dr::CH8DR_SPEC>;
#[doc = "Channel 8 data register"]
pub mod ch8dr;
#[doc = "CH9DR (r) register accessor: an alias for `Reg<CH9DR_SPEC>`"]
pub type CH9DR = crate::Reg<ch9dr::CH9DR_SPEC>;
#[doc = "Channel 9 data register"]
pub mod ch9dr;
#[doc = "CH10DR (r) register accessor: an alias for `Reg<CH10DR_SPEC>`"]
pub type CH10DR = crate::Reg<ch10dr::CH10DR_SPEC>;
#[doc = "Channel 10 data register"]
pub mod ch10dr;
#[doc = "CH11DR (r) register accessor: an alias for `Reg<CH11DR_SPEC>`"]
pub type CH11DR = crate::Reg<ch11dr::CH11DR_SPEC>;
#[doc = "Channel 11 data register"]
pub mod ch11dr;
#[doc = "CH12DR (r) register accessor: an alias for `Reg<CH12DR_SPEC>`"]
pub type CH12DR = crate::Reg<ch12dr::CH12DR_SPEC>;
#[doc = "Channel 12 data register"]
pub mod ch12dr;
#[doc = "CH13DR (r) register accessor: an alias for `Reg<CH13DR_SPEC>`"]
pub type CH13DR = crate::Reg<ch13dr::CH13DR_SPEC>;
#[doc = "Channel 13 data register"]
pub mod ch13dr;
#[doc = "CH14DR (r) register accessor: an alias for `Reg<CH14DR_SPEC>`"]
pub type CH14DR = crate::Reg<ch14dr::CH14DR_SPEC>;
#[doc = "Channel 14 data register"]
pub mod ch14dr;
#[doc = "CH15DR (r) register accessor: an alias for `Reg<CH15DR_SPEC>`"]
pub type CH15DR = crate::Reg<ch15dr::CH15DR_SPEC>;
#[doc = "Channel 15 data register"]
pub mod ch15dr;
#[doc = "STA_EXT (r) register accessor: an alias for `Reg<STA_EXT_SPEC>`"]
pub type STA_EXT = crate::Reg<sta_ext::STA_EXT_SPEC>;
#[doc = "Extend state register"]
pub mod sta_ext;
#[doc = "CHANY0 (rw) register accessor: an alias for `Reg<CHANY0_SPEC>`"]
pub type CHANY0 = crate::Reg<chany0::CHANY0_SPEC>;
#[doc = "Arbitrary channel channel selection register 0"]
pub mod chany0;
#[doc = "CHANY1 (rw) register accessor: an alias for `Reg<CHANY1_SPEC>`"]
pub type CHANY1 = crate::Reg<chany1::CHANY1_SPEC>;
#[doc = "Arbitrary channel channel selection register 1"]
pub mod chany1;
#[doc = "ANY_CFG (rw) register accessor: an alias for `Reg<ANY_CFG_SPEC>`"]
pub type ANY_CFG = crate::Reg<any_cfg::ANY_CFG_SPEC>;
#[doc = "Arbitrary channel configuration register"]
pub mod any_cfg;
#[doc = "ANY_CR (rw) register accessor: an alias for `Reg<ANY_CR_SPEC>`"]
pub type ANY_CR = crate::Reg<any_cr::ANY_CR_SPEC>;
#[doc = "Arbitrary channel control register"]
pub mod any_cr;
#[doc = "SMPR1 (rw) register accessor: an alias for `Reg<SMPR1_SPEC>`"]
pub type SMPR1 = crate::Reg<smpr1::SMPR1_SPEC>;
#[doc = "Any channel configuration register 1"]
pub mod smpr1;
#[doc = "SMPR2 (rw) register accessor: an alias for `Reg<SMPR2_SPEC>`"]
pub type SMPR2 = crate::Reg<smpr2::SMPR2_SPEC>;
#[doc = "Any channel configuration register 2"]
pub mod smpr2;
#[doc = "JOFR0 (rw) register accessor: an alias for `Reg<JOFR0_SPEC>`"]
pub type JOFR0 = crate::Reg<jofr0::JOFR0_SPEC>;
#[doc = "Injection channe 0 data compensation register"]
pub mod jofr0;
#[doc = "JOFR1 (rw) register accessor: an alias for `Reg<JOFR1_SPEC>`"]
pub type JOFR1 = crate::Reg<jofr1::JOFR1_SPEC>;
#[doc = "Injection channe 1 data compensation register"]
pub mod jofr1;
#[doc = "JOFR2 (rw) register accessor: an alias for `Reg<JOFR2_SPEC>`"]
pub type JOFR2 = crate::Reg<jofr2::JOFR2_SPEC>;
#[doc = "Injection channe 2 data compensation register"]
pub mod jofr2;
#[doc = "JOFR3 (rw) register accessor: an alias for `Reg<JOFR3_SPEC>`"]
pub type JOFR3 = crate::Reg<jofr3::JOFR3_SPEC>;
#[doc = "Injection channe 3 data compensation register"]
pub mod jofr3;
#[doc = "JSQR (rw) register accessor: an alias for `Reg<JSQR_SPEC>`"]
pub type JSQR = crate::Reg<jsqr::JSQR_SPEC>;
#[doc = "Injection sequence register"]
pub mod jsqr;
#[doc = "JDATA (r) register accessor: an alias for `Reg<JDATA_SPEC>`"]
pub type JDATA = crate::Reg<jdata::JDATA_SPEC>;
#[doc = "Injection data register"]
pub mod jdata;
#[doc = "JDR0 (r) register accessor: an alias for `Reg<JDR0_SPEC>`"]
pub type JDR0 = crate::Reg<jdr0::JDR0_SPEC>;
#[doc = "Injection data register"]
pub mod jdr0;
#[doc = "JDR1 (r) register accessor: an alias for `Reg<JDR1_SPEC>`"]
pub type JDR1 = crate::Reg<jdr1::JDR1_SPEC>;
#[doc = "Injection data register"]
pub mod jdr1;
#[doc = "JDR2 (r) register accessor: an alias for `Reg<JDR2_SPEC>`"]
pub type JDR2 = crate::Reg<jdr2::JDR2_SPEC>;
#[doc = "Injection data register"]
pub mod jdr2;
#[doc = "JDR3 (r) register accessor: an alias for `Reg<JDR3_SPEC>`"]
pub type JDR3 = crate::Reg<jdr3::JDR3_SPEC>;
#[doc = "Injection data register"]
pub mod jdr3;
