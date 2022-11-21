#[doc = "Register `CCR` reader"]
pub struct R(crate::R<CCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CCR` writer"]
pub struct W(crate::W<CCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCR_SPEC>;
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
impl From<crate::W<CCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CPHA` reader - Clock phase select bit"]
pub type CPHA_R = crate::BitReader<bool>;
#[doc = "Field `CPHA` writer - Clock phase select bit"]
pub type CPHA_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR_SPEC, bool, O>;
#[doc = "Field `CPOL` reader - Clock polarity select bit"]
pub type CPOL_R = crate::BitReader<bool>;
#[doc = "Field `CPOL` writer - Clock polarity select bit"]
pub type CPOL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR_SPEC, bool, O>;
#[doc = "Field `LSBFE` reader - LSI first enable bit"]
pub type LSBFE_R = crate::BitReader<bool>;
#[doc = "Field `LSBFE` writer - LSI first enable bit"]
pub type LSBFE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR_SPEC, bool, O>;
#[doc = "Field `SPILEN` reader - SPI character length bit"]
pub type SPILEN_R = crate::BitReader<bool>;
#[doc = "Field `SPILEN` writer - SPI character length bit"]
pub type SPILEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR_SPEC, bool, O>;
#[doc = "Field `RXEDGE` reader - Receive data edge select"]
pub type RXEDGE_R = crate::BitReader<bool>;
#[doc = "Field `RXEDGE` writer - Receive data edge select"]
pub type RXEDGE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR_SPEC, bool, O>;
#[doc = "Field `TXEDGE` reader - Transmit data edge select"]
pub type TXEDGE_R = crate::BitReader<bool>;
#[doc = "Field `TXEDGE` writer - Transmit data edge select"]
pub type TXEDGE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR_SPEC, bool, O>;
#[doc = "Field `CPHASEL` reader - CPHA polarity select"]
pub type CPHASEL_R = crate::BitReader<bool>;
#[doc = "Field `CPHASEL` writer - CPHA polarity select"]
pub type CPHASEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Clock phase select bit"]
    #[inline(always)]
    pub fn cpha(&self) -> CPHA_R {
        CPHA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Clock polarity select bit"]
    #[inline(always)]
    pub fn cpol(&self) -> CPOL_R {
        CPOL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - LSI first enable bit"]
    #[inline(always)]
    pub fn lsbfe(&self) -> LSBFE_R {
        LSBFE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SPI character length bit"]
    #[inline(always)]
    pub fn spilen(&self) -> SPILEN_R {
        SPILEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Receive data edge select"]
    #[inline(always)]
    pub fn rxedge(&self) -> RXEDGE_R {
        RXEDGE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Transmit data edge select"]
    #[inline(always)]
    pub fn txedge(&self) -> TXEDGE_R {
        TXEDGE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - CPHA polarity select"]
    #[inline(always)]
    pub fn cphasel(&self) -> CPHASEL_R {
        CPHASEL_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Clock phase select bit"]
    #[inline(always)]
    #[must_use]
    pub fn cpha(&mut self) -> CPHA_W<0> {
        CPHA_W::new(self)
    }
    #[doc = "Bit 1 - Clock polarity select bit"]
    #[inline(always)]
    #[must_use]
    pub fn cpol(&mut self) -> CPOL_W<1> {
        CPOL_W::new(self)
    }
    #[doc = "Bit 2 - LSI first enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn lsbfe(&mut self) -> LSBFE_W<2> {
        LSBFE_W::new(self)
    }
    #[doc = "Bit 3 - SPI character length bit"]
    #[inline(always)]
    #[must_use]
    pub fn spilen(&mut self) -> SPILEN_W<3> {
        SPILEN_W::new(self)
    }
    #[doc = "Bit 4 - Receive data edge select"]
    #[inline(always)]
    #[must_use]
    pub fn rxedge(&mut self) -> RXEDGE_W<4> {
        RXEDGE_W::new(self)
    }
    #[doc = "Bit 5 - Transmit data edge select"]
    #[inline(always)]
    #[must_use]
    pub fn txedge(&mut self) -> TXEDGE_W<5> {
        TXEDGE_W::new(self)
    }
    #[doc = "Bit 6 - CPHA polarity select"]
    #[inline(always)]
    #[must_use]
    pub fn cphasel(&mut self) -> CPHASEL_W<6> {
        CPHASEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Current control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccr](index.html) module"]
pub struct CCR_SPEC;
impl crate::RegisterSpec for CCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ccr::R](R) reader structure"]
impl crate::Readable for CCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ccr::W](W) writer structure"]
impl crate::Writable for CCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CCR to value 0x08"]
impl crate::Resettable for CCR_SPEC {
    const RESET_VALUE: Self::Ux = 0x08;
}
