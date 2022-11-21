#[doc = "Register `ECC_P` reader"]
pub struct R(crate::R<ECC_P_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ECC_P_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ECC_P_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ECC_P_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SEG` reader - SEG"]
pub type SEG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DIR` reader - Direction"]
pub type DIR_R = crate::BitReader<bool>;
#[doc = "Field `ERRC` reader - Error code"]
pub type ERRC_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:4 - SEG"]
    #[inline(always)]
    pub fn seg(&self) -> SEG_R {
        SEG_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5 - Direction"]
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Error code"]
    #[inline(always)]
    pub fn errc(&self) -> ERRC_R {
        ERRC_R::new(((self.bits >> 6) & 3) as u8)
    }
}
#[doc = "Peli Error Code Capture register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ecc_p](index.html) module"]
pub struct ECC_P_SPEC;
impl crate::RegisterSpec for ECC_P_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ecc_p::R](R) reader structure"]
impl crate::Readable for ECC_P_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ECC_P to value 0"]
impl crate::Resettable for ECC_P_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
