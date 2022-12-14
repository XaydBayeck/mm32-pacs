#[doc = "Register `DMABSR` reader"]
pub struct R(crate::R<DMABSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMABSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMABSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMABSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMABSR` writer"]
pub struct W(crate::W<DMABSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMABSR_SPEC>;
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
impl From<crate::W<DMABSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMABSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SR` reader - Software reset"]
pub type SR_R = crate::BitReader<bool>;
#[doc = "Field `SR` writer - Software reset"]
pub type SR_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMABSR_SPEC, bool, O>;
#[doc = "Field `DMAA` reader - DMA Arbitration"]
pub type DMAA_R = crate::BitReader<bool>;
#[doc = "Field `DMAA` writer - DMA Arbitration"]
pub type DMAA_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMABSR_SPEC, bool, O>;
#[doc = "Field `DSL` reader - Descriptor skip length"]
pub type DSL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DSL` writer - Descriptor skip length"]
pub type DSL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DMABSR_SPEC, u8, u8, 5, O>;
#[doc = "Field `PBL` reader - Programmable burst length"]
pub type PBL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PBL` writer - Programmable burst length"]
pub type PBL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DMABSR_SPEC, u8, u8, 6, O>;
#[doc = "Field `FTPR` reader - Rx Tx priority ratio"]
pub type FTPR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FTPR` writer - Rx Tx priority ratio"]
pub type FTPR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DMABSR_SPEC, u8, u8, 2, O>;
#[doc = "Field `FBST` reader - Fixed burst"]
pub type FBST_R = crate::BitReader<bool>;
#[doc = "Field `FBST` writer - Fixed burst"]
pub type FBST_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMABSR_SPEC, bool, O>;
#[doc = "Field `ALL` reader - *D25"]
pub type ALL_R = crate::BitReader<bool>;
#[doc = "Field `ALL` writer - *D25"]
pub type ALL_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMABSR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Software reset"]
    #[inline(always)]
    pub fn sr(&self) -> SR_R {
        SR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMA Arbitration"]
    #[inline(always)]
    pub fn dmaa(&self) -> DMAA_R {
        DMAA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:6 - Descriptor skip length"]
    #[inline(always)]
    pub fn dsl(&self) -> DSL_R {
        DSL_R::new(((self.bits >> 2) & 0x1f) as u8)
    }
    #[doc = "Bits 8:13 - Programmable burst length"]
    #[inline(always)]
    pub fn pbl(&self) -> PBL_R {
        PBL_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 14:15 - Rx Tx priority ratio"]
    #[inline(always)]
    pub fn ftpr(&self) -> FTPR_R {
        FTPR_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bit 16 - Fixed burst"]
    #[inline(always)]
    pub fn fbst(&self) -> FBST_R {
        FBST_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 25 - *D25"]
    #[inline(always)]
    pub fn all(&self) -> ALL_R {
        ALL_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Software reset"]
    #[inline(always)]
    #[must_use]
    pub fn sr(&mut self) -> SR_W<0> {
        SR_W::new(self)
    }
    #[doc = "Bit 1 - DMA Arbitration"]
    #[inline(always)]
    #[must_use]
    pub fn dmaa(&mut self) -> DMAA_W<1> {
        DMAA_W::new(self)
    }
    #[doc = "Bits 2:6 - Descriptor skip length"]
    #[inline(always)]
    #[must_use]
    pub fn dsl(&mut self) -> DSL_W<2> {
        DSL_W::new(self)
    }
    #[doc = "Bits 8:13 - Programmable burst length"]
    #[inline(always)]
    #[must_use]
    pub fn pbl(&mut self) -> PBL_W<8> {
        PBL_W::new(self)
    }
    #[doc = "Bits 14:15 - Rx Tx priority ratio"]
    #[inline(always)]
    #[must_use]
    pub fn ftpr(&mut self) -> FTPR_W<14> {
        FTPR_W::new(self)
    }
    #[doc = "Bit 16 - Fixed burst"]
    #[inline(always)]
    #[must_use]
    pub fn fbst(&mut self) -> FBST_W<16> {
        FBST_W::new(self)
    }
    #[doc = "Bit 25 - *D25"]
    #[inline(always)]
    #[must_use]
    pub fn all(&mut self) -> ALL_W<25> {
        ALL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ethernet DMA bus mode register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmabsr](index.html) module"]
pub struct DMABSR_SPEC;
impl crate::RegisterSpec for DMABSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmabsr::R](R) reader structure"]
impl crate::Readable for DMABSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmabsr::W](W) writer structure"]
impl crate::Writable for DMABSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMABSR to value 0"]
impl crate::Resettable for DMABSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
