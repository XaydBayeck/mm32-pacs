#[doc = "Register `IRDA` reader"]
pub struct R(crate::R<IRDA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IRDA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IRDA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IRDA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IRDA` writer"]
pub struct W(crate::W<IRDA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IRDA_SPEC>;
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
impl From<crate::W<IRDA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IRDA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SIREN` reader - IrDA mode enable"]
pub type SIREN_R = crate::BitReader<bool>;
#[doc = "Field `SIREN` writer - IrDA mode enable"]
pub type SIREN_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRDA_SPEC, bool, O>;
#[doc = "Field `SIRLP` reader - IrDA low-power"]
pub type SIRLP_R = crate::BitReader<bool>;
#[doc = "Field `SIRLP` writer - IrDA low-power"]
pub type SIRLP_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRDA_SPEC, bool, O>;
#[doc = "Field `PSC_REG` reader - *D4"]
pub type PSC_REG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PSC_REG` writer - *D4"]
pub type PSC_REG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IRDA_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bit 0 - IrDA mode enable"]
    #[inline(always)]
    pub fn siren(&self) -> SIREN_R {
        SIREN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - IrDA low-power"]
    #[inline(always)]
    pub fn sirlp(&self) -> SIRLP_R {
        SIRLP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 8:15 - *D4"]
    #[inline(always)]
    pub fn psc_reg(&self) -> PSC_REG_R {
        PSC_REG_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - IrDA mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn siren(&mut self) -> SIREN_W<0> {
        SIREN_W::new(self)
    }
    #[doc = "Bit 1 - IrDA low-power"]
    #[inline(always)]
    #[must_use]
    pub fn sirlp(&mut self) -> SIRLP_W<1> {
        SIRLP_W::new(self)
    }
    #[doc = "Bits 8:15 - *D4"]
    #[inline(always)]
    #[must_use]
    pub fn psc_reg(&mut self) -> PSC_REG_W<8> {
        PSC_REG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IrDA configure register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irda](index.html) module"]
pub struct IRDA_SPEC;
impl crate::RegisterSpec for IRDA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [irda::R](R) reader structure"]
impl crate::Readable for IRDA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [irda::W](W) writer structure"]
impl crate::Writable for IRDA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IRDA to value 0x0100"]
impl crate::Resettable for IRDA_SPEC {
    const RESET_VALUE: Self::Ux = 0x0100;
}
