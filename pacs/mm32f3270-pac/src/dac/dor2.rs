#[doc = "Register `DOR2` reader"]
pub struct R(crate::R<DOR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DOR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DOR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DOR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DOR2` writer"]
pub struct W(crate::W<DOR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DOR2_SPEC>;
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
impl From<crate::W<DOR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DOR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DACC2DOR` reader - DAC channel2 data output"]
pub type DACC2DOR_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DACC2DOR` writer - DAC channel2 data output"]
pub type DACC2DOR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DOR2_SPEC, u16, u16, 12, O>;
impl R {
    #[doc = "Bits 0:11 - DAC channel2 data output"]
    #[inline(always)]
    pub fn dacc2dor(&self) -> DACC2DOR_R {
        DACC2DOR_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - DAC channel2 data output"]
    #[inline(always)]
    #[must_use]
    pub fn dacc2dor(&mut self) -> DACC2DOR_W<0> {
        DACC2DOR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel2 data output register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dor2](index.html) module"]
pub struct DOR2_SPEC;
impl crate::RegisterSpec for DOR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dor2::R](R) reader structure"]
impl crate::Readable for DOR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dor2::W](W) writer structure"]
impl crate::Writable for DOR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DOR2 to value 0"]
impl crate::Resettable for DOR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
