#[doc = "Register `TX_OVER` reader"]
pub struct R(crate::R<TX_OVER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TX_OVER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TX_OVER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TX_OVER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TX_OVER` reader - Read this register to clear the RX_UNDER interrupt(bit 3)of the RAWISR register"]
pub type TX_OVER_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Read this register to clear the RX_UNDER interrupt(bit 3)of the RAWISR register"]
    #[inline(always)]
    pub fn tx_over(&self) -> TX_OVER_R {
        TX_OVER_R::new((self.bits & 1) != 0)
    }
}
#[doc = "Clear TX_OVER Interrupt Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tx_over](index.html) module\n\nThe register is **cleared** (set to zero) following a read operation."]
pub struct TX_OVER_SPEC;
impl crate::RegisterSpec for TX_OVER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tx_over::R](R) reader structure"]
impl crate::Readable for TX_OVER_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TX_OVER to value 0"]
impl crate::Resettable for TX_OVER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
