#[doc = "Register `CPAR5` reader"]
pub struct R(crate::R<CPAR5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CPAR5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CPAR5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CPAR5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CPAR5` writer"]
pub struct W(crate::W<CPAR5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CPAR5_SPEC>;
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
impl From<crate::W<CPAR5_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CPAR5_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PA` reader - Peripheral address"]
pub type PA_R = crate::FieldReader<u32, u32>;
#[doc = "Field `PA` writer - Peripheral address"]
pub type PA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CPAR5_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Peripheral address"]
    #[inline(always)]
    pub fn pa(&self) -> PA_R {
        PA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Peripheral address"]
    #[inline(always)]
    #[must_use]
    pub fn pa(&mut self) -> PA_W<0> {
        PA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel 5 peripheral address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpar5](index.html) module"]
pub struct CPAR5_SPEC;
impl crate::RegisterSpec for CPAR5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cpar5::R](R) reader structure"]
impl crate::Readable for CPAR5_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cpar5::W](W) writer structure"]
impl crate::Writable for CPAR5_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CPAR5 to value 0"]
impl crate::Resettable for CPAR5_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
