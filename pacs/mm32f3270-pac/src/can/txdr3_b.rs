#[doc = "Register `TXDR3_B` reader"]
pub struct R(crate::R<TXDR3_B_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TXDR3_B_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TXDR3_B_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TXDR3_B_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TXDR3_B` writer"]
pub struct W(crate::W<TXDR3_B_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TXDR3_B_SPEC>;
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
impl From<crate::W<TXDR3_B_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TXDR3_B_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXDR3` reader - Transmit data register"]
pub type TXDR3_R = crate::FieldReader<u32, u32>;
#[doc = "Field `TXDR3` writer - Transmit data register"]
pub type TXDR3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TXDR3_B_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Transmit data register"]
    #[inline(always)]
    pub fn txdr3(&self) -> TXDR3_R {
        TXDR3_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Transmit data register"]
    #[inline(always)]
    #[must_use]
    pub fn txdr3(&mut self) -> TXDR3_W<0> {
        TXDR3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Basic TX Data register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txdr3_b](index.html) module"]
pub struct TXDR3_B_SPEC;
impl crate::RegisterSpec for TXDR3_B_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [txdr3_b::R](R) reader structure"]
impl crate::Readable for TXDR3_B_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [txdr3_b::W](W) writer structure"]
impl crate::Writable for TXDR3_B_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TXDR3_B to value 0"]
impl crate::Resettable for TXDR3_B_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
