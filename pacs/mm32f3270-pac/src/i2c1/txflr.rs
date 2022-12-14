#[doc = "Register `TXFLR` reader"]
pub struct R(crate::R<TXFLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TXFLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TXFLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TXFLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TXFLR` writer"]
pub struct W(crate::W<TXFLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TXFLR_SPEC>;
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
impl From<crate::W<TXFLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TXFLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CNT` reader - Transmit FIFO level.Contains the number of valid data entires in the transmit FIFO"]
pub type CNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CNT` writer - Transmit FIFO level.Contains the number of valid data entires in the transmit FIFO"]
pub type CNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TXFLR_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:1 - Transmit FIFO level.Contains the number of valid data entires in the transmit FIFO"]
    #[inline(always)]
    pub fn cnt(&self) -> CNT_R {
        CNT_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Transmit FIFO level.Contains the number of valid data entires in the transmit FIFO"]
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
#[doc = "Transmit FIFO Level Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txflr](index.html) module"]
pub struct TXFLR_SPEC;
impl crate::RegisterSpec for TXFLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [txflr::R](R) reader structure"]
impl crate::Readable for TXFLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [txflr::W](W) writer structure"]
impl crate::Writable for TXFLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TXFLR to value 0"]
impl crate::Resettable for TXFLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
