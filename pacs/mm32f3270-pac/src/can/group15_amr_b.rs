#[doc = "Register `GROUP15_AMR_B` reader"]
pub struct R(crate::R<GROUP15_AMR_B_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GROUP15_AMR_B_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GROUP15_AMR_B_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GROUP15_AMR_B_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GROUP15_AMR_B` writer"]
pub struct W(crate::W<GROUP15_AMR_B_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GROUP15_AMR_B_SPEC>;
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
impl From<crate::W<GROUP15_AMR_B_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GROUP15_AMR_B_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AM` reader - Acceptance mask"]
pub type AM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AM` writer - Acceptance mask"]
pub type AM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GROUP15_AMR_B_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Acceptance mask"]
    #[inline(always)]
    pub fn am(&self) -> AM_R {
        AM_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Acceptance mask"]
    #[inline(always)]
    #[must_use]
    pub fn am(&mut self) -> AM_W<0> {
        AM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Basic Acceptance Mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [group15_amr_b](index.html) module"]
pub struct GROUP15_AMR_B_SPEC;
impl crate::RegisterSpec for GROUP15_AMR_B_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [group15_amr_b::R](R) reader structure"]
impl crate::Readable for GROUP15_AMR_B_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [group15_amr_b::W](W) writer structure"]
impl crate::Writable for GROUP15_AMR_B_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GROUP15_AMR_B to value 0"]
impl crate::Resettable for GROUP15_AMR_B_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
