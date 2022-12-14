#[doc = "Register `PLLDLY` reader"]
pub struct R(crate::R<PLLDLY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PLLDLY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PLLDLY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PLLDLY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PLLDLY` writer"]
pub struct W(crate::W<PLLDLY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PLLDLY_SPEC>;
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
impl From<crate::W<PLLDLY_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PLLDLY_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PLL_EQU_CNT` reader - *D0"]
pub type PLL_EQU_CNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PLL_EQU_CNT` writer - *D0"]
pub type PLL_EQU_CNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PLLDLY_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - *D0"]
    #[inline(always)]
    pub fn pll_equ_cnt(&self) -> PLL_EQU_CNT_R {
        PLL_EQU_CNT_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - *D0"]
    #[inline(always)]
    #[must_use]
    pub fn pll_equ_cnt(&mut self) -> PLL_EQU_CNT_W<0> {
        PLL_EQU_CNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PLL Delay Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [plldly](index.html) module"]
pub struct PLLDLY_SPEC;
impl crate::RegisterSpec for PLLDLY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [plldly::R](R) reader structure"]
impl crate::Readable for PLLDLY_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [plldly::W](W) writer structure"]
impl crate::Writable for PLLDLY_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PLLDLY to value 0x03eb"]
impl crate::Resettable for PLLDLY_SPEC {
    const RESET_VALUE: Self::Ux = 0x03eb;
}
