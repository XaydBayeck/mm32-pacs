#[doc = "Register `RXFLR` reader"]
pub struct R(crate::R<RXFLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXFLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXFLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXFLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RXFLR` writer"]
pub struct W(crate::W<RXFLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RXFLR_SPEC>;
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
impl From<crate::W<RXFLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RXFLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CNT` reader - Receive FIFO level. Contains the number of valid data entires in the receive FIFO"]
pub type CNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CNT` writer - Receive FIFO level. Contains the number of valid data entires in the receive FIFO"]
pub type CNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RXFLR_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:1 - Receive FIFO level. Contains the number of valid data entires in the receive FIFO"]
    #[inline(always)]
    pub fn cnt(&self) -> CNT_R {
        CNT_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Receive FIFO level. Contains the number of valid data entires in the receive FIFO"]
    #[inline(always)]
    #[must_use]
    pub fn cnt(&mut self) -> CNT_W<0> {
        CNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Receive FIFO Level Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxflr](index.html) module"]
pub struct RXFLR_SPEC;
impl crate::RegisterSpec for RXFLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxflr::R](R) reader structure"]
impl crate::Readable for RXFLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rxflr::W](W) writer structure"]
impl crate::Writable for RXFLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RXFLR to value 0"]
impl crate::Resettable for RXFLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
