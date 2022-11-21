#[doc = "Register `FSLR` reader"]
pub struct R(crate::R<FSLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FSLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FSLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FSLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FSLR` writer"]
pub struct W(crate::W<FSLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FSLR_SPEC>;
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
impl From<crate::W<FSLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FSLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CNT` reader - This register sets the SCL clock low period count for standard speed"]
pub type CNT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CNT` writer - This register sets the SCL clock low period count for standard speed"]
pub type CNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FSLR_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - This register sets the SCL clock low period count for standard speed"]
    #[inline(always)]
    pub fn cnt(&self) -> CNT_R {
        CNT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - This register sets the SCL clock low period count for standard speed"]
    #[inline(always)]
    #[must_use]
    pub fn cnt(&mut self) -> CNT_W<0> {
        CNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SCL Low Period Count for Fast Speed Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fslr](index.html) module"]
pub struct FSLR_SPEC;
impl crate::RegisterSpec for FSLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fslr::R](R) reader structure"]
impl crate::Readable for FSLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fslr::W](W) writer structure"]
impl crate::Writable for FSLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FSLR to value 0x82"]
impl crate::Resettable for FSLR_SPEC {
    const RESET_VALUE: Self::Ux = 0x82;
}
