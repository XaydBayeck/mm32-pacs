#[doc = "Register `DMATXDSAR` reader"]
pub struct R(crate::R<DMATXDSAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMATXDSAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMATXDSAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMATXDSAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMATXDSAR` writer"]
pub struct W(crate::W<DMATXDSAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMATXDSAR_SPEC>;
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
impl From<crate::W<DMATXDSAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMATXDSAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXDSA` reader - Start of receive list"]
pub type TXDSA_R = crate::FieldReader<u32, u32>;
#[doc = "Field `TXDSA` writer - Start of receive list"]
pub type TXDSA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DMATXDSAR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Start of receive list"]
    #[inline(always)]
    pub fn txdsa(&self) -> TXDSA_R {
        TXDSA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Start of receive list"]
    #[inline(always)]
    #[must_use]
    pub fn txdsa(&mut self) -> TXDSA_W<0> {
        TXDSA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ethernet DMA receive descriptor list address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmatxdsar](index.html) module"]
pub struct DMATXDSAR_SPEC;
impl crate::RegisterSpec for DMATXDSAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmatxdsar::R](R) reader structure"]
impl crate::Readable for DMATXDSAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmatxdsar::W](W) writer structure"]
impl crate::Writable for DMATXDSAR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMATXDSAR to value 0"]
impl crate::Resettable for DMATXDSAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
