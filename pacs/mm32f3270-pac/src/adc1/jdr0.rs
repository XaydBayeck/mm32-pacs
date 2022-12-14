#[doc = "Register `JDR0` reader"]
pub struct R(crate::R<JDR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<JDR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<JDR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<JDR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `JDATA` reader - Transfer data"]
pub type JDATA_R = crate::FieldReader<u16, u16>;
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
#[doc = "Injection data register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [jdr0](index.html) module"]
pub struct JDR0_SPEC;
impl crate::RegisterSpec for JDR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [jdr0::R](R) reader structure"]
impl crate::Readable for JDR0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets JDR0 to value 0"]
impl crate::Resettable for JDR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
