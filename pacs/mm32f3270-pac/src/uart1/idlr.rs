#[doc = "Register `IDLR` reader"]
pub struct R(crate::R<IDLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IDLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IDLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IDLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IDLR` writer"]
pub struct W(crate::W<IDLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IDLR_SPEC>;
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
impl From<crate::W<IDLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IDLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IDLR` reader - Idle data length register"]
pub type IDLR_R = crate::FieldReader<u16, u16>;
#[doc = "Field `IDLR` writer - Idle data length register"]
pub type IDLR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IDLR_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Idle data length register"]
    #[inline(always)]
    pub fn idlr(&self) -> IDLR_R {
        IDLR_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Idle data length register"]
    #[inline(always)]
    #[must_use]
    pub fn idlr(&mut self) -> IDLR_W<0> {
        IDLR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Data length register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [idlr](index.html) module"]
pub struct IDLR_SPEC;
impl crate::RegisterSpec for IDLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [idlr::R](R) reader structure"]
impl crate::Readable for IDLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [idlr::W](W) writer structure"]
impl crate::Writable for IDLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IDLR to value 0x0c"]
impl crate::Resettable for IDLR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0c;
}
