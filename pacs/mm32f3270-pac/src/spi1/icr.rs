#[doc = "Register `ICR` reader"]
pub struct R(crate::R<ICR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ICR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ICR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ICR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ICR` writer"]
pub struct W(crate::W<ICR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ICR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<ICR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ICR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TX` reader - Transmitter FIFO empty interrupt clear bit"]
pub type TX_R = crate::BitReader<bool>;
#[doc = "Field `TX` writer - Transmitter FIFO empty interrupt clear bit"]
pub type TX_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
#[doc = "Field `RX` reader - Receive interrupt clear bit"]
pub type RX_R = crate::BitReader<bool>;
#[doc = "Field `RX` writer - Receive interrupt clear bit"]
pub type RX_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
#[doc = "Field `UNDERRUN` reader - Transmitter underrun interrupt clear bit(SPI slave mode only)"]
pub type UNDERRUN_R = crate::BitReader<bool>;
#[doc = "Field `UNDERRUN` writer - Transmitter underrun interrupt clear bit(SPI slave mode only)"]
pub type UNDERRUN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
#[doc = "Field `RXOERR` reader - Overrun error interrupt clear bit"]
pub type RXOERR_R = crate::BitReader<bool>;
#[doc = "Field `RXOERR` writer - Overrun error interrupt clear bit"]
pub type RXOERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
#[doc = "Field `RXMATCH` reader - Receive completed interrupt clear bit"]
pub type RXMATCH_R = crate::BitReader<bool>;
#[doc = "Field `RXMATCH` writer - Receive completed interrupt clear bit"]
pub type RXMATCH_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
#[doc = "Field `RXFULL` reader - Receiver buffer full interrupt clear bit"]
pub type RXFULL_R = crate::BitReader<bool>;
#[doc = "Field `RXFULL` writer - Receiver buffer full interrupt clear bit"]
pub type RXFULL_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
#[doc = "Field `TXEPT` reader - Transmitter empty interrupt clear bit"]
pub type TXEPT_R = crate::BitReader<bool>;
#[doc = "Field `TXEPT` writer - Transmitter empty interrupt clear bit"]
pub type TXEPT_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
#[doc = "Field `FRE` reader - I2S frame transmission error interrupt clear"]
pub type FRE_R = crate::BitReader<bool>;
#[doc = "Field `FRE` writer - I2S frame transmission error interrupt clear"]
pub type FRE_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Transmitter FIFO empty interrupt clear bit"]
    #[inline(always)]
    pub fn tx(&self) -> TX_R {
        TX_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Receive interrupt clear bit"]
    #[inline(always)]
    pub fn rx(&self) -> RX_R {
        RX_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmitter underrun interrupt clear bit(SPI slave mode only)"]
    #[inline(always)]
    pub fn underrun(&self) -> UNDERRUN_R {
        UNDERRUN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Overrun error interrupt clear bit"]
    #[inline(always)]
    pub fn rxoerr(&self) -> RXOERR_R {
        RXOERR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Receive completed interrupt clear bit"]
    #[inline(always)]
    pub fn rxmatch(&self) -> RXMATCH_R {
        RXMATCH_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Receiver buffer full interrupt clear bit"]
    #[inline(always)]
    pub fn rxfull(&self) -> RXFULL_R {
        RXFULL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Transmitter empty interrupt clear bit"]
    #[inline(always)]
    pub fn txept(&self) -> TXEPT_R {
        TXEPT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - I2S frame transmission error interrupt clear"]
    #[inline(always)]
    pub fn fre(&self) -> FRE_R {
        FRE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transmitter FIFO empty interrupt clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn tx(&mut self) -> TX_W<0> {
        TX_W::new(self)
    }
    #[doc = "Bit 1 - Receive interrupt clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn rx(&mut self) -> RX_W<1> {
        RX_W::new(self)
    }
    #[doc = "Bit 2 - Transmitter underrun interrupt clear bit(SPI slave mode only)"]
    #[inline(always)]
    #[must_use]
    pub fn underrun(&mut self) -> UNDERRUN_W<2> {
        UNDERRUN_W::new(self)
    }
    #[doc = "Bit 3 - Overrun error interrupt clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn rxoerr(&mut self) -> RXOERR_W<3> {
        RXOERR_W::new(self)
    }
    #[doc = "Bit 4 - Receive completed interrupt clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn rxmatch(&mut self) -> RXMATCH_W<4> {
        RXMATCH_W::new(self)
    }
    #[doc = "Bit 5 - Receiver buffer full interrupt clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn rxfull(&mut self) -> RXFULL_W<5> {
        RXFULL_W::new(self)
    }
    #[doc = "Bit 6 - Transmitter empty interrupt clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn txept(&mut self) -> TXEPT_W<6> {
        TXEPT_W::new(self)
    }
    #[doc = "Bit 7 - I2S frame transmission error interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn fre(&mut self) -> FRE_W<7> {
        FRE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt clear register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icr](index.html) module"]
pub struct ICR_SPEC;
impl crate::RegisterSpec for ICR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [icr::R](R) reader structure"]
impl crate::Readable for ICR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [icr::W](W) writer structure"]
impl crate::Writable for ICR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ICR to value 0"]
impl crate::Resettable for ICR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
