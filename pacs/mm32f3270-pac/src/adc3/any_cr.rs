#[doc = "Register `ANY_CR` reader"]
pub struct R(crate::R<ANY_CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ANY_CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ANY_CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ANY_CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ANY_CR` writer"]
pub struct W(crate::W<ANY_CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ANY_CR_SPEC>;
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
impl From<crate::W<ANY_CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ANY_CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CHANY_MDEN` reader - Any channel configuration mode enable bit"]
pub type CHANY_MDEN_R = crate::BitReader<bool>;
#[doc = "Field `CHANY_MDEN` writer - Any channel configuration mode enable bit"]
pub type CHANY_MDEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ANY_CR_SPEC, bool, O>;
#[doc = "Field `JCEN` reader - Injected channel enable"]
pub type JCEN_R = crate::BitReader<bool>;
#[doc = "Field `JCEN` writer - Injected channel enable"]
pub type JCEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ANY_CR_SPEC, bool, O>;
#[doc = "Field `JEOSMPIE` reader - Interrupt enable the end of sequence conversion for injected channel"]
pub type JEOSMPIE_R = crate::BitReader<bool>;
#[doc = "Field `JEOSMPIE` writer - Interrupt enable the end of sequence conversion for injected channel"]
pub type JEOSMPIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, ANY_CR_SPEC, bool, O>;
#[doc = "Field `JEOCIE` reader - Interrupt enable the end of conversion for injected channel"]
pub type JEOCIE_R = crate::BitReader<bool>;
#[doc = "Field `JEOCIE` writer - Interrupt enable the end of conversion for injected channel"]
pub type JEOCIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, ANY_CR_SPEC, bool, O>;
#[doc = "Field `JEOSIE` reader - Interrupt enable the end of sequence conversion for injected channel"]
pub type JEOSIE_R = crate::BitReader<bool>;
#[doc = "Field `JEOSIE` writer - Interrupt enable the end of sequence conversion for injected channel"]
pub type JEOSIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, ANY_CR_SPEC, bool, O>;
#[doc = "Field `JAUTO` reader - Automatic Injected group conversion"]
pub type JAUTO_R = crate::BitReader<bool>;
#[doc = "Field `JAUTO` writer - Automatic Injected group conversion"]
pub type JAUTO_W<'a, const O: u8> = crate::BitWriter<'a, u32, ANY_CR_SPEC, bool, O>;
#[doc = "Field `JADST` reader - Start conversion of injected channels"]
pub type JADST_R = crate::BitReader<bool>;
#[doc = "Field `JADST` writer - Start conversion of injected channels"]
pub type JADST_W<'a, const O: u8> = crate::BitWriter<'a, u32, ANY_CR_SPEC, bool, O>;
#[doc = "Field `JTRGEN` reader - External trigger conversion mode for injected channels"]
pub type JTRGEN_R = crate::BitReader<bool>;
#[doc = "Field `JTRGEN` writer - External trigger conversion mode for injected channels"]
pub type JTRGEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ANY_CR_SPEC, bool, O>;
#[doc = "Field `JTRGSEL` reader - External event select for in-jected group"]
pub type JTRGSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `JTRGSEL` writer - External event select for in-jected group"]
pub type JTRGSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ANY_CR_SPEC, u8, u8, 3, O>;
#[doc = "Field `JTRGSHIFT` reader - Injection mode external trigger delay sampling"]
pub type JTRGSHIFT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `JTRGSHIFT` writer - Injection mode external trigger delay sampling"]
pub type JTRGSHIFT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ANY_CR_SPEC, u8, u8, 3, O>;
#[doc = "Field `JTRGEDGE` reader - Injection mode trigger edge selection"]
pub type JTRGEDGE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `JTRGEDGE` writer - Injection mode trigger edge selection"]
pub type JTRGEDGE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ANY_CR_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bit 0 - Any channel configuration mode enable bit"]
    #[inline(always)]
    pub fn chany_mden(&self) -> CHANY_MDEN_R {
        CHANY_MDEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Injected channel enable"]
    #[inline(always)]
    pub fn jcen(&self) -> JCEN_R {
        JCEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt enable the end of sequence conversion for injected channel"]
    #[inline(always)]
    pub fn jeosmpie(&self) -> JEOSMPIE_R {
        JEOSMPIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt enable the end of conversion for injected channel"]
    #[inline(always)]
    pub fn jeocie(&self) -> JEOCIE_R {
        JEOCIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Interrupt enable the end of sequence conversion for injected channel"]
    #[inline(always)]
    pub fn jeosie(&self) -> JEOSIE_R {
        JEOSIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Automatic Injected group conversion"]
    #[inline(always)]
    pub fn jauto(&self) -> JAUTO_R {
        JAUTO_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Start conversion of injected channels"]
    #[inline(always)]
    pub fn jadst(&self) -> JADST_R {
        JADST_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - External trigger conversion mode for injected channels"]
    #[inline(always)]
    pub fn jtrgen(&self) -> JTRGEN_R {
        JTRGEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - External event select for in-jected group"]
    #[inline(always)]
    pub fn jtrgsel(&self) -> JTRGSEL_R {
        JTRGSEL_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 13:15 - Injection mode external trigger delay sampling"]
    #[inline(always)]
    pub fn jtrgshift(&self) -> JTRGSHIFT_R {
        JTRGSHIFT_R::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bits 16:17 - Injection mode trigger edge selection"]
    #[inline(always)]
    pub fn jtrgedge(&self) -> JTRGEDGE_R {
        JTRGEDGE_R::new(((self.bits >> 16) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Any channel configuration mode enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn chany_mden(&mut self) -> CHANY_MDEN_W<0> {
        CHANY_MDEN_W::new(self)
    }
    #[doc = "Bit 1 - Injected channel enable"]
    #[inline(always)]
    #[must_use]
    pub fn jcen(&mut self) -> JCEN_W<1> {
        JCEN_W::new(self)
    }
    #[doc = "Bit 2 - Interrupt enable the end of sequence conversion for injected channel"]
    #[inline(always)]
    #[must_use]
    pub fn jeosmpie(&mut self) -> JEOSMPIE_W<2> {
        JEOSMPIE_W::new(self)
    }
    #[doc = "Bit 3 - Interrupt enable the end of conversion for injected channel"]
    #[inline(always)]
    #[must_use]
    pub fn jeocie(&mut self) -> JEOCIE_W<3> {
        JEOCIE_W::new(self)
    }
    #[doc = "Bit 4 - Interrupt enable the end of sequence conversion for injected channel"]
    #[inline(always)]
    #[must_use]
    pub fn jeosie(&mut self) -> JEOSIE_W<4> {
        JEOSIE_W::new(self)
    }
    #[doc = "Bit 5 - Automatic Injected group conversion"]
    #[inline(always)]
    #[must_use]
    pub fn jauto(&mut self) -> JAUTO_W<5> {
        JAUTO_W::new(self)
    }
    #[doc = "Bit 6 - Start conversion of injected channels"]
    #[inline(always)]
    #[must_use]
    pub fn jadst(&mut self) -> JADST_W<6> {
        JADST_W::new(self)
    }
    #[doc = "Bit 7 - External trigger conversion mode for injected channels"]
    #[inline(always)]
    #[must_use]
    pub fn jtrgen(&mut self) -> JTRGEN_W<7> {
        JTRGEN_W::new(self)
    }
    #[doc = "Bits 8:10 - External event select for in-jected group"]
    #[inline(always)]
    #[must_use]
    pub fn jtrgsel(&mut self) -> JTRGSEL_W<8> {
        JTRGSEL_W::new(self)
    }
    #[doc = "Bits 13:15 - Injection mode external trigger delay sampling"]
    #[inline(always)]
    #[must_use]
    pub fn jtrgshift(&mut self) -> JTRGSHIFT_W<13> {
        JTRGSHIFT_W::new(self)
    }
    #[doc = "Bits 16:17 - Injection mode trigger edge selection"]
    #[inline(always)]
    #[must_use]
    pub fn jtrgedge(&mut self) -> JTRGEDGE_W<16> {
        JTRGEDGE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Arbitrary channel control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [any_cr](index.html) module"]
pub struct ANY_CR_SPEC;
impl crate::RegisterSpec for ANY_CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [any_cr::R](R) reader structure"]
impl crate::Readable for ANY_CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [any_cr::W](W) writer structure"]
impl crate::Writable for ANY_CR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ANY_CR to value 0"]
impl crate::Resettable for ANY_CR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
