#[doc = "Register `JDATA` reader"]
pub struct R(crate::R<JDATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<JDATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<JDATA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<JDATA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `JDATA` reader - Transfer data"]
pub type JDATA_R = crate::FieldReader<u16, u16>;
#[doc = "Field `JCHANNELSEL` reader - Channel selection"]
pub type JCHANNELSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `JOVERRUN` reader - Overrun flag"]
pub type JOVERRUN_R = crate::BitReader<bool>;
#[doc = "Field `JVALID` reader - Valid flag"]
pub type JVALID_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:15 - Transfer data"]
    #[inline(always)]
    pub fn jdata(&self) -> JDATA_R {
        JDATA_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:19 - Channel selection"]
    #[inline(always)]
    pub fn jchannelsel(&self) -> JCHANNELSEL_R {
        JCHANNELSEL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 21 - Overrun flag"]
    #[inline(always)]
    pub fn joverrun(&self) -> JOVERRUN_R {
        JOVERRUN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Valid flag"]
    #[inline(always)]
    pub fn jvalid(&self) -> JVALID_R {
        JVALID_R::new(((self.bits >> 22) & 1) != 0)
    }
}
#[doc = "Injection data register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [jdata](index.html) module"]
pub struct JDATA_SPEC;
impl crate::RegisterSpec for JDATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [jdata::R](R) reader structure"]
impl crate::Readable for JDATA_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets JDATA to value 0"]
impl crate::Resettable for JDATA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
