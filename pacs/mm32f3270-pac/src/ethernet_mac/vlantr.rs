#[doc = "Register `VLANTR` reader"]
pub struct R(crate::R<VLANTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VLANTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VLANTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VLANTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `VLANTR` writer"]
pub struct W(crate::W<VLANTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VLANTR_SPEC>;
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
impl From<crate::W<VLANTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<VLANTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VLANTAG` reader - *D0"]
pub type VLANTAG_R = crate::FieldReader<u16, u16>;
#[doc = "Field `VLANTAG` writer - *D0"]
pub type VLANTAG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, VLANTR_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - *D0"]
    #[inline(always)]
    pub fn vlantag(&self) -> VLANTAG_R {
        VLANTAG_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - *D0"]
    #[inline(always)]
    #[must_use]
    pub fn vlantag(&mut self) -> VLANTAG_W<0> {
        VLANTAG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ethernet MAC VLAN tag register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vlantr](index.html) module"]
pub struct VLANTR_SPEC;
impl crate::RegisterSpec for VLANTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [vlantr::R](R) reader structure"]
impl crate::Readable for VLANTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [vlantr::W](W) writer structure"]
impl crate::Writable for VLANTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets VLANTR to value 0"]
impl crate::Resettable for VLANTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
