#[doc = "Register `AFM1` reader"]
pub struct R(crate::R<AFM1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AFM1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AFM1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AFM1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AFM1` writer"]
pub struct W(crate::W<AFM1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AFM1_SPEC>;
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
impl From<crate::W<AFM1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AFM1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AFM_15_8` reader - Acceptance filter mode"]
pub type AFM_15_8_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AFM_15_8` writer - Acceptance filter mode"]
pub type AFM_15_8_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AFM1_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Acceptance filter mode"]
    #[inline(always)]
    pub fn afm_15_8(&self) -> AFM_15_8_R {
        AFM_15_8_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Acceptance filter mode"]
    #[inline(always)]
    #[must_use]
    pub fn afm_15_8(&mut self) -> AFM_15_8_W<0> {
        AFM_15_8_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Filter Mode register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [afm1](index.html) module"]
pub struct AFM1_SPEC;
impl crate::RegisterSpec for AFM1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [afm1::R](R) reader structure"]
impl crate::Readable for AFM1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [afm1::W](W) writer structure"]
impl crate::Writable for AFM1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AFM1 to value 0"]
impl crate::Resettable for AFM1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
