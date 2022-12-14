#[doc = "Register `TXDR4_B` reader"]
pub struct R(crate::R<TXDR4_B_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TXDR4_B_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TXDR4_B_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TXDR4_B_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TXDR4_B` writer"]
pub struct W(crate::W<TXDR4_B_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TXDR4_B_SPEC>;
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
impl From<crate::W<TXDR4_B_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TXDR4_B_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXDR4` reader - Transmit data register"]
pub type TXDR4_R = crate::FieldReader<u32, u32>;
#[doc = "Field `TXDR4` writer - Transmit data register"]
pub type TXDR4_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TXDR4_B_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Transmit data register"]
    #[inline(always)]
    pub fn txdr4(&self) -> TXDR4_R {
        TXDR4_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Transmit data register"]
    #[inline(always)]
    #[must_use]
    pub fn txdr4(&mut self) -> TXDR4_W<0> {
        TXDR4_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Basic TX Data register 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txdr4_b](index.html) module"]
pub struct TXDR4_B_SPEC;
impl crate::RegisterSpec for TXDR4_B_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [txdr4_b::R](R) reader structure"]
impl crate::Readable for TXDR4_B_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [txdr4_b::W](W) writer structure"]
impl crate::Writable for TXDR4_B_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TXDR4_B to value 0"]
impl crate::Resettable for TXDR4_B_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
