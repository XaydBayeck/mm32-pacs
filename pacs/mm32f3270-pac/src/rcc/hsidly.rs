#[doc = "Register `HSIDLY` reader"]
pub struct R(crate::R<HSIDLY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HSIDLY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HSIDLY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HSIDLY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HSIDLY` writer"]
pub struct W(crate::W<HSIDLY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HSIDLY_SPEC>;
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
impl From<crate::W<HSIDLY_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HSIDLY_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HSI_EQU_CNT` reader - *D0"]
pub type HSI_EQU_CNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HSI_EQU_CNT` writer - *D0"]
pub type HSI_EQU_CNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HSIDLY_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - *D0"]
    #[inline(always)]
    pub fn hsi_equ_cnt(&self) -> HSI_EQU_CNT_R {
        HSI_EQU_CNT_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - *D0"]
    #[inline(always)]
    #[must_use]
    pub fn hsi_equ_cnt(&mut self) -> HSI_EQU_CNT_W<0> {
        HSI_EQU_CNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HSI Delay Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hsidly](index.html) module"]
pub struct HSIDLY_SPEC;
impl crate::RegisterSpec for HSIDLY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hsidly::R](R) reader structure"]
impl crate::Readable for HSIDLY_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hsidly::W](W) writer structure"]
impl crate::Writable for HSIDLY_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HSIDLY to value 0x1e"]
impl crate::Resettable for HSIDLY_SPEC {
    const RESET_VALUE: Self::Ux = 0x1e;
}
