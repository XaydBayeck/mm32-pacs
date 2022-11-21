#[doc = "Register `UID1` reader"]
pub struct R(crate::R<UID1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UID1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UID1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UID1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `U_ID` reader - 15:0 unique ID bits"]
pub type U_ID_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0 unique ID bits"]
    #[inline(always)]
    pub fn u_id(&self) -> U_ID_R {
        U_ID_R::new(self.bits)
    }
}
#[doc = "Unique product indentification register 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uid1](index.html) module"]
pub struct UID1_SPEC;
impl crate::RegisterSpec for UID1_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [uid1::R](R) reader structure"]
impl crate::Readable for UID1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets UID1 to value 0"]
impl crate::Resettable for UID1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
