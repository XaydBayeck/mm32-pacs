#[doc = "Register `RXTLR` reader"]
pub struct R(crate::R<RXTLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXTLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXTLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXTLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RXTLR` writer"]
pub struct W(crate::W<RXTLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RXTLR_SPEC>;
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
impl From<crate::W<RXTLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RXTLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TL` reader - Receive FIFO threshold level"]
pub type TL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TL` writer - Receive FIFO threshold level"]
pub type TL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RXTLR_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Receive FIFO threshold level"]
    #[inline(always)]
    pub fn tl(&self) -> TL_R {
        TL_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Receive FIFO threshold level"]
    #[inline(always)]
    #[must_use]
    pub fn tl(&mut self) -> TL_W<0> {
        TL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Receive FIFO Threshold Level Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxtlr](index.html) module"]
pub struct RXTLR_SPEC;
impl crate::RegisterSpec for RXTLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxtlr::R](R) reader structure"]
impl crate::Readable for RXTLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rxtlr::W](W) writer structure"]
impl crate::Writable for RXTLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RXTLR to value 0"]
impl crate::Resettable for RXTLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
