#[doc = "Register `UID4` reader"]
pub struct R(crate::R<UID4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UID4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UID4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UID4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `U_ID` reader - 95:64 unique ID bits"]
pub type U_ID_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - 95:64 unique ID bits"]
    #[inline(always)]
    pub fn u_id(&self) -> U_ID_R {
        U_ID_R::new(self.bits)
    }
}
#[doc = "Unique product indentification register 4\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uid4](index.html) module"]
pub struct UID4_SPEC;
impl crate::RegisterSpec for UID4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uid4::R](R) reader structure"]
impl crate::Readable for UID4_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets UID4 to value 0"]
impl crate::Resettable for UID4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
