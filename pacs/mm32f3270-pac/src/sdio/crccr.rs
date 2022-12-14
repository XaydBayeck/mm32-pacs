#[doc = "Register `CRCCR` reader"]
pub struct R(crate::R<CRCCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CRCCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CRCCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CRCCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CRCCR` writer"]
pub struct W(crate::W<CRCCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CRCCR_SPEC>;
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
impl From<crate::W<CRCCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CRCCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DAT_CRCE` reader - DAT CRC Error"]
pub type DAT_CRCE_R = crate::BitReader<bool>;
#[doc = "Field `DAT_CRCE` writer - DAT CRC Error"]
pub type DAT_CRCE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CRCCR_SPEC, bool, O>;
#[doc = "Field `CMD_CRCE` reader - CMD CRC Error"]
pub type CMD_CRCE_R = crate::BitReader<bool>;
#[doc = "Field `CMD_CRCE` writer - CMD CRC Error"]
pub type CMD_CRCE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CRCCR_SPEC, bool, O>;
#[doc = "Field `DAT_CRCS` reader - DAT CRC selection"]
pub type DAT_CRCS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DAT_CRCS` writer - DAT CRC selection"]
pub type DAT_CRCS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CRCCR_SPEC, u8, u8, 2, O>;
#[doc = "Field `RDMBEN` reader - Enable read multiple block data before response"]
pub type RDMBEN_R = crate::BitReader<bool>;
#[doc = "Field `RDMBEN` writer - Enable read multiple block data before response"]
pub type RDMBEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CRCCR_SPEC, bool, O>;
#[doc = "Field `CHKEN` reader - Enable auto check crc_status\\[2:0\\]"]
pub type CHKEN_R = crate::BitReader<bool>;
#[doc = "Field `CHKEN` writer - Enable auto check crc_status\\[2:0\\]"]
pub type CHKEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CRCCR_SPEC, bool, O>;
#[doc = "Field `DAT_CRCEN` reader - SD/MMC/SDIO port DAT Line CRC circuit enable"]
pub type DAT_CRCEN_R = crate::BitReader<bool>;
#[doc = "Field `DAT_CRCEN` writer - SD/MMC/SDIO port DAT Line CRC circuit enable"]
pub type DAT_CRCEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CRCCR_SPEC, bool, O>;
#[doc = "Field `CMD_CRCEN` reader - SD/MMC/SDIO port CMD Line CRC circuit enable."]
pub type CMD_CRCEN_R = crate::BitReader<bool>;
#[doc = "Field `CMD_CRCEN` writer - SD/MMC/SDIO port CMD Line CRC circuit enable."]
pub type CMD_CRCEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CRCCR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - DAT CRC Error"]
    #[inline(always)]
    pub fn dat_crce(&self) -> DAT_CRCE_R {
        DAT_CRCE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CMD CRC Error"]
    #[inline(always)]
    pub fn cmd_crce(&self) -> CMD_CRCE_R {
        CMD_CRCE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - DAT CRC selection"]
    #[inline(always)]
    pub fn dat_crcs(&self) -> DAT_CRCS_R {
        DAT_CRCS_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - Enable read multiple block data before response"]
    #[inline(always)]
    pub fn rdmben(&self) -> RDMBEN_R {
        RDMBEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable auto check crc_status\\[2:0\\]"]
    #[inline(always)]
    pub fn chken(&self) -> CHKEN_R {
        CHKEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SD/MMC/SDIO port DAT Line CRC circuit enable"]
    #[inline(always)]
    pub fn dat_crcen(&self) -> DAT_CRCEN_R {
        DAT_CRCEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - SD/MMC/SDIO port CMD Line CRC circuit enable."]
    #[inline(always)]
    pub fn cmd_crcen(&self) -> CMD_CRCEN_R {
        CMD_CRCEN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DAT CRC Error"]
    #[inline(always)]
    #[must_use]
    pub fn dat_crce(&mut self) -> DAT_CRCE_W<0> {
        DAT_CRCE_W::new(self)
    }
    #[doc = "Bit 1 - CMD CRC Error"]
    #[inline(always)]
    #[must_use]
    pub fn cmd_crce(&mut self) -> CMD_CRCE_W<1> {
        CMD_CRCE_W::new(self)
    }
    #[doc = "Bits 2:3 - DAT CRC selection"]
    #[inline(always)]
    #[must_use]
    pub fn dat_crcs(&mut self) -> DAT_CRCS_W<2> {
        DAT_CRCS_W::new(self)
    }
    #[doc = "Bit 4 - Enable read multiple block data before response"]
    #[inline(always)]
    #[must_use]
    pub fn rdmben(&mut self) -> RDMBEN_W<4> {
        RDMBEN_W::new(self)
    }
    #[doc = "Bit 5 - Enable auto check crc_status\\[2:0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn chken(&mut self) -> CHKEN_W<5> {
        CHKEN_W::new(self)
    }
    #[doc = "Bit 6 - SD/MMC/SDIO port DAT Line CRC circuit enable"]
    #[inline(always)]
    #[must_use]
    pub fn dat_crcen(&mut self) -> DAT_CRCEN_W<6> {
        DAT_CRCEN_W::new(self)
    }
    #[doc = "Bit 7 - SD/MMC/SDIO port CMD Line CRC circuit enable."]
    #[inline(always)]
    #[must_use]
    pub fn cmd_crcen(&mut self) -> CMD_CRCEN_W<7> {
        CMD_CRCEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CRC control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crccr](index.html) module"]
pub struct CRCCR_SPEC;
impl crate::RegisterSpec for CRCCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [crccr::R](R) reader structure"]
impl crate::Readable for CRCCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [crccr::W](W) writer structure"]
impl crate::Writable for CRCCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CRCCR to value 0"]
impl crate::Resettable for CRCCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
