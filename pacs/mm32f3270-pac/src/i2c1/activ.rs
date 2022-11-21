#[doc = "Register `ACTIV` reader"]
pub struct R(crate::R<ACTIV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ACTIV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ACTIV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ACTIV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ACTIV` reader - Reading this register clears the ACTIVITY interrupt if the I2C is not active anymore"]
pub type ACTIV_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Reading this register clears the ACTIVITY interrupt if the I2C is not active anymore"]
    #[inline(always)]
    pub fn activ(&self) -> ACTIV_R {
        ACTIV_R::new((self.bits & 1) != 0)
    }
}
#[doc = "Clear ACTIVITY Interrupt Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [activ](index.html) module\n\nThe register is **cleared** (set to zero) following a read operation."]
pub struct ACTIV_SPEC;
impl crate::RegisterSpec for ACTIV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [activ::R](R) reader structure"]
impl crate::Readable for ACTIV_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ACTIV to value 0"]
impl crate::Resettable for ACTIV_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
