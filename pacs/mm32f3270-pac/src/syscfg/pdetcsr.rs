#[doc = "Register `PDETCSR` reader"]
pub struct R(crate::R<PDETCSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDETCSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PDETCSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PDETCSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PDETCSR` writer"]
pub struct W(crate::W<PDETCSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDETCSR_SPEC>;
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
impl From<crate::W<PDETCSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PDETCSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PVDE` reader - PVD enable"]
pub type PVDE_R = crate::BitReader<bool>;
#[doc = "Field `PVDE` writer - PVD enable"]
pub type PVDE_W<'a, const O: u8> = crate::BitWriter<'a, u16, PDETCSR_SPEC, bool, O>;
#[doc = "Field `PLS` reader - PVD threshold selection"]
pub type PLS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PLS` writer - PVD threshold selection"]
pub type PLS_W<'a, const O: u8> = crate::FieldWriter<'a, u16, PDETCSR_SPEC, u8, u8, 4, O>;
#[doc = "Field `PVDO` reader - PVD output status"]
pub type PVDO_R = crate::BitReader<bool>;
#[doc = "Field `PVDO` writer - PVD output status"]
pub type PVDO_W<'a, const O: u8> = crate::BitWriter<'a, u16, PDETCSR_SPEC, bool, O>;
#[doc = "Field `VDTO` reader - VDT output status"]
pub type VDTO_R = crate::BitReader<bool>;
#[doc = "Field `VDTO` writer - VDT output status"]
pub type VDTO_W<'a, const O: u8> = crate::BitWriter<'a, u16, PDETCSR_SPEC, bool, O>;
#[doc = "Field `VDTE` reader - VDT enable"]
pub type VDTE_R = crate::BitReader<bool>;
#[doc = "Field `VDTE` writer - VDT enable"]
pub type VDTE_W<'a, const O: u8> = crate::BitWriter<'a, u16, PDETCSR_SPEC, bool, O>;
#[doc = "Field `VDTLS` reader - VDT detection threshold selection"]
pub type VDTLS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VDTLS` writer - VDT detection threshold selection"]
pub type VDTLS_W<'a, const O: u8> = crate::FieldWriter<'a, u16, PDETCSR_SPEC, u8, u8, 2, O>;
#[doc = "Field `VBAT_DIV3` reader - ADC detection VBat_DIV3 partial pressure value enable"]
pub type VBAT_DIV3_R = crate::BitReader<bool>;
#[doc = "Field `VBAT_DIV3` writer - ADC detection VBat_DIV3 partial pressure value enable"]
pub type VBAT_DIV3_W<'a, const O: u8> = crate::BitWriter<'a, u16, PDETCSR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - PVD enable"]
    #[inline(always)]
    pub fn pvde(&self) -> PVDE_R {
        PVDE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:4 - PVD threshold selection"]
    #[inline(always)]
    pub fn pls(&self) -> PLS_R {
        PLS_R::new(((self.bits >> 1) & 0x0f) as u8)
    }
    #[doc = "Bit 5 - PVD output status"]
    #[inline(always)]
    pub fn pvdo(&self) -> PVDO_R {
        PVDO_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - VDT output status"]
    #[inline(always)]
    pub fn vdto(&self) -> VDTO_R {
        VDTO_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - VDT enable"]
    #[inline(always)]
    pub fn vdte(&self) -> VDTE_R {
        VDTE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:10 - VDT detection threshold selection"]
    #[inline(always)]
    pub fn vdtls(&self) -> VDTLS_R {
        VDTLS_R::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bit 11 - ADC detection VBat_DIV3 partial pressure value enable"]
    #[inline(always)]
    pub fn vbat_div3(&self) -> VBAT_DIV3_R {
        VBAT_DIV3_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PVD enable"]
    #[inline(always)]
    #[must_use]
    pub fn pvde(&mut self) -> PVDE_W<0> {
        PVDE_W::new(self)
    }
    #[doc = "Bits 1:4 - PVD threshold selection"]
    #[inline(always)]
    #[must_use]
    pub fn pls(&mut self) -> PLS_W<1> {
        PLS_W::new(self)
    }
    #[doc = "Bit 5 - PVD output status"]
    #[inline(always)]
    #[must_use]
    pub fn pvdo(&mut self) -> PVDO_W<5> {
        PVDO_W::new(self)
    }
    #[doc = "Bit 6 - VDT output status"]
    #[inline(always)]
    #[must_use]
    pub fn vdto(&mut self) -> VDTO_W<6> {
        VDTO_W::new(self)
    }
    #[doc = "Bit 8 - VDT enable"]
    #[inline(always)]
    #[must_use]
    pub fn vdte(&mut self) -> VDTE_W<8> {
        VDTE_W::new(self)
    }
    #[doc = "Bits 9:10 - VDT detection threshold selection"]
    #[inline(always)]
    #[must_use]
    pub fn vdtls(&mut self) -> VDTLS_W<9> {
        VDTLS_W::new(self)
    }
    #[doc = "Bit 11 - ADC detection VBat_DIV3 partial pressure value enable"]
    #[inline(always)]
    #[must_use]
    pub fn vbat_div3(&mut self) -> VBAT_DIV3_W<11> {
        VBAT_DIV3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Power detection configuration status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdetcsr](index.html) module"]
pub struct PDETCSR_SPEC;
impl crate::RegisterSpec for PDETCSR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [pdetcsr::R](R) reader structure"]
impl crate::Readable for PDETCSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pdetcsr::W](W) writer structure"]
impl crate::Writable for PDETCSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PDETCSR to value 0"]
impl crate::Resettable for PDETCSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
