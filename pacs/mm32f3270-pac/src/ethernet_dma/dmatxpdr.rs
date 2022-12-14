#[doc = "Register `DMATXPDR` reader"]
pub struct R(crate::R<DMATXPDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMATXPDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMATXPDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMATXPDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMATXPDR` writer"]
pub struct W(crate::W<DMATXPDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMATXPDR_SPEC>;
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
impl From<crate::W<DMATXPDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMATXPDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXPD` reader - Transmit poll demand"]
pub type TXPD_R = crate::FieldReader<u32, u32>;
#[doc = "Field `TXPD` writer - Transmit poll demand"]
pub type TXPD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DMATXPDR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Transmit poll demand"]
    #[inline(always)]
    pub fn txpd(&self) -> TXPD_R {
        TXPD_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Transmit poll demand"]
    #[inline(always)]
    #[must_use]
    pub fn txpd(&mut self) -> TXPD_W<0> {
        TXPD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ethernet DMA transmit poll demand register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmatxpdr](index.html) module"]
pub struct DMATXPDR_SPEC;
impl crate::RegisterSpec for DMATXPDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmatxpdr::R](R) reader structure"]
impl crate::Readable for DMATXPDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmatxpdr::W](W) writer structure"]
impl crate::Writable for DMATXPDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMATXPDR to value 0"]
impl crate::Resettable for DMATXPDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
