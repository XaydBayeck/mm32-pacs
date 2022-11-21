#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub cr: CR,
    #[doc = "0x04 - Target Register"]
    pub tar: TAR,
    #[doc = "0x08 - Slave Address Register"]
    pub sar: SAR,
    _reserved3: [u8; 0x04],
    #[doc = "0x10 - Data Command Register"]
    pub dr: DR,
    #[doc = "0x14 - SCL High Period Count for Std. Speed Register"]
    pub sshr: SSHR,
    #[doc = "0x18 - SCL Low Period Count for Std. Speed Register"]
    pub sslr: SSLR,
    #[doc = "0x1c - SCL High Period Count for Fast Speed Register"]
    pub fshr: FSHR,
    #[doc = "0x20 - SCL Low Period Count for Fast Speed Register"]
    pub fslr: FSLR,
    _reserved8: [u8; 0x08],
    #[doc = "0x2c - Interrupt Status Register"]
    pub isr: ISR,
    #[doc = "0x30 - Interrupt Mask Register"]
    pub imr: IMR,
    #[doc = "0x34 - RAW Interrupt Status Register"]
    pub rawisr: RAWISR,
    #[doc = "0x38 - Receive FIFO Threshold Level Register"]
    pub rxtlr: RXTLR,
    #[doc = "0x3c - Transmit FIFO Threshold Level Register"]
    pub txtlr: TXTLR,
    #[doc = "0x40 - Clear All Interrupt Register"]
    pub icr: ICR,
    #[doc = "0x44 - Clear RX_UNDER Interrupt Register"]
    pub rx_under: RX_UNDER,
    #[doc = "0x48 - Clear RX_OVER Interrupt Register"]
    pub rx_over: RX_OVER,
    #[doc = "0x4c - Clear TX_OVER Interrupt Register"]
    pub tx_over: TX_OVER,
    #[doc = "0x50 - Clear RD_REQ Interrupt Register"]
    pub rd_req: RD_REQ,
    #[doc = "0x54 - Clear TX_ABRT Interrupt Register"]
    pub tx_abrt: TX_ABRT,
    #[doc = "0x58 - Clear RX_DONE Interrupt Register"]
    pub rx_done: RX_DONE,
    #[doc = "0x5c - Clear ACTIVITY Interrupt Register"]
    pub activ: ACTIV,
    #[doc = "0x60 - Clear STOP_DET Interrupt Register"]
    pub stop: STOP,
    #[doc = "0x64 - Clear START_DET Interrupt Register"]
    pub start: START,
    #[doc = "0x68 - Clear GEN_CALL Interrupt Register"]
    pub gc: GC,
    #[doc = "0x6c - Enable Register"]
    pub enr: ENR,
    #[doc = "0x70 - Status Register"]
    pub sr: SR,
    #[doc = "0x74 - Transmit FIFO Level Register"]
    pub txflr: TXFLR,
    #[doc = "0x78 - Receive FIFO Level Register"]
    pub rxflr: RXFLR,
    #[doc = "0x7c - SDA Hold Time Register"]
    pub hold: HOLD,
    _reserved29: [u8; 0x08],
    #[doc = "0x88 - DMA Control Register"]
    pub dma: DMA,
    _reserved30: [u8; 0x08],
    #[doc = "0x94 - SDA Setup Time Register"]
    pub setup: SETUP,
    #[doc = "0x98 - ACK General Call Register"]
    pub gcr: GCR,
    _reserved32: [u8; 0x14],
    #[doc = "0xb0 - Slave Address Mask Register"]
    pub slvmask: SLVMASK,
    #[doc = "0xb4 - Receiver Address Register"]
    pub slvrcvaddr: SLVRCVADDR,
}
#[doc = "CR (rw) register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "Control Register"]
pub mod cr;
#[doc = "TAR (rw) register accessor: an alias for `Reg<TAR_SPEC>`"]
pub type TAR = crate::Reg<tar::TAR_SPEC>;
#[doc = "Target Register"]
pub mod tar;
#[doc = "SAR (rw) register accessor: an alias for `Reg<SAR_SPEC>`"]
pub type SAR = crate::Reg<sar::SAR_SPEC>;
#[doc = "Slave Address Register"]
pub mod sar;
#[doc = "DR (rw) register accessor: an alias for `Reg<DR_SPEC>`"]
pub type DR = crate::Reg<dr::DR_SPEC>;
#[doc = "Data Command Register"]
pub mod dr;
#[doc = "SSHR (rw) register accessor: an alias for `Reg<SSHR_SPEC>`"]
pub type SSHR = crate::Reg<sshr::SSHR_SPEC>;
#[doc = "SCL High Period Count for Std. Speed Register"]
pub mod sshr;
#[doc = "SSLR (rw) register accessor: an alias for `Reg<SSLR_SPEC>`"]
pub type SSLR = crate::Reg<sslr::SSLR_SPEC>;
#[doc = "SCL Low Period Count for Std. Speed Register"]
pub mod sslr;
#[doc = "FSHR (rw) register accessor: an alias for `Reg<FSHR_SPEC>`"]
pub type FSHR = crate::Reg<fshr::FSHR_SPEC>;
#[doc = "SCL High Period Count for Fast Speed Register"]
pub mod fshr;
#[doc = "FSLR (rw) register accessor: an alias for `Reg<FSLR_SPEC>`"]
pub type FSLR = crate::Reg<fslr::FSLR_SPEC>;
#[doc = "SCL Low Period Count for Fast Speed Register"]
pub mod fslr;
#[doc = "ISR (r) register accessor: an alias for `Reg<ISR_SPEC>`"]
pub type ISR = crate::Reg<isr::ISR_SPEC>;
#[doc = "Interrupt Status Register"]
pub mod isr;
#[doc = "IMR (rw) register accessor: an alias for `Reg<IMR_SPEC>`"]
pub type IMR = crate::Reg<imr::IMR_SPEC>;
#[doc = "Interrupt Mask Register"]
pub mod imr;
#[doc = "RAWISR (r) register accessor: an alias for `Reg<RAWISR_SPEC>`"]
pub type RAWISR = crate::Reg<rawisr::RAWISR_SPEC>;
#[doc = "RAW Interrupt Status Register"]
pub mod rawisr;
#[doc = "RXTLR (rw) register accessor: an alias for `Reg<RXTLR_SPEC>`"]
pub type RXTLR = crate::Reg<rxtlr::RXTLR_SPEC>;
#[doc = "Receive FIFO Threshold Level Register"]
pub mod rxtlr;
#[doc = "TXTLR (rw) register accessor: an alias for `Reg<TXTLR_SPEC>`"]
pub type TXTLR = crate::Reg<txtlr::TXTLR_SPEC>;
#[doc = "Transmit FIFO Threshold Level Register"]
pub mod txtlr;
#[doc = "ICR (r) register accessor: an alias for `Reg<ICR_SPEC>`"]
pub type ICR = crate::Reg<icr::ICR_SPEC>;
#[doc = "Clear All Interrupt Register"]
pub mod icr;
#[doc = "RX_UNDER (r) register accessor: an alias for `Reg<RX_UNDER_SPEC>`"]
pub type RX_UNDER = crate::Reg<rx_under::RX_UNDER_SPEC>;
#[doc = "Clear RX_UNDER Interrupt Register"]
pub mod rx_under;
#[doc = "RX_OVER (r) register accessor: an alias for `Reg<RX_OVER_SPEC>`"]
pub type RX_OVER = crate::Reg<rx_over::RX_OVER_SPEC>;
#[doc = "Clear RX_OVER Interrupt Register"]
pub mod rx_over;
#[doc = "TX_OVER (r) register accessor: an alias for `Reg<TX_OVER_SPEC>`"]
pub type TX_OVER = crate::Reg<tx_over::TX_OVER_SPEC>;
#[doc = "Clear TX_OVER Interrupt Register"]
pub mod tx_over;
#[doc = "RD_REQ (r) register accessor: an alias for `Reg<RD_REQ_SPEC>`"]
pub type RD_REQ = crate::Reg<rd_req::RD_REQ_SPEC>;
#[doc = "Clear RD_REQ Interrupt Register"]
pub mod rd_req;
#[doc = "TX_ABRT (r) register accessor: an alias for `Reg<TX_ABRT_SPEC>`"]
pub type TX_ABRT = crate::Reg<tx_abrt::TX_ABRT_SPEC>;
#[doc = "Clear TX_ABRT Interrupt Register"]
pub mod tx_abrt;
#[doc = "RX_DONE (r) register accessor: an alias for `Reg<RX_DONE_SPEC>`"]
pub type RX_DONE = crate::Reg<rx_done::RX_DONE_SPEC>;
#[doc = "Clear RX_DONE Interrupt Register"]
pub mod rx_done;
#[doc = "ACTIV (r) register accessor: an alias for `Reg<ACTIV_SPEC>`"]
pub type ACTIV = crate::Reg<activ::ACTIV_SPEC>;
#[doc = "Clear ACTIVITY Interrupt Register"]
pub mod activ;
#[doc = "STOP (r) register accessor: an alias for `Reg<STOP_SPEC>`"]
pub type STOP = crate::Reg<stop::STOP_SPEC>;
#[doc = "Clear STOP_DET Interrupt Register"]
pub mod stop;
#[doc = "START (r) register accessor: an alias for `Reg<START_SPEC>`"]
pub type START = crate::Reg<start::START_SPEC>;
#[doc = "Clear START_DET Interrupt Register"]
pub mod start;
#[doc = "GC (r) register accessor: an alias for `Reg<GC_SPEC>`"]
pub type GC = crate::Reg<gc::GC_SPEC>;
#[doc = "Clear GEN_CALL Interrupt Register"]
pub mod gc;
#[doc = "ENR (rw) register accessor: an alias for `Reg<ENR_SPEC>`"]
pub type ENR = crate::Reg<enr::ENR_SPEC>;
#[doc = "Enable Register"]
pub mod enr;
#[doc = "SR (r) register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "Status Register"]
pub mod sr;
#[doc = "TXFLR (rw) register accessor: an alias for `Reg<TXFLR_SPEC>`"]
pub type TXFLR = crate::Reg<txflr::TXFLR_SPEC>;
#[doc = "Transmit FIFO Level Register"]
pub mod txflr;
#[doc = "RXFLR (rw) register accessor: an alias for `Reg<RXFLR_SPEC>`"]
pub type RXFLR = crate::Reg<rxflr::RXFLR_SPEC>;
#[doc = "Receive FIFO Level Register"]
pub mod rxflr;
#[doc = "HOLD (rw) register accessor: an alias for `Reg<HOLD_SPEC>`"]
pub type HOLD = crate::Reg<hold::HOLD_SPEC>;
#[doc = "SDA Hold Time Register"]
pub mod hold;
#[doc = "DMA (rw) register accessor: an alias for `Reg<DMA_SPEC>`"]
pub type DMA = crate::Reg<dma::DMA_SPEC>;
#[doc = "DMA Control Register"]
pub mod dma;
#[doc = "SETUP (rw) register accessor: an alias for `Reg<SETUP_SPEC>`"]
pub type SETUP = crate::Reg<setup::SETUP_SPEC>;
#[doc = "SDA Setup Time Register"]
pub mod setup;
#[doc = "GCR (rw) register accessor: an alias for `Reg<GCR_SPEC>`"]
pub type GCR = crate::Reg<gcr::GCR_SPEC>;
#[doc = "ACK General Call Register"]
pub mod gcr;
#[doc = "SLVMASK (rw) register accessor: an alias for `Reg<SLVMASK_SPEC>`"]
pub type SLVMASK = crate::Reg<slvmask::SLVMASK_SPEC>;
#[doc = "Slave Address Mask Register"]
pub mod slvmask;
#[doc = "SLVRCVADDR (r) register accessor: an alias for `Reg<SLVRCVADDR_SPEC>`"]
pub type SLVRCVADDR = crate::Reg<slvrcvaddr::SLVRCVADDR_SPEC>;
#[doc = "Receiver Address Register"]
pub mod slvrcvaddr;
