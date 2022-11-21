#[doc = "Register `DMAWDTR` reader"]
pub struct R(crate::R<DMAWDTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMAWDTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMAWDTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMAWDTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RIWT` reader - *D0"]
pub type RIWT_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - *D0"]
    #[inline(always)]
    pub fn riwt(&self) -> RIWT_R {
        RIWT_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Ethernet Watchdog register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmawdtr](index.html) module\n\nThe register is **cleared** (set to zero) following a read operation."]
pub struct DMAWDTR_SPEC;
impl crate::RegisterSpec for DMAWDTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmawdtr::R](R) reader structure"]
impl crate::Readable for DMAWDTR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DMAWDTR to value 0"]
impl crate::Resettable for DMAWDTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
