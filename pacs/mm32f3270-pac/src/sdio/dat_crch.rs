#[doc = "Register `DAT_CRCH` reader"]
pub struct R(crate::R<DAT_CRCH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DAT_CRCH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DAT_CRCH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DAT_CRCH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `VAL` reader - The DAT CRC high register value"]
pub type VAL_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - The DAT CRC high register value"]
    #[inline(always)]
    pub fn val(&self) -> VAL_R {
        VAL_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "CRC high data register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dat_crch](index.html) module"]
pub struct DAT_CRCH_SPEC;
impl crate::RegisterSpec for DAT_CRCH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dat_crch::R](R) reader structure"]
impl crate::Readable for DAT_CRCH_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DAT_CRCH to value 0"]
impl crate::Resettable for DAT_CRCH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
