#[doc = "Register `TX_ABRT` reader"]
pub struct R(crate::R<TX_ABRT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TX_ABRT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TX_ABRT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TX_ABRT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TX_ABRT` reader - Read this register to clear the RX_UNDER interrupt(bit 6)of the RAWISR register"]
pub type TX_ABRT_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Read this register to clear the RX_UNDER interrupt(bit 6)of the RAWISR register"]
    #[inline(always)]
    pub fn tx_abrt(&self) -> TX_ABRT_R {
        TX_ABRT_R::new((self.bits & 1) != 0)
    }
}
#[doc = "Clear TX_ABRT Interrupt Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tx_abrt](index.html) module\n\nThe register is **cleared** (set to zero) following a read operation."]
pub struct TX_ABRT_SPEC;
impl crate::RegisterSpec for TX_ABRT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tx_abrt::R](R) reader structure"]
impl crate::Readable for TX_ABRT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TX_ABRT to value 0"]
impl crate::Resettable for TX_ABRT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
