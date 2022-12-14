#[doc = "Register `I2S_CFGR` reader"]
pub struct R(crate::R<I2S_CFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2S_CFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2S_CFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2S_CFGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `I2S_CFGR` writer"]
pub struct W(crate::W<I2S_CFGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2S_CFGR_SPEC>;
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
impl From<crate::W<I2S_CFGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I2S_CFGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CHLEN` reader - Channel length"]
pub type CHLEN_R = crate::BitReader<bool>;
#[doc = "Field `CHLEN` writer - Channel length"]
pub type CHLEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, I2S_CFGR_SPEC, bool, O>;
#[doc = "Field `DATLEN` reader - Audio data width"]
pub type DATLEN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DATLEN` writer - Audio data width"]
pub type DATLEN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, I2S_CFGR_SPEC, u8, u8, 2, O>;
#[doc = "Field `I2SSTD` reader - I2S standard selection"]
pub type I2SSTD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `I2SSTD` writer - I2S standard selection"]
pub type I2SSTD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, I2S_CFGR_SPEC, u8, u8, 2, O>;
#[doc = "Field `PCMSYNC` reader - PCM frame synchronization mode"]
pub type PCMSYNC_R = crate::BitReader<bool>;
#[doc = "Field `PCMSYNC` writer - PCM frame synchronization mode"]
pub type PCMSYNC_W<'a, const O: u8> = crate::BitWriter<'a, u32, I2S_CFGR_SPEC, bool, O>;
#[doc = "Field `SPI_I2S` reader - SPI/I2S module function selection"]
pub type SPI_I2S_R = crate::BitReader<bool>;
#[doc = "Field `SPI_I2S` writer - SPI/I2S module function selection"]
pub type SPI_I2S_W<'a, const O: u8> = crate::BitWriter<'a, u32, I2S_CFGR_SPEC, bool, O>;
#[doc = "Field `MCKOE` reader - I2S master clock output enable"]
pub type MCKOE_R = crate::BitReader<bool>;
#[doc = "Field `MCKOE` writer - I2S master clock output enable"]
pub type MCKOE_W<'a, const O: u8> = crate::BitWriter<'a, u32, I2S_CFGR_SPEC, bool, O>;
#[doc = "Field `I2SDIV` reader - Frequency division coefficient of I2S prescaler, configuration is valid in main mode"]
pub type I2SDIV_R = crate::FieldReader<u16, u16>;
#[doc = "Field `I2SDIV` writer - Frequency division coefficient of I2S prescaler, configuration is valid in main mode"]
pub type I2SDIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, I2S_CFGR_SPEC, u16, u16, 9, O>;
impl R {
    #[doc = "Bit 0 - Channel length"]
    #[inline(always)]
    pub fn chlen(&self) -> CHLEN_R {
        CHLEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Audio data width"]
    #[inline(always)]
    pub fn datlen(&self) -> DATLEN_R {
        DATLEN_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bits 4:5 - I2S standard selection"]
    #[inline(always)]
    pub fn i2sstd(&self) -> I2SSTD_R {
        I2SSTD_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - PCM frame synchronization mode"]
    #[inline(always)]
    pub fn pcmsync(&self) -> PCMSYNC_R {
        PCMSYNC_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 10 - SPI/I2S module function selection"]
    #[inline(always)]
    pub fn spi_i2s(&self) -> SPI_I2S_R {
        SPI_I2S_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - I2S master clock output enable"]
    #[inline(always)]
    pub fn mckoe(&self) -> MCKOE_R {
        MCKOE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 16:24 - Frequency division coefficient of I2S prescaler, configuration is valid in main mode"]
    #[inline(always)]
    pub fn i2sdiv(&self) -> I2SDIV_R {
        I2SDIV_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Channel length"]
    #[inline(always)]
    #[must_use]
    pub fn chlen(&mut self) -> CHLEN_W<0> {
        CHLEN_W::new(self)
    }
    #[doc = "Bits 1:2 - Audio data width"]
    #[inline(always)]
    #[must_use]
    pub fn datlen(&mut self) -> DATLEN_W<1> {
        DATLEN_W::new(self)
    }
    #[doc = "Bits 4:5 - I2S standard selection"]
    #[inline(always)]
    #[must_use]
    pub fn i2sstd(&mut self) -> I2SSTD_W<4> {
        I2SSTD_W::new(self)
    }
    #[doc = "Bit 6 - PCM frame synchronization mode"]
    #[inline(always)]
    #[must_use]
    pub fn pcmsync(&mut self) -> PCMSYNC_W<6> {
        PCMSYNC_W::new(self)
    }
    #[doc = "Bit 10 - SPI/I2S module function selection"]
    #[inline(always)]
    #[must_use]
    pub fn spi_i2s(&mut self) -> SPI_I2S_W<10> {
        SPI_I2S_W::new(self)
    }
    #[doc = "Bit 11 - I2S master clock output enable"]
    #[inline(always)]
    #[must_use]
    pub fn mckoe(&mut self) -> MCKOE_W<11> {
        MCKOE_W::new(self)
    }
    #[doc = "Bits 16:24 - Frequency division coefficient of I2S prescaler, configuration is valid in main mode"]
    #[inline(always)]
    #[must_use]
    pub fn i2sdiv(&mut self) -> I2SDIV_W<16> {
        I2SDIV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2S Configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2s_cfgr](index.html) module"]
pub struct I2S_CFGR_SPEC;
impl crate::RegisterSpec for I2S_CFGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i2s_cfgr::R](R) reader structure"]
impl crate::Readable for I2S_CFGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i2s_cfgr::W](W) writer structure"]
impl crate::Writable for I2S_CFGR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets I2S_CFGR to value 0x01"]
impl crate::Resettable for I2S_CFGR_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
