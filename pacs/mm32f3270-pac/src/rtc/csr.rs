#[doc = "Register `CSR` reader"]
pub struct R(crate::R<CSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CSR` writer"]
pub struct W(crate::W<CSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSR_SPEC>;
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
impl From<crate::W<CSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SECF` reader - Second flag"]
pub type SECF_R = crate::BitReader<bool>;
#[doc = "Field `SECF` writer - Second flag"]
pub type SECF_W<'a, const O: u8> = crate::BitWriter0C<'a, u16, CSR_SPEC, bool, O>;
#[doc = "Field `ALRF` reader - Alarm flag"]
pub type ALRF_R = crate::BitReader<bool>;
#[doc = "Field `ALRF` writer - Alarm flag"]
pub type ALRF_W<'a, const O: u8> = crate::BitWriter0C<'a, u16, CSR_SPEC, bool, O>;
#[doc = "Field `OWF` reader - Overflow flag"]
pub type OWF_R = crate::BitReader<bool>;
#[doc = "Field `OWF` writer - Overflow flag"]
pub type OWF_W<'a, const O: u8> = crate::BitWriter0C<'a, u16, CSR_SPEC, bool, O>;
#[doc = "Field `RSF` reader - Registers synchronized flag"]
pub type RSF_R = crate::BitReader<bool>;
#[doc = "Field `RSF` writer - Registers synchronized flag"]
pub type RSF_W<'a, const O: u8> = crate::BitWriter0C<'a, u16, CSR_SPEC, bool, O>;
#[doc = "Field `CNF` reader - Configuration flag"]
pub type CNF_R = crate::BitReader<bool>;
#[doc = "Field `CNF` writer - Configuration flag"]
pub type CNF_W<'a, const O: u8> = crate::BitWriter<'a, u16, CSR_SPEC, bool, O>;
#[doc = "Field `RTOFF` reader - RTC operation OFF"]
pub type RTOFF_R = crate::BitReader<bool>;
#[doc = "Field `ALPEN` reader - RTC alarm loop enable"]
pub type ALPEN_R = crate::BitReader<bool>;
#[doc = "Field `ALPEN` writer - RTC alarm loop enable"]
pub type ALPEN_W<'a, const O: u8> = crate::BitWriter<'a, u16, CSR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Second flag"]
    #[inline(always)]
    pub fn secf(&self) -> SECF_R {
        SECF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Alarm flag"]
    #[inline(always)]
    pub fn alrf(&self) -> ALRF_R {
        ALRF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Overflow flag"]
    #[inline(always)]
    pub fn owf(&self) -> OWF_R {
        OWF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Registers synchronized flag"]
    #[inline(always)]
    pub fn rsf(&self) -> RSF_R {
        RSF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Configuration flag"]
    #[inline(always)]
    pub fn cnf(&self) -> CNF_R {
        CNF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RTC operation OFF"]
    #[inline(always)]
    pub fn rtoff(&self) -> RTOFF_R {
        RTOFF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - RTC alarm loop enable"]
    #[inline(always)]
    pub fn alpen(&self) -> ALPEN_R {
        ALPEN_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Second flag"]
    #[inline(always)]
    #[must_use]
    pub fn secf(&mut self) -> SECF_W<0> {
        SECF_W::new(self)
    }
    #[doc = "Bit 1 - Alarm flag"]
    #[inline(always)]
    #[must_use]
    pub fn alrf(&mut self) -> ALRF_W<1> {
        ALRF_W::new(self)
    }
    #[doc = "Bit 2 - Overflow flag"]
    #[inline(always)]
    #[must_use]
    pub fn owf(&mut self) -> OWF_W<2> {
        OWF_W::new(self)
    }
    #[doc = "Bit 3 - Registers synchronized flag"]
    #[inline(always)]
    #[must_use]
    pub fn rsf(&mut self) -> RSF_W<3> {
        RSF_W::new(self)
    }
    #[doc = "Bit 4 - Configuration flag"]
    #[inline(always)]
    #[must_use]
    pub fn cnf(&mut self) -> CNF_W<4> {
        CNF_W::new(self)
    }
    #[doc = "Bit 6 - RTC alarm loop enable"]
    #[inline(always)]
    #[must_use]
    pub fn alpen(&mut self) -> ALPEN_W<6> {
        ALPEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CSR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csr](index.html) module"]
pub struct CSR_SPEC;
impl crate::RegisterSpec for CSR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [csr::R](R) reader structure"]
impl crate::Readable for CSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [csr::W](W) writer structure"]
impl crate::Writable for CSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x0f;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CSR to value 0x20"]
impl crate::Resettable for CSR_SPEC {
    const RESET_VALUE: Self::Ux = 0x20;
}
