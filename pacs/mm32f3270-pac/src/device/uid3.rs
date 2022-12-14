#[doc = "Register `UID3` reader"]
pub struct R(crate::R<UID3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UID3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UID3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UID3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `U_ID` reader - 63:32 unique ID bits"]
pub type U_ID_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - 63:32 unique ID bits"]
    #[inline(always)]
    pub fn u_id(&self) -> U_ID_R {
        U_ID_R::new(self.bits)
    }
}
#[doc = "Unique product indentification register 3\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uid3](index.html) module"]
pub struct UID3_SPEC;
impl crate::RegisterSpec for UID3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uid3::R](R) reader structure"]
impl crate::Readable for UID3_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets UID3 to value 0"]
impl crate::Resettable for UID3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
