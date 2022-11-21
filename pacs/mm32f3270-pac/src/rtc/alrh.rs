#[doc = "Register `ALRH` reader"]
pub struct R(crate::R<ALRH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ALRH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ALRH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ALRH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ALRH` writer"]
pub struct W(crate::W<ALRH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ALRH_SPEC>;
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
impl From<crate::W<ALRH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ALRH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RTC_ALR` reader - RTC alarm high"]
pub type RTC_ALR_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RTC_ALR` writer - RTC alarm high"]
pub type RTC_ALR_W<'a, const O: u8> = crate::FieldWriter<'a, u16, ALRH_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - RTC alarm high"]
    #[inline(always)]
    pub fn rtc_alr(&self) -> RTC_ALR_R {
        RTC_ALR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - RTC alarm high"]
    #[inline(always)]
    #[must_use]
    pub fn rtc_alr(&mut self) -> RTC_ALR_W<0> {
        RTC_ALR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Alarm high register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [alrh](index.html) module"]
pub struct ALRH_SPEC;
impl crate::RegisterSpec for ALRH_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [alrh::R](R) reader structure"]
impl crate::Readable for ALRH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [alrh::W](W) writer structure"]
impl crate::Writable for ALRH_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ALRH to value 0xffff"]
impl crate::Resettable for ALRH_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff;
}
