#[doc = "Register `AFM2` reader"]
pub struct R(crate::R<AFM2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AFM2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AFM2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AFM2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AFM2` writer"]
pub struct W(crate::W<AFM2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AFM2_SPEC>;
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
impl From<crate::W<AFM2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AFM2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AFM_19_16` reader - Acceptance filter mode"]
pub type AFM_19_16_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AFM_19_16` writer - Acceptance filter mode"]
pub type AFM_19_16_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AFM2_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3 - Acceptance filter mode"]
    #[inline(always)]
    pub fn afm_19_16(&self) -> AFM_19_16_R {
        AFM_19_16_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Acceptance filter mode"]
    #[inline(always)]
    #[must_use]
    pub fn afm_19_16(&mut self) -> AFM_19_16_W<0> {
        AFM_19_16_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Filter Mode register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [afm2](index.html) module"]
pub struct AFM2_SPEC;
impl crate::RegisterSpec for AFM2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [afm2::R](R) reader structure"]
impl crate::Readable for AFM2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [afm2::W](W) writer structure"]
impl crate::Writable for AFM2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AFM2 to value 0"]
impl crate::Resettable for AFM2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
