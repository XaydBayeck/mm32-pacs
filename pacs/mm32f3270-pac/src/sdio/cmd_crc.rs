#[doc = "Register `CMD_CRC` reader"]
pub struct R(crate::R<CMD_CRC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMD_CRC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMD_CRC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMD_CRC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `VAL` reader - CMD CRC register value"]
pub type VAL_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:6 - CMD CRC register value"]
    #[inline(always)]
    pub fn val(&self) -> VAL_R {
        VAL_R::new((self.bits & 0x7f) as u8)
    }
}
#[doc = "CMD_CRC register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmd_crc](index.html) module"]
pub struct CMD_CRC_SPEC;
impl crate::RegisterSpec for CMD_CRC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmd_crc::R](R) reader structure"]
impl crate::Readable for CMD_CRC_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CMD_CRC to value 0"]
impl crate::Resettable for CMD_CRC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
