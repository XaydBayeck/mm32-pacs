#[doc = "Register `ISR` reader"]
pub struct R(crate::R<ISR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ISR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ISR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ISR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TX` reader - Transmit FIFO avialable interrupt flag bit"]
pub type TX_R = crate::BitReader<bool>;
#[doc = "Field `RX` reader - Receive data available interrupt flag bit"]
pub type RX_R = crate::BitReader<bool>;
#[doc = "Field `UNDERRUN` reader - SPI underrun interrupt flag bit"]
pub type UNDERRUN_R = crate::BitReader<bool>;
#[doc = "Field `RXOERR` reader - Receive overrun error interrupt flag bit"]
pub type RXOERR_R = crate::BitReader<bool>;
#[doc = "Field `RXMATCH` reader - Receive data match the RDNR number"]
pub type RXMATCH_R = crate::BitReader<bool>;
#[doc = "Field `RXFULL` reader - RX FIFO full interrupt flag bit"]
pub type RXFULL_R = crate::BitReader<bool>;
#[doc = "Field `TXEPT` reader - Transmitter empty interrupt flag bit"]
pub type TXEPT_R = crate::BitReader<bool>;
#[doc = "Field `FRE` reader - I2S frame transmission error flag bit (only valid in slave mode)"]
pub type FRE_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Transmit FIFO avialable interrupt flag bit"]
    #[inline(always)]
    pub fn tx(&self) -> TX_R {
        TX_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Receive data available interrupt flag bit"]
    #[inline(always)]
    pub fn rx(&self) -> RX_R {
        RX_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SPI underrun interrupt flag bit"]
    #[inline(always)]
    pub fn underrun(&self) -> UNDERRUN_R {
        UNDERRUN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Receive overrun error interrupt flag bit"]
    #[inline(always)]
    pub fn rxoerr(&self) -> RXOERR_R {
        RXOERR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Receive data match the RDNR number"]
    #[inline(always)]
    pub fn rxmatch(&self) -> RXMATCH_R {
        RXMATCH_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RX FIFO full interrupt flag bit"]
    #[inline(always)]
    pub fn rxfull(&self) -> RXFULL_R {
        RXFULL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Transmitter empty interrupt flag bit"]
    #[inline(always)]
    pub fn txept(&self) -> TXEPT_R {
        TXEPT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - I2S frame transmission error flag bit (only valid in slave mode)"]
    #[inline(always)]
    pub fn fre(&self) -> FRE_R {
        FRE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "Interrupt status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isr](index.html) module"]
pub struct ISR_SPEC;
impl crate::RegisterSpec for ISR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [isr::R](R) reader structure"]
impl crate::Readable for ISR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ISR to value 0"]
impl crate::Resettable for ISR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
