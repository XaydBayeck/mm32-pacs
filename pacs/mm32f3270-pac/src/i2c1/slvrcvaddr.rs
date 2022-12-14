#[doc = "Register `SLVRCVADDR` reader"]
pub struct R(crate::R<SLVRCVADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SLVRCVADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SLVRCVADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SLVRCVADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ADDR` reader - Slave Address"]
pub type ADDR_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:9 - Slave Address"]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new((self.bits & 0x03ff) as u16)
    }
}
#[doc = "Receiver Address Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [slvrcvaddr](index.html) module"]
pub struct SLVRCVADDR_SPEC;
impl crate::RegisterSpec for SLVRCVADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [slvrcvaddr::R](R) reader structure"]
impl crate::Readable for SLVRCVADDR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SLVRCVADDR to value 0"]
impl crate::Resettable for SLVRCVADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
