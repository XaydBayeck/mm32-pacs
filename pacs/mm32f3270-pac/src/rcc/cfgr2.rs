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
#[doc = "Field `TIMADV_CLKSEL` reader - TIMADV clock selection"]
pub type TIMADV_CLKSEL_R = crate::BitReader<bool>;
#[doc = "Field `TIMADV_CLKSEL` writer - TIMADV clock selection"]
pub type TIMADV_CLKSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR2_SPEC, bool, O>;
#[doc = "Field `TIMADV_PRE` reader - SYSCLK clock Frequency division coefficient"]
pub type TIMADV_PRE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TIMADV_PRE` writer - SYSCLK clock Frequency division coefficient"]
pub type TIMADV_PRE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFGR2_SPEC, u8, u8, 3, O>;
#[doc = "Field `FSMC_PRE` reader - *D8"]
pub type FSMC_PRE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FSMC_PRE` writer - *D8"]
pub type FSMC_PRE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFGR2_SPEC, u8, u8, 5, O>;
#[doc = "Field `APB1_CLK_HV_PRE` reader - FSMC_PRE clock Frequency division coefficient"]
pub type APB1_CLK_HV_PRE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `APB1_CLK_HV_PRE` writer - FSMC_PRE clock Frequency division coefficient"]
pub type APB1_CLK_HV_PRE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFGR2_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bit 0 - TIMADV clock selection"]
    #[inline(always)]
    pub fn timadv_clksel(&self) -> TIMADV_CLKSEL_R {
        TIMADV_CLKSEL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - SYSCLK clock Frequency division coefficient"]
    #[inline(always)]
    pub fn timadv_pre(&self) -> TIMADV_PRE_R {
        TIMADV_PRE_R::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bits 8:12 - *D8"]
    #[inline(always)]
    pub fn fsmc_pre(&self) -> FSMC_PRE_R {
        FSMC_PRE_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:19 - FSMC_PRE clock Frequency division coefficient"]
    #[inline(always)]
    pub fn apb1_clk_hv_pre(&self) -> APB1_CLK_HV_PRE_R {
        APB1_CLK_HV_PRE_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - TIMADV clock selection"]
    #[inline(always)]
    #[must_use]
    pub fn timadv_clksel(&mut self) -> TIMADV_CLKSEL_W<0> {
        TIMADV_CLKSEL_W::new(self)
    }
    #[doc = "Bits 1:3 - SYSCLK clock Frequency division coefficient"]
    #[inline(always)]
    #[must_use]
    pub fn timadv_pre(&mut self) -> TIMADV_PRE_W<1> {
        TIMADV_PRE_W::new(self)
    }
    #[doc = "Bits 8:12 - *D8"]
    #[inline(always)]
    #[must_use]
    pub fn fsmc_pre(&mut self) -> FSMC_PRE_W<8> {
        FSMC_PRE_W::new(self)
    }
    #[doc = "Bits 16:19 - FSMC_PRE clock Frequency division coefficient"]
    #[inline(always)]
    #[must_use]
    pub fn apb1_clk_hv_pre(&mut self) -> APB1_CLK_HV_PRE_W<16> {
        APB1_CLK_HV_PRE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configure Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfgr2](index.html) module"]
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
#[doc = "`reset()` method sets CFGR2 to value 0x0003_1f00"]
impl crate::Resettable for CFGR2_SPEC {
    const RESET_VALUE: Self::Ux = 0x0003_1f00;
}
