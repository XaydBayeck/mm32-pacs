#[doc = "Register `PLLCFGR` reader"]
pub struct R(crate::R<PLLCFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PLLCFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PLLCFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PLLCFGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PLLCFGR` writer"]
pub struct W(crate::W<PLLCFGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PLLCFGR_SPEC>;
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
impl From<crate::W<PLLCFGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PLLCFGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PLLSRC` reader - PLL entry clock source"]
pub type PLLSRC_R = crate::BitReader<bool>;
#[doc = "Field `PLLSRC` writer - PLL entry clock source"]
pub type PLLSRC_W<'a, const O: u8> = crate::BitWriter<'a, u32, PLLCFGR_SPEC, bool, O>;
#[doc = "Field `PLLXTPRE` reader - HSE divider for PLL entry"]
pub type PLLXTPRE_R = crate::BitReader<bool>;
#[doc = "Field `PLLXTPRE` writer - HSE divider for PLL entry"]
pub type PLLXTPRE_W<'a, const O: u8> = crate::BitWriter<'a, u32, PLLCFGR_SPEC, bool, O>;
#[doc = "Field `PLL_ICTRL` reader - PLL CP current control signals"]
pub type PLL_ICTRL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PLL_ICTRL` writer - PLL CP current control signals"]
pub type PLL_ICTRL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PLLCFGR_SPEC, u8, u8, 2, O>;
#[doc = "Field `PLL_LDS` reader - PLL lock detector accuracy select"]
pub type PLL_LDS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PLL_LDS` writer - PLL lock detector accuracy select"]
pub type PLL_LDS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PLLCFGR_SPEC, u8, u8, 2, O>;
#[doc = "Field `PLLDIV` reader - PLL divide factor"]
pub type PLLDIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PLLDIV` writer - PLL divide factor"]
pub type PLLDIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PLLCFGR_SPEC, u8, u8, 3, O>;
#[doc = "Field `PLLMUL` reader - PLL multiplication factor"]
pub type PLLMUL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PLLMUL` writer - PLL multiplication factor"]
pub type PLLMUL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PLLCFGR_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bit 0 - PLL entry clock source"]
    #[inline(always)]
    pub fn pllsrc(&self) -> PLLSRC_R {
        PLLSRC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - HSE divider for PLL entry"]
    #[inline(always)]
    pub fn pllxtpre(&self) -> PLLXTPRE_R {
        PLLXTPRE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - PLL CP current control signals"]
    #[inline(always)]
    pub fn pll_ictrl(&self) -> PLL_ICTRL_R {
        PLL_ICTRL_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - PLL lock detector accuracy select"]
    #[inline(always)]
    pub fn pll_lds(&self) -> PLL_LDS_R {
        PLL_LDS_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:10 - PLL divide factor"]
    #[inline(always)]
    pub fn plldiv(&self) -> PLLDIV_R {
        PLLDIV_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 16:22 - PLL multiplication factor"]
    #[inline(always)]
    pub fn pllmul(&self) -> PLLMUL_R {
        PLLMUL_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - PLL entry clock source"]
    #[inline(always)]
    #[must_use]
    pub fn pllsrc(&mut self) -> PLLSRC_W<0> {
        PLLSRC_W::new(self)
    }
    #[doc = "Bit 1 - HSE divider for PLL entry"]
    #[inline(always)]
    #[must_use]
    pub fn pllxtpre(&mut self) -> PLLXTPRE_W<1> {
        PLLXTPRE_W::new(self)
    }
    #[doc = "Bits 2:3 - PLL CP current control signals"]
    #[inline(always)]
    #[must_use]
    pub fn pll_ictrl(&mut self) -> PLL_ICTRL_W<2> {
        PLL_ICTRL_W::new(self)
    }
    #[doc = "Bits 4:5 - PLL lock detector accuracy select"]
    #[inline(always)]
    #[must_use]
    pub fn pll_lds(&mut self) -> PLL_LDS_W<4> {
        PLL_LDS_W::new(self)
    }
    #[doc = "Bits 8:10 - PLL divide factor"]
    #[inline(always)]
    #[must_use]
    pub fn plldiv(&mut self) -> PLLDIV_W<8> {
        PLLDIV_W::new(self)
    }
    #[doc = "Bits 16:22 - PLL multiplication factor"]
    #[inline(always)]
    #[must_use]
    pub fn pllmul(&mut self) -> PLLMUL_W<16> {
        PLLMUL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PLL Configure Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pllcfgr](index.html) module"]
pub struct PLLCFGR_SPEC;
impl crate::RegisterSpec for PLLCFGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pllcfgr::R](R) reader structure"]
impl crate::Readable for PLLCFGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pllcfgr::W](W) writer structure"]
impl crate::Writable for PLLCFGR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PLLCFGR to value 0x0018_031c"]
impl crate::Resettable for PLLCFGR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0018_031c;
}
