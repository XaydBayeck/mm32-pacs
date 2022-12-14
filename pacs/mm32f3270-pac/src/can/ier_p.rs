#[doc = "Register `IER_P` reader"]
pub struct R(crate::R<IER_P_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IER_P_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IER_P_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IER_P_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IER_P` writer"]
pub struct W(crate::W<IER_P_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IER_P_SPEC>;
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
impl From<crate::W<IER_P_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IER_P_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RIE` reader - Receive interrupt enable"]
pub type RIE_R = crate::BitReader<bool>;
#[doc = "Field `RIE` writer - Receive interrupt enable"]
pub type RIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_P_SPEC, bool, O>;
#[doc = "Field `TIE` reader - Transmit interrupt enable"]
pub type TIE_R = crate::BitReader<bool>;
#[doc = "Field `TIE` writer - Transmit interrupt enable"]
pub type TIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_P_SPEC, bool, O>;
#[doc = "Field `EIE` reader - Error interrupt enable"]
pub type EIE_R = crate::BitReader<bool>;
#[doc = "Field `EIE` writer - Error interrupt enable"]
pub type EIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_P_SPEC, bool, O>;
#[doc = "Field `DOIE` reader - Data overrun interrupt enable"]
pub type DOIE_R = crate::BitReader<bool>;
#[doc = "Field `DOIE` writer - Data overrun interrupt enable"]
pub type DOIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_P_SPEC, bool, O>;
#[doc = "Field `EPIE` reader - Error passive interrupt enable"]
pub type EPIE_R = crate::BitReader<bool>;
#[doc = "Field `EPIE` writer - Error passive interrupt enable"]
pub type EPIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_P_SPEC, bool, O>;
#[doc = "Field `ALIE` reader - Arbitration lost interrupt enable"]
pub type ALIE_R = crate::BitReader<bool>;
#[doc = "Field `ALIE` writer - Arbitration lost interrupt enable"]
pub type ALIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_P_SPEC, bool, O>;
#[doc = "Field `BEIE` reader - Bus error interrupt enable"]
pub type BEIE_R = crate::BitReader<bool>;
#[doc = "Field `BEIE` writer - Bus error interrupt enable"]
pub type BEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_P_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Receive interrupt enable"]
    #[inline(always)]
    pub fn rie(&self) -> RIE_R {
        RIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit interrupt enable"]
    #[inline(always)]
    pub fn tie(&self) -> TIE_R {
        TIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Error interrupt enable"]
    #[inline(always)]
    pub fn eie(&self) -> EIE_R {
        EIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Data overrun interrupt enable"]
    #[inline(always)]
    pub fn doie(&self) -> DOIE_R {
        DOIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - Error passive interrupt enable"]
    #[inline(always)]
    pub fn epie(&self) -> EPIE_R {
        EPIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Arbitration lost interrupt enable"]
    #[inline(always)]
    pub fn alie(&self) -> ALIE_R {
        ALIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Bus error interrupt enable"]
    #[inline(always)]
    pub fn beie(&self) -> BEIE_R {
        BEIE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Receive interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn rie(&mut self) -> RIE_W<0> {
        RIE_W::new(self)
    }
    #[doc = "Bit 1 - Transmit interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn tie(&mut self) -> TIE_W<1> {
        TIE_W::new(self)
    }
    #[doc = "Bit 2 - Error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn eie(&mut self) -> EIE_W<2> {
        EIE_W::new(self)
    }
    #[doc = "Bit 3 - Data overrun interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn doie(&mut self) -> DOIE_W<3> {
        DOIE_W::new(self)
    }
    #[doc = "Bit 5 - Error passive interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn epie(&mut self) -> EPIE_W<5> {
        EPIE_W::new(self)
    }
    #[doc = "Bit 6 - Arbitration lost interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn alie(&mut self) -> ALIE_W<6> {
        ALIE_W::new(self)
    }
    #[doc = "Bit 7 - Bus error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn beie(&mut self) -> BEIE_W<7> {
        BEIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Peli Interrupt Enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ier_p](index.html) module"]
pub struct IER_P_SPEC;
impl crate::RegisterSpec for IER_P_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ier_p::R](R) reader structure"]
impl crate::Readable for IER_P_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ier_p::W](W) writer structure"]
impl crate::Writable for IER_P_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IER_P to value 0"]
impl crate::Resettable for IER_P_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
