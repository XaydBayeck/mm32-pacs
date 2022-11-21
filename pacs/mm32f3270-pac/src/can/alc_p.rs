#[doc = "Register `ALC_P` reader"]
pub struct R(crate::R<ALC_P_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ALC_P_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ALC_P_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ALC_P_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ALC_P` writer"]
pub struct W(crate::W<ALC_P_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ALC_P_SPEC>;
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
impl From<crate::W<ALC_P_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ALC_P_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BITNO` reader - Bit number"]
pub type BITNO_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BITNO` writer - Bit number"]
pub type BITNO_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ALC_P_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:4 - Bit number"]
    #[inline(always)]
    pub fn bitno(&self) -> BITNO_R {
        BITNO_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Bit number"]
    #[inline(always)]
    #[must_use]
    pub fn bitno(&mut self) -> BITNO_W<0> {
        BITNO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Peli Arbitration Lost Capture register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [alc_p](index.html) module"]
pub struct ALC_P_SPEC;
impl crate::RegisterSpec for ALC_P_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [alc_p::R](R) reader structure"]
impl crate::Readable for ALC_P_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [alc_p::W](W) writer structure"]
impl crate::Writable for ALC_P_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ALC_P to value 0"]
impl crate::Resettable for ALC_P_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
