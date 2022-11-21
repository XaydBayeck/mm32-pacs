#[doc = "Register `GCR` reader"]
pub struct R(crate::R<GCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GCR` writer"]
pub struct W(crate::W<GCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GCR_SPEC>;
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
impl From<crate::W<GCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UARTEN` reader - UART mode selection bit"]
pub type UARTEN_R = crate::BitReader<bool>;
#[doc = "Field `UARTEN` writer - UART mode selection bit"]
pub type UARTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, GCR_SPEC, bool, O>;
#[doc = "Field `DMAMODE` reader - DMA mode selection bit"]
pub type DMAMODE_R = crate::BitReader<bool>;
#[doc = "Field `DMAMODE` writer - DMA mode selection bit"]
pub type DMAMODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GCR_SPEC, bool, O>;
#[doc = "Field `AUTOFLOWEN` reader - Automatic flow control enable bit"]
pub type AUTOFLOWEN_R = crate::BitReader<bool>;
#[doc = "Field `AUTOFLOWEN` writer - Automatic flow control enable bit"]
pub type AUTOFLOWEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, GCR_SPEC, bool, O>;
#[doc = "Field `RXEN` reader - Enable receive"]
pub type RXEN_R = crate::BitReader<bool>;
#[doc = "Field `RXEN` writer - Enable receive"]
pub type RXEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, GCR_SPEC, bool, O>;
#[doc = "Field `TXEN` reader - Enable transmit"]
pub type TXEN_R = crate::BitReader<bool>;
#[doc = "Field `TXEN` writer - Enable transmit"]
pub type TXEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, GCR_SPEC, bool, O>;
#[doc = "Field `SELB8` reader - Select bit8"]
pub type SELB8_R = crate::BitReader<bool>;
#[doc = "Field `SELB8` writer - Select bit8"]
pub type SELB8_W<'a, const O: u8> = crate::BitWriter<'a, u32, GCR_SPEC, bool, O>;
#[doc = "Field `SWAP` reader - change swap"]
pub type SWAP_R = crate::BitReader<bool>;
#[doc = "Field `SWAP` writer - change swap"]
pub type SWAP_W<'a, const O: u8> = crate::BitWriter<'a, u32, GCR_SPEC, bool, O>;
#[doc = "Field `RXTOG` reader - Toggle RX"]
pub type RXTOG_R = crate::BitReader<bool>;
#[doc = "Field `RXTOG` writer - Toggle RX"]
pub type RXTOG_W<'a, const O: u8> = crate::BitWriter<'a, u32, GCR_SPEC, bool, O>;
#[doc = "Field `TXTOG` reader - Toggle TX"]
pub type TXTOG_R = crate::BitReader<bool>;
#[doc = "Field `TXTOG` writer - Toggle TX"]
pub type TXTOG_W<'a, const O: u8> = crate::BitWriter<'a, u32, GCR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - UART mode selection bit"]
    #[inline(always)]
    pub fn uarten(&self) -> UARTEN_R {
        UARTEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMA mode selection bit"]
    #[inline(always)]
    pub fn dmamode(&self) -> DMAMODE_R {
        DMAMODE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Automatic flow control enable bit"]
    #[inline(always)]
    pub fn autoflowen(&self) -> AUTOFLOWEN_R {
        AUTOFLOWEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable receive"]
    #[inline(always)]
    pub fn rxen(&self) -> RXEN_R {
        RXEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable transmit"]
    #[inline(always)]
    pub fn txen(&self) -> TXEN_R {
        TXEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - Select bit8"]
    #[inline(always)]
    pub fn selb8(&self) -> SELB8_R {
        SELB8_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - change swap"]
    #[inline(always)]
    pub fn swap(&self) -> SWAP_R {
        SWAP_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Toggle RX"]
    #[inline(always)]
    pub fn rxtog(&self) -> RXTOG_R {
        RXTOG_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Toggle TX"]
    #[inline(always)]
    pub fn txtog(&self) -> TXTOG_R {
        TXTOG_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - UART mode selection bit"]
    #[inline(always)]
    #[must_use]
    pub fn uarten(&mut self) -> UARTEN_W<0> {
        UARTEN_W::new(self)
    }
    #[doc = "Bit 1 - DMA mode selection bit"]
    #[inline(always)]
    #[must_use]
    pub fn dmamode(&mut self) -> DMAMODE_W<1> {
        DMAMODE_W::new(self)
    }
    #[doc = "Bit 2 - Automatic flow control enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn autoflowen(&mut self) -> AUTOFLOWEN_W<2> {
        AUTOFLOWEN_W::new(self)
    }
    #[doc = "Bit 3 - Enable receive"]
    #[inline(always)]
    #[must_use]
    pub fn rxen(&mut self) -> RXEN_W<3> {
        RXEN_W::new(self)
    }
    #[doc = "Bit 4 - Enable transmit"]
    #[inline(always)]
    #[must_use]
    pub fn txen(&mut self) -> TXEN_W<4> {
        TXEN_W::new(self)
    }
    #[doc = "Bit 7 - Select bit8"]
    #[inline(always)]
    #[must_use]
    pub fn selb8(&mut self) -> SELB8_W<7> {
        SELB8_W::new(self)
    }
    #[doc = "Bit 8 - change swap"]
    #[inline(always)]
    #[must_use]
    pub fn swap(&mut self) -> SWAP_W<8> {
        SWAP_W::new(self)
    }
    #[doc = "Bit 9 - Toggle RX"]
    #[inline(always)]
    #[must_use]
    pub fn rxtog(&mut self) -> RXTOG_W<9> {
        RXTOG_W::new(self)
    }
    #[doc = "Bit 10 - Toggle TX"]
    #[inline(always)]
    #[must_use]
    pub fn txtog(&mut self) -> TXTOG_W<10> {
        TXTOG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Global control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gcr](index.html) module"]
pub struct GCR_SPEC;
impl crate::RegisterSpec for GCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gcr::R](R) reader structure"]
impl crate::Readable for GCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gcr::W](W) writer structure"]
impl crate::Writable for GCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GCR to value 0"]
impl crate::Resettable for GCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
