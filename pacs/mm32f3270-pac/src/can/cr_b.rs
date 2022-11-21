#[doc = "Register `CR_B` reader"]
pub struct R(crate::R<CR_B_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR_B_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR_B_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR_B_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR_B` writer"]
pub struct W(crate::W<CR_B_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR_B_SPEC>;
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
impl From<crate::W<CR_B_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR_B_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RR` reader - Reset request"]
pub type RR_R = crate::BitReader<bool>;
#[doc = "Field `RR` writer - Reset request"]
pub type RR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_B_SPEC, bool, O>;
#[doc = "Field `RIE` reader - Receive interrupt enable"]
pub type RIE_R = crate::BitReader<bool>;
#[doc = "Field `RIE` writer - Receive interrupt enable"]
pub type RIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_B_SPEC, bool, O>;
#[doc = "Field `TIE` reader - Transmit interrupt enable"]
pub type TIE_R = crate::BitReader<bool>;
#[doc = "Field `TIE` writer - Transmit interrupt enable"]
pub type TIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_B_SPEC, bool, O>;
#[doc = "Field `EIE` reader - Error interrupt enable"]
pub type EIE_R = crate::BitReader<bool>;
#[doc = "Field `EIE` writer - Error interrupt enable"]
pub type EIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_B_SPEC, bool, O>;
#[doc = "Field `OIE` reader - Overflow interrupt enable"]
pub type OIE_R = crate::BitReader<bool>;
#[doc = "Field `OIE` writer - Overflow interrupt enable"]
pub type OIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_B_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Reset request"]
    #[inline(always)]
    pub fn rr(&self) -> RR_R {
        RR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Receive interrupt enable"]
    #[inline(always)]
    pub fn rie(&self) -> RIE_R {
        RIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmit interrupt enable"]
    #[inline(always)]
    pub fn tie(&self) -> TIE_R {
        TIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Error interrupt enable"]
    #[inline(always)]
    pub fn eie(&self) -> EIE_R {
        EIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Overflow interrupt enable"]
    #[inline(always)]
    pub fn oie(&self) -> OIE_R {
        OIE_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Reset request"]
    #[inline(always)]
    #[must_use]
    pub fn rr(&mut self) -> RR_W<0> {
        RR_W::new(self)
    }
    #[doc = "Bit 1 - Receive interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn rie(&mut self) -> RIE_W<1> {
        RIE_W::new(self)
    }
    #[doc = "Bit 2 - Transmit interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn tie(&mut self) -> TIE_W<2> {
        TIE_W::new(self)
    }
    #[doc = "Bit 3 - Error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn eie(&mut self) -> EIE_W<3> {
        EIE_W::new(self)
    }
    #[doc = "Bit 4 - Overflow interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn oie(&mut self) -> OIE_W<4> {
        OIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Basic control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr_b](index.html) module"]
pub struct CR_B_SPEC;
impl crate::RegisterSpec for CR_B_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr_b::R](R) reader structure"]
impl crate::Readable for CR_B_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr_b::W](W) writer structure"]
impl crate::Writable for CR_B_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CR_B to value 0x21"]
impl crate::Resettable for CR_B_SPEC {
    const RESET_VALUE: Self::Ux = 0x21;
}
