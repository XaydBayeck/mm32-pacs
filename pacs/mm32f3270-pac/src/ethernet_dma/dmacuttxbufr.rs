#[doc = "Register `DMACUTTXBUFR` reader"]
pub struct R(crate::R<DMACUTTXBUFR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMACUTTXBUFR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMACUTTXBUFR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMACUTTXBUFR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CUTTXBUF` reader - Host Transmit Buffer Address Pointer"]
pub type CUTTXBUF_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Host Transmit Buffer Address Pointer"]
    #[inline(always)]
    pub fn cuttxbuf(&self) -> CUTTXBUF_R {
        CUTTXBUF_R::new(self.bits)
    }
}
#[doc = "Ethernet DMA current host transmit buffer address register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmacuttxbufr](index.html) module"]
pub struct DMACUTTXBUFR_SPEC;
impl crate::RegisterSpec for DMACUTTXBUFR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmacuttxbufr::R](R) reader structure"]
impl crate::Readable for DMACUTTXBUFR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DMACUTTXBUFR to value 0"]
impl crate::Resettable for DMACUTTXBUFR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
