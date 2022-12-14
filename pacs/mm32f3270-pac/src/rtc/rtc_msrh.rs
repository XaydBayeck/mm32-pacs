#[doc = "Register `RTC_MSRH` reader"]
pub struct R(crate::R<RTC_MSRH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTC_MSRH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTC_MSRH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTC_MSRH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTC_MSRH` writer"]
pub struct W(crate::W<RTC_MSRH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTC_MSRH_SPEC>;
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
impl From<crate::W<RTC_MSRH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTC_MSRH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MSR` reader - RTC msec high"]
pub type MSR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MSR` writer - RTC msec high"]
pub type MSR_W<'a, const O: u8> = crate::FieldWriter<'a, u16, RTC_MSRH_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3 - RTC msec high"]
    #[inline(always)]
    pub fn msr(&self) -> MSR_R {
        MSR_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - RTC msec high"]
    #[inline(always)]
    #[must_use]
    pub fn msr(&mut self) -> MSR_W<0> {
        MSR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC millisecond alarm high register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_msrh](index.html) module"]
pub struct RTC_MSRH_SPEC;
impl crate::RegisterSpec for RTC_MSRH_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [rtc_msrh::R](R) reader structure"]
impl crate::Readable for RTC_MSRH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtc_msrh::W](W) writer structure"]
impl crate::Writable for RTC_MSRH_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RTC_MSRH to value 0"]
impl crate::Resettable for RTC_MSRH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
