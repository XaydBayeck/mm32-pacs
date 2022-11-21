#[doc = "Register `HSEDLY` reader"]
pub struct R(crate::R<HSEDLY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HSEDLY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HSEDLY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HSEDLY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HSEDLY` writer"]
pub struct W(crate::W<HSEDLY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HSEDLY_SPEC>;
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
impl From<crate::W<HSEDLY_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HSEDLY_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HSE_EQU_CNT` reader - *D0"]
pub type HSE_EQU_CNT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `HSE_EQU_CNT` writer - *D0"]
pub type HSE_EQU_CNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HSEDLY_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - *D0"]
    #[inline(always)]
    pub fn hse_equ_cnt(&self) -> HSE_EQU_CNT_R {
        HSE_EQU_CNT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - *D0"]
    #[inline(always)]
    #[must_use]
    pub fn hse_equ_cnt(&mut self) -> HSE_EQU_CNT_W<0> {
        HSE_EQU_CNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HSE Delay Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hsedly](index.html) module"]
pub struct HSEDLY_SPEC;
impl crate::RegisterSpec for HSEDLY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hsedly::R](R) reader structure"]
impl crate::Readable for HSEDLY_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hsedly::W](W) writer structure"]
impl crate::Writable for HSEDLY_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HSEDLY to value 0xbb80"]
impl crate::Resettable for HSEDLY_SPEC {
    const RESET_VALUE: Self::Ux = 0xbb80;
}
