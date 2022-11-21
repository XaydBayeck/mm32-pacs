#[doc = "Register `FRM_NUMH` reader"]
pub struct R(crate::R<FRM_NUMH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FRM_NUMH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FRM_NUMH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FRM_NUMH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `FRM_10_8` reader - These bits represent the higher 8 bits of 11 bit frames"]
pub type FRM_10_8_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:2 - These bits represent the higher 8 bits of 11 bit frames"]
    #[inline(always)]
    pub fn frm_10_8(&self) -> FRM_10_8_R {
        FRM_10_8_R::new((self.bits & 7) as u8)
    }
}
#[doc = "Frame number register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [frm_numh](index.html) module"]
pub struct FRM_NUMH_SPEC;
impl crate::RegisterSpec for FRM_NUMH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [frm_numh::R](R) reader structure"]
impl crate::Readable for FRM_NUMH_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FRM_NUMH to value 0"]
impl crate::Resettable for FRM_NUMH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
