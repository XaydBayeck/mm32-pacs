#[doc = "Register `HTLR` reader"]
pub struct R(crate::R<HTLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HTLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HTLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HTLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HTLR` writer"]
pub struct W(crate::W<HTLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HTLR_SPEC>;
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
impl From<crate::W<HTLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HTLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HTABL` reader - Hash Table Low"]
pub type HTABL_R = crate::FieldReader<u32, u32>;
#[doc = "Field `HTABL` writer - Hash Table Low"]
pub type HTABL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HTLR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Hash Table Low"]
    #[inline(always)]
    pub fn htabl(&self) -> HTABL_R {
        HTABL_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Hash Table Low"]
    #[inline(always)]
    #[must_use]
    pub fn htabl(&mut self) -> HTABL_W<0> {
        HTABL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ethernet MAC hash table low register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [htlr](index.html) module"]
pub struct HTLR_SPEC;
impl crate::RegisterSpec for HTLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [htlr::R](R) reader structure"]
impl crate::Readable for HTLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [htlr::W](W) writer structure"]
impl crate::Writable for HTLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HTLR to value 0"]
impl crate::Resettable for HTLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
