#[doc = "Register `PS` reader"]
pub struct R(crate::R<PS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PS` reader - Watchdog prescaler counter value"]
pub type PS_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Watchdog prescaler counter value"]
    #[inline(always)]
    pub fn ps(&self) -> PS_R {
        PS_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "prescaler Counter\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ps](index.html) module"]
pub struct PS_SPEC;
impl crate::RegisterSpec for PS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ps::R](R) reader structure"]
impl crate::Readable for PS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PS to value 0x01"]
impl crate::Resettable for PS_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
