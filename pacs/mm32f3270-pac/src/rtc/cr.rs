#[doc = "Register `CR` reader"]
pub struct R(crate::R<CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR` writer"]
pub struct W(crate::W<CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR_SPEC>;
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
impl From<crate::W<CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SECIE` reader - HSI delay time"]
pub type SECIE_R = crate::BitReader<bool>;
#[doc = "Field `SECIE` writer - HSI delay time"]
pub type SECIE_W<'a, const O: u8> = crate::BitWriter<'a, u16, CR_SPEC, bool, O>;
#[doc = "Field `ALRIE` reader - Alarm interrupt enable"]
pub type ALRIE_R = crate::BitReader<bool>;
#[doc = "Field `ALRIE` writer - Alarm interrupt enable"]
pub type ALRIE_W<'a, const O: u8> = crate::BitWriter<'a, u16, CR_SPEC, bool, O>;
#[doc = "Field `OWIE` reader - Overflow interrupt enable"]
pub type OWIE_R = crate::BitReader<bool>;
#[doc = "Field `OWIE` writer - Overflow interrupt enable"]
pub type OWIE_W<'a, const O: u8> = crate::BitWriter<'a, u16, CR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - HSI delay time"]
    #[inline(always)]
    pub fn secie(&self) -> SECIE_R {
        SECIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Alarm interrupt enable"]
    #[inline(always)]
    pub fn alrie(&self) -> ALRIE_R {
        ALRIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Overflow interrupt enable"]
    #[inline(always)]
    pub fn owie(&self) -> OWIE_R {
        OWIE_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - HSI delay time"]
    #[inline(always)]
    #[must_use]
    pub fn secie(&mut self) -> SECIE_W<0> {
        SECIE_W::new(self)
    }
    #[doc = "Bit 1 - Alarm interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn alrie(&mut self) -> ALRIE_W<1> {
        ALRIE_W::new(self)
    }
    #[doc = "Bit 2 - Overflow interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn owie(&mut self) -> OWIE_W<2> {
        OWIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](index.html) module"]
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [cr::R](R) reader structure"]
impl crate::Readable for CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr::W](W) writer structure"]
impl crate::Writable for CR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
