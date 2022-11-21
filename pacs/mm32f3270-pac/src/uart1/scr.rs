#[doc = "Register `SCR` reader"]
pub struct R(crate::R<SCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCR` writer"]
pub struct W(crate::W<SCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCR_SPEC>;
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
impl From<crate::W<SCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SCEN` reader - ISO7816 enable control bit"]
pub type SCEN_R = crate::BitReader<bool>;
#[doc = "Field `SCEN` writer - ISO7816 enable control bit"]
pub type SCEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCR_SPEC, bool, O>;
#[doc = "Field `SCAEN` reader - ISO7816 check auto-response bit"]
pub type SCAEN_R = crate::BitReader<bool>;
#[doc = "Field `SCAEN` writer - ISO7816 check auto-response bit"]
pub type SCAEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCR_SPEC, bool, O>;
#[doc = "Field `NACK` reader - Master receive frame answer bit"]
pub type NACK_R = crate::BitReader<bool>;
#[doc = "Field `SCFCNT` reader - ISO7816 protection counter bit"]
pub type SCFCNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SCFCNT` writer - ISO7816 protection counter bit"]
pub type SCFCNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SCR_SPEC, u8, u8, 8, O>;
#[doc = "Field `HDSEL` reader - Single-wire half-duplex mode selection bit"]
pub type HDSEL_R = crate::BitReader<bool>;
#[doc = "Field `HDSEL` writer - Single-wire half-duplex mode selection bit"]
pub type HDSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - ISO7816 enable control bit"]
    #[inline(always)]
    pub fn scen(&self) -> SCEN_R {
        SCEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ISO7816 check auto-response bit"]
    #[inline(always)]
    pub fn scaen(&self) -> SCAEN_R {
        SCAEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Master receive frame answer bit"]
    #[inline(always)]
    pub fn nack(&self) -> NACK_R {
        NACK_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:11 - ISO7816 protection counter bit"]
    #[inline(always)]
    pub fn scfcnt(&self) -> SCFCNT_R {
        SCFCNT_R::new(((self.bits >> 4) & 0xff) as u8)
    }
    #[doc = "Bit 12 - Single-wire half-duplex mode selection bit"]
    #[inline(always)]
    pub fn hdsel(&self) -> HDSEL_R {
        HDSEL_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ISO7816 enable control bit"]
    #[inline(always)]
    #[must_use]
    pub fn scen(&mut self) -> SCEN_W<0> {
        SCEN_W::new(self)
    }
    #[doc = "Bit 1 - ISO7816 check auto-response bit"]
    #[inline(always)]
    #[must_use]
    pub fn scaen(&mut self) -> SCAEN_W<1> {
        SCAEN_W::new(self)
    }
    #[doc = "Bits 4:11 - ISO7816 protection counter bit"]
    #[inline(always)]
    #[must_use]
    pub fn scfcnt(&mut self) -> SCFCNT_W<4> {
        SCFCNT_W::new(self)
    }
    #[doc = "Bit 12 - Single-wire half-duplex mode selection bit"]
    #[inline(always)]
    #[must_use]
    pub fn hdsel(&mut self) -> HDSEL_W<12> {
        HDSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ISO7816 configure register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scr](index.html) module"]
pub struct SCR_SPEC;
impl crate::RegisterSpec for SCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [scr::R](R) reader structure"]
impl crate::Readable for SCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [scr::W](W) writer structure"]
impl crate::Writable for SCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCR to value 0"]
impl crate::Resettable for SCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
