#[doc = "Register `CCR5FALL` reader"]
pub struct R(crate::R<CCR5FALL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCR5FALL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCR5FALL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCR5FALL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CCR5FALL` writer"]
pub struct W(crate::W<CCR5FALL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCR5FALL_SPEC>;
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
impl From<crate::W<CCR5FALL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCR5FALL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CCR5FALL` reader - Capture/compare value for ch5 when counting down in PWM center-aligned mode"]
pub type CCR5FALL_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CCR5FALL` writer - Capture/compare value for ch5 when counting down in PWM center-aligned mode"]
pub type CCR5FALL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCR5FALL_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Capture/compare value for ch5 when counting down in PWM center-aligned mode"]
    #[inline(always)]
    pub fn ccr5fall(&self) -> CCR5FALL_R {
        CCR5FALL_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Capture/compare value for ch5 when counting down in PWM center-aligned mode"]
    #[inline(always)]
    #[must_use]
    pub fn ccr5fall(&mut self) -> CCR5FALL_W<0> {
        CCR5FALL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "pwm shift count CCR5 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccr5fall](index.html) module"]
pub struct CCR5FALL_SPEC;
impl crate::RegisterSpec for CCR5FALL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ccr5fall::R](R) reader structure"]
impl crate::Readable for CCR5FALL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ccr5fall::W](W) writer structure"]
impl crate::Writable for CCR5FALL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CCR5FALL to value 0"]
impl crate::Resettable for CCR5FALL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
