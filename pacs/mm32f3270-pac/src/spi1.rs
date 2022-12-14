#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Transmit data register"]
    pub tdr: TDR,
    #[doc = "0x04 - Receive data register"]
    pub rdr: RDR,
    #[doc = "0x08 - Current status register"]
    pub sr: SR,
    #[doc = "0x0c - Interrupt status register"]
    pub isr: ISR,
    #[doc = "0x10 - Interrupt enable register"]
    pub ier: IER,
    #[doc = "0x14 - Interrupt clear register"]
    pub icr: ICR,
    #[doc = "0x18 - Global control register"]
    pub gcr: GCR,
    #[doc = "0x1c - Current control register"]
    pub ccr: CCR,
    #[doc = "0x20 - Baud rate generation register"]
    pub brr: BRR,
    #[doc = "0x24 - Receive data count register"]
    pub rdnr: RDNR,
    #[doc = "0x28 - Slave chip select register"]
    pub nssr: NSSR,
    _reserved11: [u8; 0x02],
    #[doc = "0x2c - Extent data control register"]
    pub ecr: ECR,
    #[doc = "0x30 - I2S Configuration register"]
    pub i2s_cfgr: I2S_CFGR,
}
#[doc = "TDR (rw) register accessor: an alias for `Reg<TDR_SPEC>`"]
pub type TDR = crate::Reg<tdr::TDR_SPEC>;
#[doc = "Transmit data register"]
pub mod tdr;
#[doc = "RDR (r) register accessor: an alias for `Reg<RDR_SPEC>`"]
pub type RDR = crate::Reg<rdr::RDR_SPEC>;
#[doc = "Receive data register"]
pub mod rdr;
#[doc = "SR (r) register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "Current status register"]
pub mod sr;
#[doc = "ISR (r) register accessor: an alias for `Reg<ISR_SPEC>`"]
pub type ISR = crate::Reg<isr::ISR_SPEC>;
#[doc = "Interrupt status register"]
pub mod isr;
#[doc = "IER (rw) register accessor: an alias for `Reg<IER_SPEC>`"]
pub type IER = crate::Reg<ier::IER_SPEC>;
#[doc = "Interrupt enable register"]
pub mod ier;
#[doc = "ICR (rw) register accessor: an alias for `Reg<ICR_SPEC>`"]
pub type ICR = crate::Reg<icr::ICR_SPEC>;
#[doc = "Interrupt clear register"]
pub mod icr;
#[doc = "GCR (rw) register accessor: an alias for `Reg<GCR_SPEC>`"]
pub type GCR = crate::Reg<gcr::GCR_SPEC>;
#[doc = "Global control register"]
pub mod gcr;
#[doc = "CCR (rw) register accessor: an alias for `Reg<CCR_SPEC>`"]
pub type CCR = crate::Reg<ccr::CCR_SPEC>;
#[doc = "Current control register"]
pub mod ccr;
#[doc = "BRR (rw) register accessor: an alias for `Reg<BRR_SPEC>`"]
pub type BRR = crate::Reg<brr::BRR_SPEC>;
#[doc = "Baud rate generation register"]
pub mod brr;
#[doc = "RDNR (rw) register accessor: an alias for `Reg<RDNR_SPEC>`"]
pub type RDNR = crate::Reg<rdnr::RDNR_SPEC>;
#[doc = "Receive data count register"]
pub mod rdnr;
#[doc = "NSSR (rw) register accessor: an alias for `Reg<NSSR_SPEC>`"]
pub type NSSR = crate::Reg<nssr::NSSR_SPEC>;
#[doc = "Slave chip select register"]
pub mod nssr;
#[doc = "ECR (rw) register accessor: an alias for `Reg<ECR_SPEC>`"]
pub type ECR = crate::Reg<ecr::ECR_SPEC>;
#[doc = "Extent data control register"]
pub mod ecr;
#[doc = "I2S_CFGR (rw) register accessor: an alias for `Reg<I2S_CFGR_SPEC>`"]
pub type I2S_CFGR = crate::Reg<i2s_cfgr::I2S_CFGR_SPEC>;
#[doc = "I2S Configuration register"]
pub mod i2s_cfgr;
