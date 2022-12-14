#[doc = "Register `DMARXPDR` reader"]
pub struct R(crate::R<DMARXPDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMARXPDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMARXPDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMARXPDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMARXPDR` writer"]
pub struct W(crate::W<DMARXPDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMARXPDR_SPEC>;
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
impl From<crate::W<DMARXPDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMARXPDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXPD` reader - Receive poll demand"]
pub type RXPD_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RXPD` writer - Receive poll demand"]
pub type RXPD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DMARXPDR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Receive poll demand"]
    #[inline(always)]
    pub fn rxpd(&self) -> RXPD_R {
        RXPD_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Receive poll demand"]
    #[inline(always)]
    #[must_use]
    pub fn rxpd(&mut self) -> RXPD_W<0> {
        RXPD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EHERNET DMA receive poll demand register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmarxpdr](index.html) module"]
pub struct DMARXPDR_SPEC;
impl crate::RegisterSpec for DMARXPDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmarxpdr::R](R) reader structure"]
impl crate::Readable for DMARXPDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmarxpdr::W](W) writer structure"]
impl crate::Writable for DMARXPDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMARXPDR to value 0"]
impl crate::Resettable for DMARXPDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
