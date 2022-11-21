#[doc = "Register `MBCR` reader"]
pub struct R(crate::R<MBCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MBCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MBCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MBCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MBCR` writer"]
pub struct W(crate::W<MBCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MBCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<MBCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MBCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PMBDTREN` reader - Set SD/MMC/SDIO port auto multiple block data transfer"]
pub type PMBDTREN_R = crate::BitReader<bool>;
#[doc = "Field `PMBDTREN` writer - Set SD/MMC/SDIO port auto multiple block data transfer"]
pub type PMBDTREN_W<'a, const O: u8> = crate::BitWriter<'a, u32, MBCR_SPEC, bool, O>;
#[doc = "Field `MBDDIR` reader - Select multiple block data transfer direction"]
pub type MBDDIR_R = crate::BitReader<bool>;
#[doc = "Field `MBDDIR` writer - Select multiple block data transfer direction"]
pub type MBDDIR_W<'a, const O: u8> = crate::BitWriter<'a, u32, MBCR_SPEC, bool, O>;
#[doc = "Field `PAUTOTR` reader - Set SD/MMC/SDIO port full auto cmd and multiple block data transferring"]
pub type PAUTOTR_R = crate::BitReader<bool>;
#[doc = "Field `PAUTOTR` writer - Set SD/MMC/SDIO port full auto cmd and multiple block data transferring"]
pub type PAUTOTR_W<'a, const O: u8> = crate::BitWriter<'a, u32, MBCR_SPEC, bool, O>;
#[doc = "Field `PCLKP` reader - SD/MMC/SDIO port CLK line polarity"]
pub type PCLKP_R = crate::BitReader<bool>;
#[doc = "Field `PCLKP` writer - SD/MMC/SDIO port CLK line polarity"]
pub type PCLKP_W<'a, const O: u8> = crate::BitWriter<'a, u32, MBCR_SPEC, bool, O>;
#[doc = "Field `BTSSEL` reader - SD/MMC/SDIO Busy timeout scale selection"]
pub type BTSSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BTSSEL` writer - SD/MMC/SDIO Busy timeout scale selection"]
pub type BTSSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MBCR_SPEC, u8, u8, 2, O>;
#[doc = "Field `NTSSEL` reader - SD/MMC/SDIO NAC timeout scale selection"]
pub type NTSSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `NTSSEL` writer - SD/MMC/SDIO NAC timeout scale selection"]
pub type NTSSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MBCR_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bit 0 - Set SD/MMC/SDIO port auto multiple block data transfer"]
    #[inline(always)]
    pub fn pmbdtren(&self) -> PMBDTREN_R {
        PMBDTREN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Select multiple block data transfer direction"]
    #[inline(always)]
    pub fn mbddir(&self) -> MBDDIR_R {
        MBDDIR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Set SD/MMC/SDIO port full auto cmd and multiple block data transferring"]
    #[inline(always)]
    pub fn pautotr(&self) -> PAUTOTR_R {
        PAUTOTR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SD/MMC/SDIO port CLK line polarity"]
    #[inline(always)]
    pub fn pclkp(&self) -> PCLKP_R {
        PCLKP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - SD/MMC/SDIO Busy timeout scale selection"]
    #[inline(always)]
    pub fn btssel(&self) -> BTSSEL_R {
        BTSSEL_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - SD/MMC/SDIO NAC timeout scale selection"]
    #[inline(always)]
    pub fn ntssel(&self) -> NTSSEL_R {
        NTSSEL_R::new(((self.bits >> 6) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Set SD/MMC/SDIO port auto multiple block data transfer"]
    #[inline(always)]
    #[must_use]
    pub fn pmbdtren(&mut self) -> PMBDTREN_W<0> {
        PMBDTREN_W::new(self)
    }
    #[doc = "Bit 1 - Select multiple block data transfer direction"]
    #[inline(always)]
    #[must_use]
    pub fn mbddir(&mut self) -> MBDDIR_W<1> {
        MBDDIR_W::new(self)
    }
    #[doc = "Bit 2 - Set SD/MMC/SDIO port full auto cmd and multiple block data transferring"]
    #[inline(always)]
    #[must_use]
    pub fn pautotr(&mut self) -> PAUTOTR_W<2> {
        PAUTOTR_W::new(self)
    }
    #[doc = "Bit 3 - SD/MMC/SDIO port CLK line polarity"]
    #[inline(always)]
    #[must_use]
    pub fn pclkp(&mut self) -> PCLKP_W<3> {
        PCLKP_W::new(self)
    }
    #[doc = "Bits 4:5 - SD/MMC/SDIO Busy timeout scale selection"]
    #[inline(always)]
    #[must_use]
    pub fn btssel(&mut self) -> BTSSEL_W<4> {
        BTSSEL_W::new(self)
    }
    #[doc = "Bits 6:7 - SD/MMC/SDIO NAC timeout scale selection"]
    #[inline(always)]
    #[must_use]
    pub fn ntssel(&mut self) -> NTSSEL_W<6> {
        NTSSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Multi-block data transmission register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mbcr](index.html) module"]
pub struct MBCR_SPEC;
impl crate::RegisterSpec for MBCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mbcr::R](R) reader structure"]
impl crate::Readable for MBCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mbcr::W](W) writer structure"]
impl crate::Writable for MBCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MBCR to value 0x10"]
impl crate::Resettable for MBCR_SPEC {
    const RESET_VALUE: Self::Ux = 0x10;
}
