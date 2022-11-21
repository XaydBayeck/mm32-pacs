#[doc = "Register `TXDATA7_P` reader"]
pub struct R(crate::R<TXDATA7_P_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TXDATA7_P_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TXDATA7_P_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TXDATA7_P_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TXDATA7_P` writer"]
pub struct W(crate::W<TXDATA7_P_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TXDATA7_P_SPEC>;
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
impl From<crate::W<TXDATA7_P_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TXDATA7_P_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATA7` reader - DATA7"]
pub type DATA7_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DATA7` writer - DATA7"]
pub type DATA7_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TXDATA7_P_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - DATA7"]
    #[inline(always)]
    pub fn data7(&self) -> DATA7_R {
        DATA7_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - DATA7"]
    #[inline(always)]
    #[must_use]
    pub fn data7(&mut self) -> DATA7_W<0> {
        DATA7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Peli TX Data register 7\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txdata7_p](index.html) module"]
pub struct TXDATA7_P_SPEC;
impl crate::RegisterSpec for TXDATA7_P_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [txdata7_p::R](R) reader structure"]
impl crate::Readable for TXDATA7_P_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [txdata7_p::W](W) writer structure"]
impl crate::Writable for TXDATA7_P_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TXDATA7_P to value 0"]
impl crate::Resettable for TXDATA7_P_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
