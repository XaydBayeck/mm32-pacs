#[doc = "Register `TXDATA9_P` reader"]
pub struct R(crate::R<TXDATA9_P_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TXDATA9_P_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TXDATA9_P_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TXDATA9_P_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TXDATA9_P` writer"]
pub struct W(crate::W<TXDATA9_P_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TXDATA9_P_SPEC>;
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
impl From<crate::W<TXDATA9_P_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TXDATA9_P_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATA9` reader - DATA9"]
pub type DATA9_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DATA9` writer - DATA9"]
pub type DATA9_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TXDATA9_P_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - DATA9"]
    #[inline(always)]
    pub fn data9(&self) -> DATA9_R {
        DATA9_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - DATA9"]
    #[inline(always)]
    #[must_use]
    pub fn data9(&mut self) -> DATA9_W<0> {
        DATA9_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Peli TX Data register 9\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txdata9_p](index.html) module"]
pub struct TXDATA9_P_SPEC;
impl crate::RegisterSpec for TXDATA9_P_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [txdata9_p::R](R) reader structure"]
impl crate::Readable for TXDATA9_P_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [txdata9_p::W](W) writer structure"]
impl crate::Writable for TXDATA9_P_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TXDATA9_P to value 0"]
impl crate::Resettable for TXDATA9_P_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
