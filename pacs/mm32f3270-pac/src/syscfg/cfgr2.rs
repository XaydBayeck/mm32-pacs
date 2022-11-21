#[doc = "Register `CFGR2` reader"]
pub struct R(crate::R<CFGR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFGR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFGR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFGR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFGR2` writer"]
pub struct W(crate::W<CFGR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFGR2_SPEC>;
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
impl From<crate::W<CFGR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFGR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `I2C1_MODE_SEL` reader - I2C1 port mode selection bit"]
pub type I2C1_MODE_SEL_R = crate::BitReader<bool>;
#[doc = "Field `I2C1_MODE_SEL` writer - I2C1 port mode selection bit"]
pub type I2C1_MODE_SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR2_SPEC, bool, O>;
#[doc = "Field `I2C2_MODE_SEL` reader - I2C2 port mode selection bit"]
pub type I2C2_MODE_SEL_R = crate::BitReader<bool>;
#[doc = "Field `I2C2_MODE_SEL` writer - I2C2 port mode selection bit"]
pub type I2C2_MODE_SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR2_SPEC, bool, O>;
#[doc = "Field `MII_RMII_SEL` reader - Ethernet PHY interface selection"]
pub type MII_RMII_SEL_R = crate::BitReader<bool>;
#[doc = "Field `MII_RMII_SEL` writer - Ethernet PHY interface selection"]
pub type MII_RMII_SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR2_SPEC, bool, O>;
#[doc = "Field `MAC_SPD_SEL` reader - MAC simplifies media independent interface speed selection"]
pub type MAC_SPD_SEL_R = crate::BitReader<bool>;
#[doc = "Field `MAC_SPD_SEL` writer - MAC simplifies media independent interface speed selection"]
pub type MAC_SPD_SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR2_SPEC, bool, O>;
impl R {
    #[doc = "Bit 16 - I2C1 port mode selection bit"]
    #[inline(always)]
    pub fn i2c1_mode_sel(&self) -> I2C1_MODE_SEL_R {
        I2C1_MODE_SEL_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - I2C2 port mode selection bit"]
    #[inline(always)]
    pub fn i2c2_mode_sel(&self) -> I2C2_MODE_SEL_R {
        I2C2_MODE_SEL_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 20 - Ethernet PHY interface selection"]
    #[inline(always)]
    pub fn mii_rmii_sel(&self) -> MII_RMII_SEL_R {
        MII_RMII_SEL_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - MAC simplifies media independent interface speed selection"]
    #[inline(always)]
    pub fn mac_spd_sel(&self) -> MAC_SPD_SEL_R {
        MAC_SPD_SEL_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 16 - I2C1 port mode selection bit"]
    #[inline(always)]
    #[must_use]
    pub fn i2c1_mode_sel(&mut self) -> I2C1_MODE_SEL_W<16> {
        I2C1_MODE_SEL_W::new(self)
    }
    #[doc = "Bit 17 - I2C2 port mode selection bit"]
    #[inline(always)]
    #[must_use]
    pub fn i2c2_mode_sel(&mut self) -> I2C2_MODE_SEL_W<17> {
        I2C2_MODE_SEL_W::new(self)
    }
    #[doc = "Bit 20 - Ethernet PHY interface selection"]
    #[inline(always)]
    #[must_use]
    pub fn mii_rmii_sel(&mut self) -> MII_RMII_SEL_W<20> {
        MII_RMII_SEL_W::new(self)
    }
    #[doc = "Bit 21 - MAC simplifies media independent interface speed selection"]
    #[inline(always)]
    #[must_use]
    pub fn mac_spd_sel(&mut self) -> MAC_SPD_SEL_W<21> {
        MAC_SPD_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SYSCFG configuration register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfgr2](index.html) module"]
pub struct CFGR2_SPEC;
impl crate::RegisterSpec for CFGR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfgr2::R](R) reader structure"]
impl crate::Readable for CFGR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfgr2::W](W) writer structure"]
impl crate::Writable for CFGR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFGR2 to value 0"]
impl crate::Resettable for CFGR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
