#[doc = "Register `CSHR` reader"]
pub struct R(crate::R<CSHR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSHR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSHR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSHR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CSHR` writer"]
pub struct W(crate::W<CSHR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSHR_SPEC>;
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
impl From<crate::W<CSHR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSHR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CSHR` reader - hit statistics"]
pub type CSHR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CSHR` writer - hit statistics"]
pub type CSHR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CSHR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - hit statistics"]
    #[inline(always)]
    pub fn cshr(&self) -> CSHR_R {
        CSHR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - hit statistics"]
    #[inline(always)]
    #[must_use]
    pub fn cshr(&mut self) -> CSHR_W<0> {
        CSHR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Cache statistics hit register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cshr](index.html) module"]
pub struct CSHR_SPEC;
impl crate::RegisterSpec for CSHR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cshr::R](R) reader structure"]
impl crate::Readable for CSHR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cshr::W](W) writer structure"]
impl crate::Writable for CSHR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CSHR to value 0"]
impl crate::Resettable for CSHR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
