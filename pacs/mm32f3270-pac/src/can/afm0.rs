#[doc = "Register `AFM0` reader"]
pub struct R(crate::R<AFM0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AFM0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AFM0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AFM0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AFM0` writer"]
pub struct W(crate::W<AFM0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AFM0_SPEC>;
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
impl From<crate::W<AFM0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AFM0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AFM_7_1` reader - Acceptance filter mode"]
pub type AFM_7_1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AFM_7_1` writer - Acceptance filter mode"]
pub type AFM_7_1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AFM0_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bits 1:7 - Acceptance filter mode"]
    #[inline(always)]
    pub fn afm_7_1(&self) -> AFM_7_1_R {
        AFM_7_1_R::new(((self.bits >> 1) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 1:7 - Acceptance filter mode"]
    #[inline(always)]
    #[must_use]
    pub fn afm_7_1(&mut self) -> AFM_7_1_W<1> {
        AFM_7_1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Filter Mode register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [afm0](index.html) module"]
pub struct AFM0_SPEC;
impl crate::RegisterSpec for AFM0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [afm0::R](R) reader structure"]
impl crate::Readable for AFM0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [afm0::W](W) writer structure"]
impl crate::Writable for AFM0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AFM0 to value 0"]
impl crate::Resettable for AFM0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
