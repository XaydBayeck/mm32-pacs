#[doc = "Register `RXMASK` reader"]
pub struct R(crate::R<RXMASK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXMASK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXMASK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXMASK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RXMASK` writer"]
pub struct W(crate::W<RXMASK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RXMASK_SPEC>;
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
impl From<crate::W<RXMASK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RXMASK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXMASK` reader - Synchronous frame match address mask"]
pub type RXMASK_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RXMASK` writer - Synchronous frame match address mask"]
pub type RXMASK_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RXMASK_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Synchronous frame match address mask"]
    #[inline(always)]
    pub fn rxmask(&self) -> RXMASK_R {
        RXMASK_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Synchronous frame match address mask"]
    #[inline(always)]
    #[must_use]
    pub fn rxmask(&mut self) -> RXMASK_W<0> {
        RXMASK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Receive Mask Registe\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxmask](index.html) module"]
pub struct RXMASK_SPEC;
impl crate::RegisterSpec for RXMASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxmask::R](R) reader structure"]
impl crate::Readable for RXMASK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rxmask::W](W) writer structure"]
impl crate::Writable for RXMASK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RXMASK to value 0xff"]
impl crate::Resettable for RXMASK_SPEC {
    const RESET_VALUE: Self::Ux = 0xff;
}
