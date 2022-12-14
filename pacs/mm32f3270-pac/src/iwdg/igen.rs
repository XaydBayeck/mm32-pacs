#[doc = "Register `IGEN` reader"]
pub struct R(crate::R<IGEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IGEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IGEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IGEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IGEN` writer"]
pub struct W(crate::W<IGEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IGEN_SPEC>;
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
impl From<crate::W<IGEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IGEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IGEN` reader - Watchdog Interrupt Generate value"]
pub type IGEN_R = crate::FieldReader<u16, u16>;
#[doc = "Field `IGEN` writer - Watchdog Interrupt Generate value"]
pub type IGEN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IGEN_SPEC, u16, u16, 12, O>;
impl R {
    #[doc = "Bits 0:11 - Watchdog Interrupt Generate value"]
    #[inline(always)]
    pub fn igen(&self) -> IGEN_R {
        IGEN_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Watchdog Interrupt Generate value"]
    #[inline(always)]
    #[must_use]
    pub fn igen(&mut self) -> IGEN_W<0> {
        IGEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interruput generate value register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [igen](index.html) module"]
pub struct IGEN_SPEC;
impl crate::RegisterSpec for IGEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [igen::R](R) reader structure"]
impl crate::Readable for IGEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [igen::W](W) writer structure"]
impl crate::Writable for IGEN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IGEN to value 0x0fff"]
impl crate::Resettable for IGEN_SPEC {
    const RESET_VALUE: Self::Ux = 0x0fff;
}
