#[doc = "Register `CARDSEL` reader"]
pub struct R(crate::R<CARDSEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CARDSEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CARDSEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CARDSEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CARDSEL` writer"]
pub struct W(crate::W<CARDSEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CARDSEL_SPEC>;
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
impl From<crate::W<CARDSEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CARDSEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TSCALE` reader - SD/MMC/SDIO Time scale base(1Mhz) coefficient"]
pub type TSCALE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TSCALE` writer - SD/MMC/SDIO Time scale base(1Mhz) coefficient"]
pub type TSCALE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CARDSEL_SPEC, u8, u8, 6, O>;
#[doc = "Field `PCLKEN` reader - SD/MMC/SDIO Time scale base(1Mhz) coefficient"]
pub type PCLKEN_R = crate::BitReader<bool>;
#[doc = "Field `PCLKEN` writer - SD/MMC/SDIO Time scale base(1Mhz) coefficient"]
pub type PCLKEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CARDSEL_SPEC, bool, O>;
#[doc = "Field `CTREN` reader - SD/MMC/SDIO controller enable"]
pub type CTREN_R = crate::BitReader<bool>;
#[doc = "Field `CTREN` writer - SD/MMC/SDIO controller enable"]
pub type CTREN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CARDSEL_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:5 - SD/MMC/SDIO Time scale base(1Mhz) coefficient"]
    #[inline(always)]
    pub fn tscale(&self) -> TSCALE_R {
        TSCALE_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 6 - SD/MMC/SDIO Time scale base(1Mhz) coefficient"]
    #[inline(always)]
    pub fn pclken(&self) -> PCLKEN_R {
        PCLKEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - SD/MMC/SDIO controller enable"]
    #[inline(always)]
    pub fn ctren(&self) -> CTREN_R {
        CTREN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - SD/MMC/SDIO Time scale base(1Mhz) coefficient"]
    #[inline(always)]
    #[must_use]
    pub fn tscale(&mut self) -> TSCALE_W<0> {
        TSCALE_W::new(self)
    }
    #[doc = "Bit 6 - SD/MMC/SDIO Time scale base(1Mhz) coefficient"]
    #[inline(always)]
    #[must_use]
    pub fn pclken(&mut self) -> PCLKEN_W<6> {
        PCLKEN_W::new(self)
    }
    #[doc = "Bit 7 - SD/MMC/SDIO controller enable"]
    #[inline(always)]
    #[must_use]
    pub fn ctren(&mut self) -> CTREN_W<7> {
        CTREN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "card sell\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cardsel](index.html) module"]
pub struct CARDSEL_SPEC;
impl crate::RegisterSpec for CARDSEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cardsel::R](R) reader structure"]
impl crate::Readable for CARDSEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cardsel::W](W) writer structure"]
impl crate::Writable for CARDSEL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CARDSEL to value 0x40"]
impl crate::Resettable for CARDSEL_SPEC {
    const RESET_VALUE: Self::Ux = 0x40;
}
