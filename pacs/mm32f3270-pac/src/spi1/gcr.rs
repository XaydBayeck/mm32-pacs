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
#[doc = "Field `SPIEN` reader - SPI select bit"]
pub type SPIEN_R = crate::BitReader<bool>;
#[doc = "Field `SPIEN` writer - SPI select bit"]
pub type SPIEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, GCR_SPEC, bool, O>;
#[doc = "Field `IEN` reader - SPI interrupt enable bit"]
pub type IEN_R = crate::BitReader<bool>;
#[doc = "Field `IEN` writer - SPI interrupt enable bit"]
pub type IEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, GCR_SPEC, bool, O>;
#[doc = "Field `MODE` reader - Master mode bit"]
pub type MODE_R = crate::BitReader<bool>;
#[doc = "Field `MODE` writer - Master mode bit"]
pub type MODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GCR_SPEC, bool, O>;
#[doc = "Field `TXEN` reader - Transmit enable bit"]
pub type TXEN_R = crate::BitReader<bool>;
#[doc = "Field `TXEN` writer - Transmit enable bit"]
pub type TXEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, GCR_SPEC, bool, O>;
#[doc = "Field `RXEN` reader - Receive enable bit"]
pub type RXEN_R = crate::BitReader<bool>;
#[doc = "Field `RXEN` writer - Receive enable bit"]
pub type RXEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, GCR_SPEC, bool, O>;
#[doc = "Field `RXTLF` reader - RX FIFO trigger level bit"]
pub type RXTLF_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RXTLF` writer - RX FIFO trigger level bit"]
pub type RXTLF_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GCR_SPEC, u8, u8, 2, O>;
#[doc = "Field `TXTLF` reader - TX FIFO trigger level bit"]
pub type TXTLF_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TXTLF` writer - TX FIFO trigger level bit"]
pub type TXTLF_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GCR_SPEC, u8, u8, 2, O>;
#[doc = "Field `DMAEN` reader - DMA access mode enable"]
pub type DMAEN_R = crate::BitReader<bool>;
#[doc = "Field `DMAEN` writer - DMA access mode enable"]
pub type DMAEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, GCR_SPEC, bool, O>;
#[doc = "Field `NSS` reader - NSS select signal that from software and hardware"]
pub type NSS_R = crate::BitReader<bool>;
#[doc = "Field `NSS` writer - NSS select signal that from software and hardware"]
pub type NSS_W<'a, const O: u8> = crate::BitWriter<'a, u32, GCR_SPEC, bool, O>;
#[doc = "Field `DWSEL` reader - Valid byte or double-word data select signal"]
pub type DWSEL_R = crate::BitReader<bool>;
#[doc = "Field `DWSEL` writer - Valid byte or double-word data select signal"]
pub type DWSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, GCR_SPEC, bool, O>;
#[doc = "Field `NSSTOG` reader - NSS selection signal is automatically flipped"]
pub type NSSTOG_R = crate::BitReader<bool>;
#[doc = "Field `NSSTOG` writer - NSS selection signal is automatically flipped"]
pub type NSSTOG_W<'a, const O: u8> = crate::BitWriter<'a, u32, GCR_SPEC, bool, O>;
#[doc = "Field `PAD_SEL` reader - Bus mapping transformation."]
pub type PAD_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PAD_SEL` writer - Bus mapping transformation."]
pub type PAD_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GCR_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bit 0 - SPI select bit"]
    #[inline(always)]
    pub fn spien(&self) -> SPIEN_R {
        SPIEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SPI interrupt enable bit"]
    #[inline(always)]
    pub fn ien(&self) -> IEN_R {
        IEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Master mode bit"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmit enable bit"]
    #[inline(always)]
    pub fn txen(&self) -> TXEN_R {
        TXEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Receive enable bit"]
    #[inline(always)]
    pub fn rxen(&self) -> RXEN_R {
        RXEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - RX FIFO trigger level bit"]
    #[inline(always)]
    pub fn rxtlf(&self) -> RXTLF_R {
        RXTLF_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bits 7:8 - TX FIFO trigger level bit"]
    #[inline(always)]
    pub fn txtlf(&self) -> TXTLF_R {
        TXTLF_R::new(((self.bits >> 7) & 3) as u8)
    }
    #[doc = "Bit 9 - DMA access mode enable"]
    #[inline(always)]
    pub fn dmaen(&self) -> DMAEN_R {
        DMAEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - NSS select signal that from software and hardware"]
    #[inline(always)]
    pub fn nss(&self) -> NSS_R {
        NSS_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Valid byte or double-word data select signal"]
    #[inline(always)]
    pub fn dwsel(&self) -> DWSEL_R {
        DWSEL_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - NSS selection signal is automatically flipped"]
    #[inline(always)]
    pub fn nsstog(&self) -> NSSTOG_R {
        NSSTOG_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:17 - Bus mapping transformation."]
    #[inline(always)]
    pub fn pad_sel(&self) -> PAD_SEL_R {
        PAD_SEL_R::new(((self.bits >> 13) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - SPI select bit"]
    #[inline(always)]
    #[must_use]
    pub fn spien(&mut self) -> SPIEN_W<0> {
        SPIEN_W::new(self)
    }
    #[doc = "Bit 1 - SPI interrupt enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn ien(&mut self) -> IEN_W<1> {
        IEN_W::new(self)
    }
    #[doc = "Bit 2 - Master mode bit"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<2> {
        MODE_W::new(self)
    }
    #[doc = "Bit 3 - Transmit enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn txen(&mut self) -> TXEN_W<3> {
        TXEN_W::new(self)
    }
    #[doc = "Bit 4 - Receive enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn rxen(&mut self) -> RXEN_W<4> {
        RXEN_W::new(self)
    }
    #[doc = "Bits 5:6 - RX FIFO trigger level bit"]
    #[inline(always)]
    #[must_use]
    pub fn rxtlf(&mut self) -> RXTLF_W<5> {
        RXTLF_W::new(self)
    }
    #[doc = "Bits 7:8 - TX FIFO trigger level bit"]
    #[inline(always)]
    #[must_use]
    pub fn txtlf(&mut self) -> TXTLF_W<7> {
        TXTLF_W::new(self)
    }
    #[doc = "Bit 9 - DMA access mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn dmaen(&mut self) -> DMAEN_W<9> {
        DMAEN_W::new(self)
    }
    #[doc = "Bit 10 - NSS select signal that from software and hardware"]
    #[inline(always)]
    #[must_use]
    pub fn nss(&mut self) -> NSS_W<10> {
        NSS_W::new(self)
    }
    #[doc = "Bit 11 - Valid byte or double-word data select signal"]
    #[inline(always)]
    #[must_use]
    pub fn dwsel(&mut self) -> DWSEL_W<11> {
        DWSEL_W::new(self)
    }
    #[doc = "Bit 12 - NSS selection signal is automatically flipped"]
    #[inline(always)]
    #[must_use]
    pub fn nsstog(&mut self) -> NSSTOG_W<12> {
        NSSTOG_W::new(self)
    }
    #[doc = "Bits 13:17 - Bus mapping transformation."]
    #[inline(always)]
    #[must_use]
    pub fn pad_sel(&mut self) -> PAD_SEL_W<13> {
        PAD_SEL_W::new(self)
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
#[doc = "`reset()` method sets GCR to value 0x04"]
impl crate::Resettable for GCR_SPEC {
    const RESET_VALUE: Self::Ux = 0x04;
}
