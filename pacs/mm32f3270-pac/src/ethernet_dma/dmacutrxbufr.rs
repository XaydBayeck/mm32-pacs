#[doc = "Register `DMACUTRXBUFR` reader"]
pub struct R(crate::R<DMACUTRXBUFR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMACUTRXBUFR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMACUTRXBUFR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMACUTRXBUFR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CUTRXBUF` reader - Host Receive Buffer Address Pointer"]
pub type CUTRXBUF_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Host Receive Buffer Address Pointer"]
    #[inline(always)]
    pub fn cutrxbuf(&self) -> CUTRXBUF_R {
        CUTRXBUF_R::new(self.bits)
    }
}
#[doc = "Ethernet DMA current host receive buffer address register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmacutrxbufr](index.html) module"]
pub struct DMACUTRXBUFR_SPEC;
impl crate::RegisterSpec for DMACUTRXBUFR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmacutrxbufr::R](R) reader structure"]
impl crate::Readable for DMACUTRXBUFR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DMACUTRXBUFR to value 0"]
impl crate::Resettable for DMACUTRXBUFR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
