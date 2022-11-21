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
#[doc = "Field `TXICLR` reader - Transmit buffer empty interrupt clear bit"]
pub type TXICLR_R = crate::BitReader<bool>;
#[doc = "Field `TXICLR` writer - Transmit buffer empty interrupt clear bit"]
pub type TXICLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
#[doc = "Field `RXICLR` reader - Receive interrupt clear bit"]
pub type RXICLR_R = crate::BitReader<bool>;
#[doc = "Field `RXICLR` writer - Receive interrupt clear bit"]
pub type RXICLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
#[doc = "Field `TXC_ICLR` reader - Transmit complete interrupt clear bit"]
pub type TXC_ICLR_R = crate::BitReader<bool>;
#[doc = "Field `TXC_ICLR` writer - Transmit complete interrupt clear bit"]
pub type TXC_ICLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
#[doc = "Field `RXOERR_ICLR` reader - Receive overflow error interrupt clear bit"]
pub type RXOERR_ICLR_R = crate::BitReader<bool>;
#[doc = "Field `RXOERR_ICLR` writer - Receive overflow error interrupt clear bit"]
pub type RXOERR_ICLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
#[doc = "Field `RXPERR_ICLR` reader - Parity error interrupt clear bit"]
pub type RXPERR_ICLR_R = crate::BitReader<bool>;
#[doc = "Field `RXPERR_ICLR` writer - Parity error interrupt clear bit"]
pub type RXPERR_ICLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
#[doc = "Field `RXFERR_ICLR` reader - Frame error interrupt clear bit"]
pub type RXFERR_ICLR_R = crate::BitReader<bool>;
#[doc = "Field `RXFERR_ICLR` writer - Frame error interrupt clear bit"]
pub type RXFERR_ICLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
#[doc = "Field `RXBRK_ICLR` reader - Receive frame break interrupt clear bit"]
pub type RXBRK_ICLR_R = crate::BitReader<bool>;
#[doc = "Field `RXBRK_ICLR` writer - Receive frame break interrupt clear bit"]
pub type RXBRK_ICLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
#[doc = "Field `TXBRK_ICLR` reader - Transmit Break Frame Interrupt clear Bit"]
pub type TXBRK_ICLR_R = crate::BitReader<bool>;
#[doc = "Field `TXBRK_ICLR` writer - Transmit Break Frame Interrupt clear Bit"]
pub type TXBRK_ICLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
#[doc = "Field `RXB8_ICLR` reader - Receive Bit 8 Interrupt clear Bit"]
pub type RXB8_ICLR_R = crate::BitReader<bool>;
#[doc = "Field `RXB8_ICLR` writer - Receive Bit 8 Interrupt clear Bit"]
pub type RXB8_ICLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
#[doc = "Field `RXIDLE_ICLR` reader - Receive Bit 8 Interrupt clear Bit"]
pub type RXIDLE_ICLR_R = crate::BitReader<bool>;
#[doc = "Field `RXIDLE_ICLR` writer - Receive Bit 8 Interrupt clear Bit"]
pub type RXIDLE_ICLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
#[doc = "Field `ABREND__ICLR` reader - Auto baud rate end interrupt clear bit"]
pub type ABREND__ICLR_R = crate::BitReader<bool>;
#[doc = "Field `ABREND__ICLR` writer - Auto baud rate end interrupt clear bit"]
pub type ABREND__ICLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
#[doc = "Field `ABRERR__ICLR` reader - Auto baud rate error interrupt clear bit"]
pub type ABRERR__ICLR_R = crate::BitReader<bool>;
#[doc = "Field `ABRERR__ICLR` writer - Auto baud rate error interrupt clear bit"]
pub type ABRERR__ICLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Transmit buffer empty interrupt clear bit"]
    #[inline(always)]
    pub fn txiclr(&self) -> TXICLR_R {
        TXICLR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Receive interrupt clear bit"]
    #[inline(always)]
    pub fn rxiclr(&self) -> RXICLR_R {
        RXICLR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmit complete interrupt clear bit"]
    #[inline(always)]
    pub fn txc_iclr(&self) -> TXC_ICLR_R {
        TXC_ICLR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Receive overflow error interrupt clear bit"]
    #[inline(always)]
    pub fn rxoerr_iclr(&self) -> RXOERR_ICLR_R {
        RXOERR_ICLR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Parity error interrupt clear bit"]
    #[inline(always)]
    pub fn rxperr_iclr(&self) -> RXPERR_ICLR_R {
        RXPERR_ICLR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Frame error interrupt clear bit"]
    #[inline(always)]
    pub fn rxferr_iclr(&self) -> RXFERR_ICLR_R {
        RXFERR_ICLR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Receive frame break interrupt clear bit"]
    #[inline(always)]
    pub fn rxbrk_iclr(&self) -> RXBRK_ICLR_R {
        RXBRK_ICLR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Transmit Break Frame Interrupt clear Bit"]
    #[inline(always)]
    pub fn txbrk_iclr(&self) -> TXBRK_ICLR_R {
        TXBRK_ICLR_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Receive Bit 8 Interrupt clear Bit"]
    #[inline(always)]
    pub fn rxb8_iclr(&self) -> RXB8_ICLR_R {
        RXB8_ICLR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Receive Bit 8 Interrupt clear Bit"]
    #[inline(always)]
    pub fn rxidle_iclr(&self) -> RXIDLE_ICLR_R {
        RXIDLE_ICLR_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Auto baud rate end interrupt clear bit"]
    #[inline(always)]
    pub fn abrend__iclr(&self) -> ABREND__ICLR_R {
        ABREND__ICLR_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Auto baud rate error interrupt clear bit"]
    #[inline(always)]
    pub fn abrerr__iclr(&self) -> ABRERR__ICLR_R {
        ABRERR__ICLR_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transmit buffer empty interrupt clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn txiclr(&mut self) -> TXICLR_W<0> {
        TXICLR_W::new(self)
    }
    #[doc = "Bit 1 - Receive interrupt clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn rxiclr(&mut self) -> RXICLR_W<1> {
        RXICLR_W::new(self)
    }
    #[doc = "Bit 2 - Transmit complete interrupt clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn txc_iclr(&mut self) -> TXC_ICLR_W<2> {
        TXC_ICLR_W::new(self)
    }
    #[doc = "Bit 3 - Receive overflow error interrupt clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn rxoerr_iclr(&mut self) -> RXOERR_ICLR_W<3> {
        RXOERR_ICLR_W::new(self)
    }
    #[doc = "Bit 4 - Parity error interrupt clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn rxperr_iclr(&mut self) -> RXPERR_ICLR_W<4> {
        RXPERR_ICLR_W::new(self)
    }
    #[doc = "Bit 5 - Frame error interrupt clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn rxferr_iclr(&mut self) -> RXFERR_ICLR_W<5> {
        RXFERR_ICLR_W::new(self)
    }
    #[doc = "Bit 6 - Receive frame break interrupt clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn rxbrk_iclr(&mut self) -> RXBRK_ICLR_W<6> {
        RXBRK_ICLR_W::new(self)
    }
    #[doc = "Bit 7 - Transmit Break Frame Interrupt clear Bit"]
    #[inline(always)]
    #[must_use]
    pub fn txbrk_iclr(&mut self) -> TXBRK_ICLR_W<7> {
        TXBRK_ICLR_W::new(self)
    }
    #[doc = "Bit 8 - Receive Bit 8 Interrupt clear Bit"]
    #[inline(always)]
    #[must_use]
    pub fn rxb8_iclr(&mut self) -> RXB8_ICLR_W<8> {
        RXB8_ICLR_W::new(self)
    }
    #[doc = "Bit 9 - Receive Bit 8 Interrupt clear Bit"]
    #[inline(always)]
    #[must_use]
    pub fn rxidle_iclr(&mut self) -> RXIDLE_ICLR_W<9> {
        RXIDLE_ICLR_W::new(self)
    }
    #[doc = "Bit 10 - Auto baud rate end interrupt clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn abrend__iclr(&mut self) -> ABREND__ICLR_W<10> {
        ABREND__ICLR_W::new(self)
    }
    #[doc = "Bit 11 - Auto baud rate error interrupt clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn abrerr__iclr(&mut self) -> ABRERR__ICLR_W<11> {
        ABRERR__ICLR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Clear Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icr](index.html) module"]
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
