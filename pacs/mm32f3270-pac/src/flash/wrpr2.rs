#[doc = "Register `WRPR2` reader"]
pub struct R(crate::R<WRPR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WRPR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WRPR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WRPR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `WRP` reader - Write protect"]
pub type WRP_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Write protect"]
    #[inline(always)]
    pub fn wrp(&self) -> WRP_R {
        WRP_R::new(self.bits)
    }
}
#[doc = "Write protect register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wrpr2](index.html) module"]
pub struct WRPR2_SPEC;
impl crate::RegisterSpec for WRPR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wrpr2::R](R) reader structure"]
impl crate::Readable for WRPR2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets WRPR2 to value 0xffff_ffff"]
impl crate::Resettable for WRPR2_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
