#[doc = "Register `RX_DONE` reader"]
pub struct R(crate::R<RX_DONE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RX_DONE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RX_DONE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RX_DONE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RX_DONE` reader - Read this register to clear the RX_UNDER interrupt(bit 7)of the RAWISR register"]
pub type RX_DONE_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Read this register to clear the RX_UNDER interrupt(bit 7)of the RAWISR register"]
    #[inline(always)]
    pub fn rx_done(&self) -> RX_DONE_R {
        RX_DONE_R::new((self.bits & 1) != 0)
    }
}
#[doc = "Clear RX_DONE Interrupt Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx_done](index.html) module\n\nThe register is **cleared** (set to zero) following a read operation."]
pub struct RX_DONE_SPEC;
impl crate::RegisterSpec for RX_DONE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rx_done::R](R) reader structure"]
impl crate::Readable for RX_DONE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RX_DONE to value 0"]
impl crate::Resettable for RX_DONE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
