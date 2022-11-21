#[doc = "Register `DMA` reader"]
pub struct R(crate::R<DMA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMA` writer"]
pub struct W(crate::W<DMA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA_SPEC>;
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
impl From<crate::W<DMA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXEN` reader - Receive DMA enable"]
pub type RXEN_R = crate::BitReader<bool>;
#[doc = "Field `RXEN` writer - Receive DMA enable"]
pub type RXEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_SPEC, bool, O>;
#[doc = "Field `TXEN` reader - Transmit DMA enable"]
pub type TXEN_R = crate::BitReader<bool>;
#[doc = "Field `TXEN` writer - Transmit DMA enable"]
pub type TXEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Receive DMA enable"]
    #[inline(always)]
    pub fn rxen(&self) -> RXEN_R {
        RXEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit DMA enable"]
    #[inline(always)]
    pub fn txen(&self) -> TXEN_R {
        TXEN_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Receive DMA enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxen(&mut self) -> RXEN_W<0> {
        RXEN_W::new(self)
    }
    #[doc = "Bit 1 - Transmit DMA enable"]
    #[inline(always)]
    #[must_use]
    pub fn txen(&mut self) -> TXEN_W<1> {
        TXEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma](index.html) module"]
pub struct DMA_SPEC;
impl crate::RegisterSpec for DMA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma::R](R) reader structure"]
impl crate::Readable for DMA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dma::W](W) writer structure"]
impl crate::Writable for DMA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMA to value 0"]
impl crate::Resettable for DMA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
