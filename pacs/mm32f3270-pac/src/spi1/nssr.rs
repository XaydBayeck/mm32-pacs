#[doc = "Register `NSSR` reader"]
pub struct R(crate::R<NSSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NSSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NSSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NSSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NSSR` writer"]
pub struct W(crate::W<NSSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NSSR_SPEC>;
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
impl From<crate::W<NSSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NSSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NSS` reader - Chip select output signal in Master mode"]
pub type NSS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `NSS` writer - Chip select output signal in Master mode"]
pub type NSS_W<'a, const O: u8> = crate::FieldWriter<'a, u16, NSSR_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Chip select output signal in Master mode"]
    #[inline(always)]
    pub fn nss(&self) -> NSS_R {
        NSS_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Chip select output signal in Master mode"]
    #[inline(always)]
    #[must_use]
    pub fn nss(&mut self) -> NSS_W<0> {
        NSS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Slave chip select register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nssr](index.html) module"]
pub struct NSSR_SPEC;
impl crate::RegisterSpec for NSSR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [nssr::R](R) reader structure"]
impl crate::Readable for NSSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [nssr::W](W) writer structure"]
impl crate::Writable for NSSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets NSSR to value 0xff"]
impl crate::Resettable for NSSR_SPEC {
    const RESET_VALUE: Self::Ux = 0xff;
}
