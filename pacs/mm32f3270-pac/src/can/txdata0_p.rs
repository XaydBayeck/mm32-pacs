#[doc = "Register `TXDATA0_P` reader"]
pub struct R(crate::R<TXDATA0_P_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TXDATA0_P_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TXDATA0_P_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TXDATA0_P_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TXDATA0_P` writer"]
pub struct W(crate::W<TXDATA0_P_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TXDATA0_P_SPEC>;
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
impl From<crate::W<TXDATA0_P_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TXDATA0_P_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATA0` reader - DATA0"]
pub type DATA0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DATA0` writer - DATA0"]
pub type DATA0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TXDATA0_P_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - DATA0"]
    #[inline(always)]
    pub fn data0(&self) -> DATA0_R {
        DATA0_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - DATA0"]
    #[inline(always)]
    #[must_use]
    pub fn data0(&mut self) -> DATA0_W<0> {
        DATA0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Peli TX Data register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txdata0_p](index.html) module"]
pub struct TXDATA0_P_SPEC;
impl crate::RegisterSpec for TXDATA0_P_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [txdata0_p::R](R) reader structure"]
impl crate::Readable for TXDATA0_P_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [txdata0_p::W](W) writer structure"]
impl crate::Writable for TXDATA0_P_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TXDATA0_P to value 0"]
impl crate::Resettable for TXDATA0_P_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
