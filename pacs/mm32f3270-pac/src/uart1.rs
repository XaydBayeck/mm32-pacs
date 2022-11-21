#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Transmit data register"]
    pub tdr: TDR,
    #[doc = "0x04 - Receive data register"]
    pub rdr: RDR,
    #[doc = "0x08 - Current status register"]
    pub csr: CSR,
    #[doc = "0x0c - Interrupt status register"]
    pub isr: ISR,
    #[doc = "0x10 - Interrupt enable register"]
    pub ier: IER,
    #[doc = "0x14 - Interrupt Clear Register"]
    pub icr: ICR,
    #[doc = "0x18 - Global control register"]
    pub gcr: GCR,
    #[doc = "0x1c - common control register"]
    pub ccr: CCR,
    #[doc = "0x20 - Baud rate register"]
    pub brr: BRR,
    #[doc = "0x24 - Fractional baud rate register"]
    pub fra: FRA,
    #[doc = "0x28 - Receive Address Register"]
    pub rxaddr: RXADDR,
    #[doc = "0x2c - Receive Mask Registe"]
    pub rxmask: RXMASK,
    #[doc = "0x30 - ISO7816 configure register"]
    pub scr: SCR,
    #[doc = "0x34 - Data length register"]
    pub idlr: IDLR,
    #[doc = "0x38 - Automatic Baud Rate Register"]
    pub abrcr: ABRCR,
    #[doc = "0x3c - IrDA configure register"]
    pub irda: IRDA,
}
#[doc = "TDR (rw) register accessor: an alias for `Reg<TDR_SPEC>`"]
pub type TDR = crate::Reg<tdr::TDR_SPEC>;
#[doc = "Transmit data register"]
pub mod tdr;
#[doc = "RDR (r) register accessor: an alias for `Reg<RDR_SPEC>`"]
pub type RDR = crate::Reg<rdr::RDR_SPEC>;
#[doc = "Receive data register"]
pub mod rdr;
#[doc = "CSR (r) register accessor: an alias for `Reg<CSR_SPEC>`"]
pub type CSR = crate::Reg<csr::CSR_SPEC>;
#[doc = "Current status register"]
pub mod csr;
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
#[doc = "Interrupt Clear Register"]
pub mod icr;
#[doc = "GCR (rw) register accessor: an alias for `Reg<GCR_SPEC>`"]
pub type GCR = crate::Reg<gcr::GCR_SPEC>;
#[doc = "Global control register"]
pub mod gcr;
#[doc = "CCR (rw) register accessor: an alias for `Reg<CCR_SPEC>`"]
pub type CCR = crate::Reg<ccr::CCR_SPEC>;
#[doc = "common control register"]
pub mod ccr;
#[doc = "BRR (rw) register accessor: an alias for `Reg<BRR_SPEC>`"]
pub type BRR = crate::Reg<brr::BRR_SPEC>;
#[doc = "Baud rate register"]
pub mod brr;
#[doc = "FRA (rw) register accessor: an alias for `Reg<FRA_SPEC>`"]
pub type FRA = crate::Reg<fra::FRA_SPEC>;
#[doc = "Fractional baud rate register"]
pub mod fra;
#[doc = "RXADDR (rw) register accessor: an alias for `Reg<RXADDR_SPEC>`"]
pub type RXADDR = crate::Reg<rxaddr::RXADDR_SPEC>;
#[doc = "Receive Address Register"]
pub mod rxaddr;
#[doc = "RXMASK (rw) register accessor: an alias for `Reg<RXMASK_SPEC>`"]
pub type RXMASK = crate::Reg<rxmask::RXMASK_SPEC>;
#[doc = "Receive Mask Registe"]
pub mod rxmask;
#[doc = "SCR (rw) register accessor: an alias for `Reg<SCR_SPEC>`"]
pub type SCR = crate::Reg<scr::SCR_SPEC>;
#[doc = "ISO7816 configure register"]
pub mod scr;
#[doc = "IDLR (rw) register accessor: an alias for `Reg<IDLR_SPEC>`"]
pub type IDLR = crate::Reg<idlr::IDLR_SPEC>;
#[doc = "Data length register"]
pub mod idlr;
#[doc = "ABRCR (rw) register accessor: an alias for `Reg<ABRCR_SPEC>`"]
pub type ABRCR = crate::Reg<abrcr::ABRCR_SPEC>;
#[doc = "Automatic Baud Rate Register"]
pub mod abrcr;
#[doc = "IRDA (rw) register accessor: an alias for `Reg<IRDA_SPEC>`"]
pub type IRDA = crate::Reg<irda::IRDA_SPEC>;
#[doc = "IrDA configure register"]
pub mod irda;
