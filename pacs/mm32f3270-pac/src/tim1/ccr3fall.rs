#[doc = "Register `CCR3FALL` reader"]
pub struct R(crate::R<CCR3FALL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCR3FALL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCR3FALL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCR3FALL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CCR3FALL` writer"]
pub struct W(crate::W<CCR3FALL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCR3FALL_SPEC>;
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
impl From<crate::W<CCR3FALL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCR3FALL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CCR3FALL` reader - Capture/compare value for ch3 when counting down in PWM center-aligned mode"]
pub type CCR3FALL_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CCR3FALL` writer - Capture/compare value for ch3 when counting down in PWM center-aligned mode"]
pub type CCR3FALL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCR3FALL_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Capture/compare value for ch3 when counting down in PWM center-aligned mode"]
    #[inline(always)]
    pub fn ccr3fall(&self) -> CCR3FALL_R {
        CCR3FALL_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Capture/compare value for ch3 when counting down in PWM center-aligned mode"]
    #[inline(always)]
    #[must_use]
    pub fn ccr3fall(&mut self) -> CCR3FALL_W<0> {
        CCR3FALL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "pwm shift count CCR3 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccr3fall](index.html) module"]
pub struct CCR3FALL_SPEC;
impl crate::RegisterSpec for CCR3FALL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ccr3fall::R](R) reader structure"]
impl crate::Readable for CCR3FALL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ccr3fall::W](W) writer structure"]
impl crate::Writable for CCR3FALL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CCR3FALL to value 0"]
impl crate::Resettable for CCR3FALL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
