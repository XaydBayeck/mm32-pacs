#[doc = "Register `ICSCR` reader"]
pub struct R(crate::R<ICSCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ICSCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ICSCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ICSCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ICSCR` writer"]
pub struct W(crate::W<ICSCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ICSCR_SPEC>;
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
impl From<crate::W<ICSCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ICSCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIME_CRS_SEL` reader - CRS module flag"]
pub type TIME_CRS_SEL_R = crate::BitReader<bool>;
#[doc = "Field `TIME_CRS_SEL` writer - CRS module flag"]
pub type TIME_CRS_SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICSCR_SPEC, bool, O>;
#[doc = "Field `HSI_CAL_SEL` reader - Initial value of internal high-speed clock calibration"]
pub type HSI_CAL_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HSI_CAL_SEL` writer - Initial value of internal high-speed clock calibration"]
pub type HSI_CAL_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ICSCR_SPEC, u8, u8, 5, O>;
#[doc = "Field `HSI_CAL_SFT` reader - Internal high-speed clock calibration"]
pub type HSI_CAL_SFT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `HSI_CAL_SFT` writer - Internal high-speed clock calibration"]
pub type HSI_CAL_SFT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ICSCR_SPEC, u16, u16, 10, O>;
impl R {
    #[doc = "Bit 0 - CRS module flag"]
    #[inline(always)]
    pub fn time_crs_sel(&self) -> TIME_CRS_SEL_R {
        TIME_CRS_SEL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 11:15 - Initial value of internal high-speed clock calibration"]
    #[inline(always)]
    pub fn hsi_cal_sel(&self) -> HSI_CAL_SEL_R {
        HSI_CAL_SEL_R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bits 16:25 - Internal high-speed clock calibration"]
    #[inline(always)]
    pub fn hsi_cal_sft(&self) -> HSI_CAL_SFT_R {
        HSI_CAL_SFT_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - CRS module flag"]
    #[inline(always)]
    #[must_use]
    pub fn time_crs_sel(&mut self) -> TIME_CRS_SEL_W<0> {
        TIME_CRS_SEL_W::new(self)
    }
    #[doc = "Bits 11:15 - Initial value of internal high-speed clock calibration"]
    #[inline(always)]
    #[must_use]
    pub fn hsi_cal_sel(&mut self) -> HSI_CAL_SEL_W<11> {
        HSI_CAL_SEL_W::new(self)
    }
    #[doc = "Bits 16:25 - Internal high-speed clock calibration"]
    #[inline(always)]
    #[must_use]
    pub fn hsi_cal_sft(&mut self) -> HSI_CAL_SFT_W<16> {
        HSI_CAL_SFT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal clock sourcec Configure Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icscr](index.html) module"]
pub struct ICSCR_SPEC;
impl crate::RegisterSpec for ICSCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [icscr::R](R) reader structure"]
impl crate::Readable for ICSCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [icscr::W](W) writer structure"]
impl crate::Writable for ICSCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ICSCR to value 0x0200_0000"]
impl crate::Resettable for ICSCR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0200_0000;
}
