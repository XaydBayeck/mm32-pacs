#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Ethernet DMA bus mode register"]
    pub dmabsr: DMABSR,
    #[doc = "0x04 - Ethernet DMA transmit poll demand register"]
    pub dmatxpdr: DMATXPDR,
    #[doc = "0x08 - EHERNET DMA receive poll demand register"]
    pub dmarxpdr: DMARXPDR,
    #[doc = "0x0c - Ethernet DMA receive descriptor list address register"]
    pub dmatxdsar: DMATXDSAR,
    #[doc = "0x10 - Ethernet DMA transmit descriptor list address register"]
    pub dmatdlar: DMATDLAR,
    #[doc = "0x14 - Ethernet DMA status register"]
    pub dmasr: DMASR,
    #[doc = "0x18 - Ethernet DMA operation mode register"]
    pub dmamdr: DMAMDR,
    #[doc = "0x1c - Ethernet DMA interrupt enable register"]
    pub dmaier: DMAIER,
    #[doc = "0x20 - Ethernet DMA missed frame and buffer overflow counter register"]
    pub dmaflcr: DMAFLCR,
    #[doc = "0x24 - Ethernet Watchdog register"]
    pub dmawdtr: DMAWDTR,
    _reserved10: [u8; 0x20],
    #[doc = "0x48 - Ethernet DMA current host transmit descriptor register"]
    pub dmacuttxdsar: DMACUTTXDSAR,
    _reserved11: [u8; 0x04],
    #[doc = "0x50 - Ethernet DMA current host transmit buffer address register"]
    pub dmacuttxbufr: DMACUTTXBUFR,
    #[doc = "0x54 - Ethernet DMA current host receive buffer address register"]
    pub dmacutrxbufr: DMACUTRXBUFR,
}
#[doc = "DMABSR (rw) register accessor: an alias for `Reg<DMABSR_SPEC>`"]
pub type DMABSR = crate::Reg<dmabsr::DMABSR_SPEC>;
#[doc = "Ethernet DMA bus mode register"]
pub mod dmabsr;
#[doc = "DMATXPDR (rw) register accessor: an alias for `Reg<DMATXPDR_SPEC>`"]
pub type DMATXPDR = crate::Reg<dmatxpdr::DMATXPDR_SPEC>;
#[doc = "Ethernet DMA transmit poll demand register"]
pub mod dmatxpdr;
#[doc = "DMARXPDR (rw) register accessor: an alias for `Reg<DMARXPDR_SPEC>`"]
pub type DMARXPDR = crate::Reg<dmarxpdr::DMARXPDR_SPEC>;
#[doc = "EHERNET DMA receive poll demand register"]
pub mod dmarxpdr;
#[doc = "DMATXDSAR (rw) register accessor: an alias for `Reg<DMATXDSAR_SPEC>`"]
pub type DMATXDSAR = crate::Reg<dmatxdsar::DMATXDSAR_SPEC>;
#[doc = "Ethernet DMA receive descriptor list address register"]
pub mod dmatxdsar;
#[doc = "DMATDLAR (rw) register accessor: an alias for `Reg<DMATDLAR_SPEC>`"]
pub type DMATDLAR = crate::Reg<dmatdlar::DMATDLAR_SPEC>;
#[doc = "Ethernet DMA transmit descriptor list address register"]
pub mod dmatdlar;
#[doc = "DMASR (rw) register accessor: an alias for `Reg<DMASR_SPEC>`"]
pub type DMASR = crate::Reg<dmasr::DMASR_SPEC>;
#[doc = "Ethernet DMA status register"]
pub mod dmasr;
#[doc = "DMAMDR (rw) register accessor: an alias for `Reg<DMAMDR_SPEC>`"]
pub type DMAMDR = crate::Reg<dmamdr::DMAMDR_SPEC>;
#[doc = "Ethernet DMA operation mode register"]
pub mod dmamdr;
#[doc = "DMAIER (rw) register accessor: an alias for `Reg<DMAIER_SPEC>`"]
pub type DMAIER = crate::Reg<dmaier::DMAIER_SPEC>;
#[doc = "Ethernet DMA interrupt enable register"]
pub mod dmaier;
#[doc = "DMAFLCR (r) register accessor: an alias for `Reg<DMAFLCR_SPEC>`"]
pub type DMAFLCR = crate::Reg<dmaflcr::DMAFLCR_SPEC>;
#[doc = "Ethernet DMA missed frame and buffer overflow counter register"]
pub mod dmaflcr;
#[doc = "DMAWDTR (r) register accessor: an alias for `Reg<DMAWDTR_SPEC>`"]
pub type DMAWDTR = crate::Reg<dmawdtr::DMAWDTR_SPEC>;
#[doc = "Ethernet Watchdog register"]
pub mod dmawdtr;
#[doc = "DMACUTTXDSAR (r) register accessor: an alias for `Reg<DMACUTTXDSAR_SPEC>`"]
pub type DMACUTTXDSAR = crate::Reg<dmacuttxdsar::DMACUTTXDSAR_SPEC>;
#[doc = "Ethernet DMA current host transmit descriptor register"]
pub mod dmacuttxdsar;
#[doc = "DMACUTTXBUFR (r) register accessor: an alias for `Reg<DMACUTTXBUFR_SPEC>`"]
pub type DMACUTTXBUFR = crate::Reg<dmacuttxbufr::DMACUTTXBUFR_SPEC>;
#[doc = "Ethernet DMA current host transmit buffer address register"]
pub mod dmacuttxbufr;
#[doc = "DMACUTRXBUFR (r) register accessor: an alias for `Reg<DMACUTRXBUFR_SPEC>`"]
pub type DMACUTRXBUFR = crate::Reg<dmacutrxbufr::DMACUTRXBUFR_SPEC>;
#[doc = "Ethernet DMA current host receive buffer address register"]
pub mod dmacutrxbufr;
