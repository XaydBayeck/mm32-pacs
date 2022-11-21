#[doc = "Register `CCR2FALL` reader"]
pub struct R(crate::R<CCR2FALL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCR2FALL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCR2FALL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCR2FALL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CCR2FALL` writer"]
pub struct W(crate::W<CCR2FALL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCR2FALL_SPEC>;
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
impl From<crate::W<CCR2FALL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCR2FALL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CCR2FALL` reader - Capture/compare value for ch2 when counting down in PWM center-aligned mode"]
pub type CCR2FALL_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CCR2FALL` writer - Capture/compare value for ch2 when counting down in PWM center-aligned mode"]
pub type CCR2FALL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCR2FALL_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Capture/compare value for ch2 when counting down in PWM center-aligned mode"]
    #[inline(always)]
    pub fn ccr2fall(&self) -> CCR2FALL_R {
        CCR2FALL_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Capture/compare value for ch2 when counting down in PWM center-aligned mode"]
    #[inline(always)]
    #[must_use]
    pub fn ccr2fall(&mut self) -> CCR2FALL_W<0> {
        CCR2FALL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "pwm shift count CCR2 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccr2fall](index.html) module"]
pub struct CCR2FALL_SPEC;
impl crate::RegisterSpec for CCR2FALL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ccr2fall::R](R) reader structure"]
impl crate::Readable for CCR2FALL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ccr2fall::W](W) writer structure"]
impl crate::Writable for CCR2FALL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CCR2FALL to value 0"]
impl crate::Resettable for CCR2FALL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
