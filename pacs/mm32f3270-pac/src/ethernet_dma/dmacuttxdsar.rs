#[doc = "Register `DMACUTTXDSAR` reader"]
pub struct R(crate::R<DMACUTTXDSAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMACUTTXDSAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMACUTTXDSAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMACUTTXDSAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CUTTXDSA` reader - Host Receive Descriptor AddressPointer"]
pub type CUTTXDSA_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Host Receive Descriptor AddressPointer"]
    #[inline(always)]
    pub fn cuttxdsa(&self) -> CUTTXDSA_R {
        CUTTXDSA_R::new(self.bits)
    }
}
#[doc = "Ethernet DMA current host transmit descriptor register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmacuttxdsar](index.html) module"]
pub struct DMACUTTXDSAR_SPEC;
impl crate::RegisterSpec for DMACUTTXDSAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmacuttxdsar::R](R) reader structure"]
impl crate::Readable for DMACUTTXDSAR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DMACUTTXDSAR to value 0"]
impl crate::Resettable for DMACUTTXDSAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
