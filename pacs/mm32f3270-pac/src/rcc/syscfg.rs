#[doc = "Register `SYSCFG` reader"]
pub struct R(crate::R<SYSCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYSCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYSCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYSCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYSCFG` writer"]
pub struct W(crate::W<SYSCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYSCFG_SPEC>;
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
impl From<crate::W<SYSCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYSCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PROG_CHECK_EN` reader - Whether to check if the data in Flash is 0xFF when writing Flash"]
pub type PROG_CHECK_EN_R = crate::BitReader<bool>;
#[doc = "Field `PROG_CHECK_EN` writer - Whether to check if the data in Flash is 0xFF when writing Flash"]
pub type PROG_CHECK_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYSCFG_SPEC, bool, O>;
#[doc = "Field `SECTOR_1K_CFG` reader - The size of flash page erase"]
pub type SECTOR_1K_CFG_R = crate::BitReader<bool>;
#[doc = "Field `SECTOR_1K_CFG` writer - The size of flash page erase"]
pub type SECTOR_1K_CFG_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYSCFG_SPEC, bool, O>;
#[doc = "Field `SFT_NRST_RMP` reader - Software mapping NRST"]
pub type SFT_NRST_RMP_R = crate::BitReader<bool>;
#[doc = "Field `SFT_NRST_RMP` writer - Software mapping NRST"]
pub type SFT_NRST_RMP_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYSCFG_SPEC, bool, O>;
#[doc = "Field `PAD_OSC_TRIM` reader - Calibration of external crystal oscillator"]
pub type PAD_OSC_TRIM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PAD_OSC_TRIM` writer - Calibration of external crystal oscillator"]
pub type PAD_OSC_TRIM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SYSCFG_SPEC, u8, u8, 5, O>;
#[doc = "Field `OSC_LPF_EN` reader - External crystal low-pass filter enable"]
pub type OSC_LPF_EN_R = crate::BitReader<bool>;
#[doc = "Field `OSC_LPF_EN` writer - External crystal low-pass filter enable"]
pub type OSC_LPF_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYSCFG_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Whether to check if the data in Flash is 0xFF when writing Flash"]
    #[inline(always)]
    pub fn prog_check_en(&self) -> PROG_CHECK_EN_R {
        PROG_CHECK_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The size of flash page erase"]
    #[inline(always)]
    pub fn sector_1k_cfg(&self) -> SECTOR_1K_CFG_R {
        SECTOR_1K_CFG_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Software mapping NRST"]
    #[inline(always)]
    pub fn sft_nrst_rmp(&self) -> SFT_NRST_RMP_R {
        SFT_NRST_RMP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 8:12 - Calibration of external crystal oscillator"]
    #[inline(always)]
    pub fn pad_osc_trim(&self) -> PAD_OSC_TRIM_R {
        PAD_OSC_TRIM_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 14 - External crystal low-pass filter enable"]
    #[inline(always)]
    pub fn osc_lpf_en(&self) -> OSC_LPF_EN_R {
        OSC_LPF_EN_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Whether to check if the data in Flash is 0xFF when writing Flash"]
    #[inline(always)]
    #[must_use]
    pub fn prog_check_en(&mut self) -> PROG_CHECK_EN_W<0> {
        PROG_CHECK_EN_W::new(self)
    }
    #[doc = "Bit 1 - The size of flash page erase"]
    #[inline(always)]
    #[must_use]
    pub fn sector_1k_cfg(&mut self) -> SECTOR_1K_CFG_W<1> {
        SECTOR_1K_CFG_W::new(self)
    }
    #[doc = "Bit 2 - Software mapping NRST"]
    #[inline(always)]
    #[must_use]
    pub fn sft_nrst_rmp(&mut self) -> SFT_NRST_RMP_W<2> {
        SFT_NRST_RMP_W::new(self)
    }
    #[doc = "Bits 8:12 - Calibration of external crystal oscillator"]
    #[inline(always)]
    #[must_use]
    pub fn pad_osc_trim(&mut self) -> PAD_OSC_TRIM_W<8> {
        PAD_OSC_TRIM_W::new(self)
    }
    #[doc = "Bit 14 - External crystal low-pass filter enable"]
    #[inline(always)]
    #[must_use]
    pub fn osc_lpf_en(&mut self) -> OSC_LPF_EN_W<14> {
        OSC_LPF_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syscfg](index.html) module"]
pub struct SYSCFG_SPEC;
impl crate::RegisterSpec for SYSCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [syscfg::R](R) reader structure"]
impl crate::Readable for SYSCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [syscfg::W](W) writer structure"]
impl crate::Writable for SYSCFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SYSCFG to value 0x0101"]
impl crate::Resettable for SYSCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0x0101;
}
