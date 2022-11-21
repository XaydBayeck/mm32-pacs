#[doc = "Register `SOF_THLD` reader"]
pub struct R(crate::R<SOF_THLD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SOF_THLD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SOF_THLD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SOF_THLD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SOF_THLD` writer"]
pub struct W(crate::W<SOF_THLD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SOF_THLD_SPEC>;
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
impl From<crate::W<SOF_THLD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SOF_THLD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CNT` reader - These bits represent the sof count threshold for byte time"]
pub type CNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CNT` writer - These bits represent the sof count threshold for byte time"]
pub type CNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SOF_THLD_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - These bits represent the sof count threshold for byte time"]
    #[inline(always)]
    pub fn cnt(&self) -> CNT_R {
        CNT_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - These bits represent the sof count threshold for byte time"]
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
#[doc = "SOF threshold register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sof_thld](index.html) module"]
pub struct SOF_THLD_SPEC;
impl crate::RegisterSpec for SOF_THLD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sof_thld::R](R) reader structure"]
impl crate::Readable for SOF_THLD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sof_thld::W](W) writer structure"]
impl crate::Writable for SOF_THLD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SOF_THLD to value 0"]
impl crate::Resettable for SOF_THLD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
