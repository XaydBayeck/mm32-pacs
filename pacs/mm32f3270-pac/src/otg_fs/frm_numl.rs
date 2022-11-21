#[doc = "Register `FRM_NUML` reader"]
pub struct R(crate::R<FRM_NUML_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FRM_NUML_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FRM_NUML_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FRM_NUML_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `FRM_7_0` reader - These bits represent the lower 8 bits of 11 bit frames"]
pub type FRM_7_0_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - These bits represent the lower 8 bits of 11 bit frames"]
    #[inline(always)]
    pub fn frm_7_0(&self) -> FRM_7_0_R {
        FRM_7_0_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Frame number register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [frm_numl](index.html) module"]
pub struct FRM_NUML_SPEC;
impl crate::RegisterSpec for FRM_NUML_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [frm_numl::R](R) reader structure"]
impl crate::Readable for FRM_NUML_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FRM_NUML to value 0"]
impl crate::Resettable for FRM_NUML_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
