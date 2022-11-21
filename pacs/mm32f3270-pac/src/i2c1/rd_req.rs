#[doc = "Register `RD_REQ` reader"]
pub struct R(crate::R<RD_REQ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RD_REQ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RD_REQ_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RD_REQ_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RD_REQ` reader - Read this register to clear the RX_UNDER interrupt(bit 5)of the RAWISR register"]
pub type RD_REQ_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Read this register to clear the RX_UNDER interrupt(bit 5)of the RAWISR register"]
    #[inline(always)]
    pub fn rd_req(&self) -> RD_REQ_R {
        RD_REQ_R::new((self.bits & 1) != 0)
    }
}
#[doc = "Clear RD_REQ Interrupt Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rd_req](index.html) module\n\nThe register is **cleared** (set to zero) following a read operation."]
pub struct RD_REQ_SPEC;
impl crate::RegisterSpec for RD_REQ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rd_req::R](R) reader structure"]
impl crate::Readable for RD_REQ_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RD_REQ to value 0"]
impl crate::Resettable for RD_REQ_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
