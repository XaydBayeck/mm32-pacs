#[doc = "Register `CIR` reader"]
pub struct R(crate::R<CIR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CIR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CIR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CIR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CIR` writer"]
pub struct W(crate::W<CIR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CIR_SPEC>;
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
impl From<crate::W<CIR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CIR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LSIRDYF` reader - LSI ready interrupt flag"]
pub type LSIRDYF_R = crate::BitReader<bool>;
#[doc = "Field `LSIRDYF` writer - LSI ready interrupt flag"]
pub type LSIRDYF_W<'a, const O: u8> = crate::BitWriter<'a, u32, CIR_SPEC, bool, O>;
#[doc = "Field `LSERDYF` reader - *D1"]
pub type LSERDYF_R = crate::BitReader<bool>;
#[doc = "Field `LSERDYF` writer - *D1"]
pub type LSERDYF_W<'a, const O: u8> = crate::BitWriter<'a, u32, CIR_SPEC, bool, O>;
#[doc = "Field `HSIRDYF` reader - HSI ready interrupt flag"]
pub type HSIRDYF_R = crate::BitReader<bool>;
#[doc = "Field `HSIRDYF` writer - HSI ready interrupt flag"]
pub type HSIRDYF_W<'a, const O: u8> = crate::BitWriter<'a, u32, CIR_SPEC, bool, O>;
#[doc = "Field `HSERDYF` reader - HSE ready interrupt flag"]
pub type HSERDYF_R = crate::BitReader<bool>;
#[doc = "Field `HSERDYF` writer - HSE ready interrupt flag"]
pub type HSERDYF_W<'a, const O: u8> = crate::BitWriter<'a, u32, CIR_SPEC, bool, O>;
#[doc = "Field `PLLRDYF` reader - *D4"]
pub type PLLRDYF_R = crate::BitReader<bool>;
#[doc = "Field `PLLRDYF` writer - *D4"]
pub type PLLRDYF_W<'a, const O: u8> = crate::BitWriter<'a, u32, CIR_SPEC, bool, O>;
#[doc = "Field `CSSF` reader - Clock security system interrupt flag"]
pub type CSSF_R = crate::BitReader<bool>;
#[doc = "Field `CSSF` writer - Clock security system interrupt flag"]
pub type CSSF_W<'a, const O: u8> = crate::BitWriter<'a, u32, CIR_SPEC, bool, O>;
#[doc = "Field `LSIRDYIE` reader - LSI ready interrupt enable"]
pub type LSIRDYIE_R = crate::BitReader<bool>;
#[doc = "Field `LSIRDYIE` writer - LSI ready interrupt enable"]
pub type LSIRDYIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CIR_SPEC, bool, O>;
#[doc = "Field `LSERDYFIE` reader - *D9"]
pub type LSERDYFIE_R = crate::BitReader<bool>;
#[doc = "Field `LSERDYFIE` writer - *D9"]
pub type LSERDYFIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CIR_SPEC, bool, O>;
#[doc = "Field `HSIRDYIE` reader - HSI ready interrupt enable"]
pub type HSIRDYIE_R = crate::BitReader<bool>;
#[doc = "Field `HSIRDYIE` writer - HSI ready interrupt enable"]
pub type HSIRDYIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CIR_SPEC, bool, O>;
#[doc = "Field `HSERDYIE` reader - HSE ready interrupt enable"]
pub type HSERDYIE_R = crate::BitReader<bool>;
#[doc = "Field `HSERDYIE` writer - HSE ready interrupt enable"]
pub type HSERDYIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CIR_SPEC, bool, O>;
#[doc = "Field `PLLRDYIE` reader - *D12"]
pub type PLLRDYIE_R = crate::BitReader<bool>;
#[doc = "Field `PLLRDYIE` writer - *D12"]
pub type PLLRDYIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CIR_SPEC, bool, O>;
#[doc = "Field `LSIRDYC` reader - LSI ready interrupt clear"]
pub type LSIRDYC_R = crate::BitReader<bool>;
#[doc = "Field `LSIRDYC` writer - LSI ready interrupt clear"]
pub type LSIRDYC_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, CIR_SPEC, bool, O>;
#[doc = "Field `LSIRDYCF` reader - *D17"]
pub type LSIRDYCF_R = crate::BitReader<bool>;
#[doc = "Field `LSIRDYCF` writer - *D17"]
pub type LSIRDYCF_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, CIR_SPEC, bool, O>;
#[doc = "Field `HSIRDYC` reader - HSI ready interrupt clear"]
pub type HSIRDYC_R = crate::BitReader<bool>;
#[doc = "Field `HSIRDYC` writer - HSI ready interrupt clear"]
pub type HSIRDYC_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, CIR_SPEC, bool, O>;
#[doc = "Field `HSERDYC` reader - HSE ready interrupt clear"]
pub type HSERDYC_R = crate::BitReader<bool>;
#[doc = "Field `HSERDYC` writer - HSE ready interrupt clear"]
pub type HSERDYC_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, CIR_SPEC, bool, O>;
#[doc = "Field `PLLRDYC` reader - *D20"]
pub type PLLRDYC_R = crate::BitReader<bool>;
#[doc = "Field `PLLRDYC` writer - *D20"]
pub type PLLRDYC_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, CIR_SPEC, bool, O>;
#[doc = "Field `CSSC` reader - Clock security system interrupt clear"]
pub type CSSC_R = crate::BitReader<bool>;
#[doc = "Field `CSSC` writer - Clock security system interrupt clear"]
pub type CSSC_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, CIR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - LSI ready interrupt flag"]
    #[inline(always)]
    pub fn lsirdyf(&self) -> LSIRDYF_R {
        LSIRDYF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - *D1"]
    #[inline(always)]
    pub fn lserdyf(&self) -> LSERDYF_R {
        LSERDYF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - HSI ready interrupt flag"]
    #[inline(always)]
    pub fn hsirdyf(&self) -> HSIRDYF_R {
        HSIRDYF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - HSE ready interrupt flag"]
    #[inline(always)]
    pub fn hserdyf(&self) -> HSERDYF_R {
        HSERDYF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - *D4"]
    #[inline(always)]
    pub fn pllrdyf(&self) -> PLLRDYF_R {
        PLLRDYF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - Clock security system interrupt flag"]
    #[inline(always)]
    pub fn cssf(&self) -> CSSF_R {
        CSSF_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - LSI ready interrupt enable"]
    #[inline(always)]
    pub fn lsirdyie(&self) -> LSIRDYIE_R {
        LSIRDYIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - *D9"]
    #[inline(always)]
    pub fn lserdyfie(&self) -> LSERDYFIE_R {
        LSERDYFIE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - HSI ready interrupt enable"]
    #[inline(always)]
    pub fn hsirdyie(&self) -> HSIRDYIE_R {
        HSIRDYIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - HSE ready interrupt enable"]
    #[inline(always)]
    pub fn hserdyie(&self) -> HSERDYIE_R {
        HSERDYIE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - *D12"]
    #[inline(always)]
    pub fn pllrdyie(&self) -> PLLRDYIE_R {
        PLLRDYIE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - LSI ready interrupt clear"]
    #[inline(always)]
    pub fn lsirdyc(&self) -> LSIRDYC_R {
        LSIRDYC_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - *D17"]
    #[inline(always)]
    pub fn lsirdycf(&self) -> LSIRDYCF_R {
        LSIRDYCF_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - HSI ready interrupt clear"]
    #[inline(always)]
    pub fn hsirdyc(&self) -> HSIRDYC_R {
        HSIRDYC_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - HSE ready interrupt clear"]
    #[inline(always)]
    pub fn hserdyc(&self) -> HSERDYC_R {
        HSERDYC_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - *D20"]
    #[inline(always)]
    pub fn pllrdyc(&self) -> PLLRDYC_R {
        PLLRDYC_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 23 - Clock security system interrupt clear"]
    #[inline(always)]
    pub fn cssc(&self) -> CSSC_R {
        CSSC_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LSI ready interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn lsirdyf(&mut self) -> LSIRDYF_W<0> {
        LSIRDYF_W::new(self)
    }
    #[doc = "Bit 1 - *D1"]
    #[inline(always)]
    #[must_use]
    pub fn lserdyf(&mut self) -> LSERDYF_W<1> {
        LSERDYF_W::new(self)
    }
    #[doc = "Bit 2 - HSI ready interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn hsirdyf(&mut self) -> HSIRDYF_W<2> {
        HSIRDYF_W::new(self)
    }
    #[doc = "Bit 3 - HSE ready interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn hserdyf(&mut self) -> HSERDYF_W<3> {
        HSERDYF_W::new(self)
    }
    #[doc = "Bit 4 - *D4"]
    #[inline(always)]
    #[must_use]
    pub fn pllrdyf(&mut self) -> PLLRDYF_W<4> {
        PLLRDYF_W::new(self)
    }
    #[doc = "Bit 7 - Clock security system interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn cssf(&mut self) -> CSSF_W<7> {
        CSSF_W::new(self)
    }
    #[doc = "Bit 8 - LSI ready interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn lsirdyie(&mut self) -> LSIRDYIE_W<8> {
        LSIRDYIE_W::new(self)
    }
    #[doc = "Bit 9 - *D9"]
    #[inline(always)]
    #[must_use]
    pub fn lserdyfie(&mut self) -> LSERDYFIE_W<9> {
        LSERDYFIE_W::new(self)
    }
    #[doc = "Bit 10 - HSI ready interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn hsirdyie(&mut self) -> HSIRDYIE_W<10> {
        HSIRDYIE_W::new(self)
    }
    #[doc = "Bit 11 - HSE ready interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn hserdyie(&mut self) -> HSERDYIE_W<11> {
        HSERDYIE_W::new(self)
    }
    #[doc = "Bit 12 - *D12"]
    #[inline(always)]
    #[must_use]
    pub fn pllrdyie(&mut self) -> PLLRDYIE_W<12> {
        PLLRDYIE_W::new(self)
    }
    #[doc = "Bit 16 - LSI ready interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn lsirdyc(&mut self) -> LSIRDYC_W<16> {
        LSIRDYC_W::new(self)
    }
    #[doc = "Bit 17 - *D17"]
    #[inline(always)]
    #[must_use]
    pub fn lsirdycf(&mut self) -> LSIRDYCF_W<17> {
        LSIRDYCF_W::new(self)
    }
    #[doc = "Bit 18 - HSI ready interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn hsirdyc(&mut self) -> HSIRDYC_W<18> {
        HSIRDYC_W::new(self)
    }
    #[doc = "Bit 19 - HSE ready interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn hserdyc(&mut self) -> HSERDYC_W<19> {
        HSERDYC_W::new(self)
    }
    #[doc = "Bit 20 - *D20"]
    #[inline(always)]
    #[must_use]
    pub fn pllrdyc(&mut self) -> PLLRDYC_W<20> {
        PLLRDYC_W::new(self)
    }
    #[doc = "Bit 23 - Clock security system interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn cssc(&mut self) -> CSSC_W<23> {
        CSSC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock Interrupt Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cir](index.html) module"]
pub struct CIR_SPEC;
impl crate::RegisterSpec for CIR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cir::R](R) reader structure"]
impl crate::Readable for CIR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cir::W](W) writer structure"]
impl crate::Writable for CIR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x009f_0000;
}
#[doc = "`reset()` method sets CIR to value 0"]
impl crate::Resettable for CIR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
