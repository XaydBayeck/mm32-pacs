#[doc = "Register `CH7DR` reader"]
pub struct R(crate::R<CH7DR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CH7DR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CH7DR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CH7DR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DATA` reader - Transfer data"]
pub type DATA_R = crate::FieldReader<u16, u16>;
#[doc = "Field `OVERRUN` reader - Overrun flag"]
pub type OVERRUN_R = crate::BitReader<bool>;
#[doc = "Field `VALID` reader - Valid flag"]
pub type VALID_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:15 - Transfer data"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 20 - Overrun flag"]
    #[inline(always)]
    pub fn overrun(&self) -> OVERRUN_R {
        OVERRUN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Valid flag"]
    #[inline(always)]
    pub fn valid(&self) -> VALID_R {
        VALID_R::new(((self.bits >> 21) & 1) != 0)
    }
}
#[doc = "Channel 7 data register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch7dr](index.html) module"]
pub struct CH7DR_SPEC;
impl crate::RegisterSpec for CH7DR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ch7dr::R](R) reader structure"]
impl crate::Readable for CH7DR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CH7DR to value 0"]
impl crate::Resettable for CH7DR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
