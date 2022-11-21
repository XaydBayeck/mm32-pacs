#[doc = "Register `ISR` reader"]
pub struct R(crate::R<ISR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ISR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ISR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ISR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SYNCOKF` reader - SYNC event OK flag"]
pub type SYNCOKF_R = crate::BitReader<bool>;
#[doc = "Field `WARNIF` reader - SYNC warning flag"]
pub type WARNIF_R = crate::BitReader<bool>;
#[doc = "Field `ERRIF` reader - Error flag"]
pub type ERRIF_R = crate::BitReader<bool>;
#[doc = "Field `ESYNCF` reader - Expected SYNC flag"]
pub type ESYNCF_R = crate::BitReader<bool>;
#[doc = "Field `ERR` reader - SYNC error"]
pub type ERR_R = crate::BitReader<bool>;
#[doc = "Field `MISS` reader - SYNC missed"]
pub type MISS_R = crate::BitReader<bool>;
#[doc = "Field `TRIMOVF` reader - Trimming overflow or underflow"]
pub type TRIMOVF_R = crate::BitReader<bool>;
#[doc = "Field `FEDIR` reader - Frequency error direction"]
pub type FEDIR_R = crate::BitReader<bool>;
#[doc = "Field `FECAP` reader - Frequency error capture"]
pub type FECAP_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bit 0 - SYNC event OK flag"]
    #[inline(always)]
    pub fn syncokf(&self) -> SYNCOKF_R {
        SYNCOKF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SYNC warning flag"]
    #[inline(always)]
    pub fn warnif(&self) -> WARNIF_R {
        WARNIF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Error flag"]
    #[inline(always)]
    pub fn errif(&self) -> ERRIF_R {
        ERRIF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Expected SYNC flag"]
    #[inline(always)]
    pub fn esyncf(&self) -> ESYNCF_R {
        ESYNCF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - SYNC error"]
    #[inline(always)]
    pub fn err(&self) -> ERR_R {
        ERR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SYNC missed"]
    #[inline(always)]
    pub fn miss(&self) -> MISS_R {
        MISS_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Trimming overflow or underflow"]
    #[inline(always)]
    pub fn trimovf(&self) -> TRIMOVF_R {
        TRIMOVF_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 15 - Frequency error direction"]
    #[inline(always)]
    pub fn fedir(&self) -> FEDIR_R {
        FEDIR_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:31 - Frequency error capture"]
    #[inline(always)]
    pub fn fecap(&self) -> FECAP_R {
        FECAP_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "Interrupt status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isr](index.html) module"]
pub struct ISR_SPEC;
impl crate::RegisterSpec for ISR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [isr::R](R) reader structure"]
impl crate::Readable for ISR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ISR to value 0"]
impl crate::Resettable for ISR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
