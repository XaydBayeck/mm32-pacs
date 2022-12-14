#[doc = "Register `STOP` reader"]
pub struct R(crate::R<STOP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STOP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STOP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STOP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `STOP` reader - Read this register to clear the STOP_DET interrupt(bit 9)of the RAWISR register"]
pub type STOP_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Read this register to clear the STOP_DET interrupt(bit 9)of the RAWISR register"]
    #[inline(always)]
    pub fn stop(&self) -> STOP_R {
        STOP_R::new((self.bits & 1) != 0)
    }
}
#[doc = "Clear STOP_DET Interrupt Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stop](index.html) module\n\nThe register is **cleared** (set to zero) following a read operation."]
pub struct STOP_SPEC;
impl crate::RegisterSpec for STOP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [stop::R](R) reader structure"]
impl crate::Readable for STOP_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STOP to value 0"]
impl crate::Resettable for STOP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
