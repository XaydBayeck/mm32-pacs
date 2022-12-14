#[doc = "Register `PDER` reader"]
pub struct R(crate::R<PDER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PDER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PDER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PDER` writer"]
pub struct W(crate::W<PDER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDER_SPEC>;
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
impl From<crate::W<PDER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PDER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CCDREPPE` reader - DMA Update repeat mode select"]
pub type CCDREPPE_R = crate::BitReader<bool>;
#[doc = "Field `CCDREPPE` writer - DMA Update repeat mode select"]
pub type CCDREPPE_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDER_SPEC, bool, O>;
#[doc = "Field `CCR1SHIFTEN` reader - CCR1 pwm shift enable"]
pub type CCR1SHIFTEN_R = crate::BitReader<bool>;
#[doc = "Field `CCR1SHIFTEN` writer - CCR1 pwm shift enable"]
pub type CCR1SHIFTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDER_SPEC, bool, O>;
#[doc = "Field `CCR2SHIFTEN` reader - CCR2 pwm shift enable"]
pub type CCR2SHIFTEN_R = crate::BitReader<bool>;
#[doc = "Field `CCR2SHIFTEN` writer - CCR2 pwm shift enable"]
pub type CCR2SHIFTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDER_SPEC, bool, O>;
#[doc = "Field `CCR3SHIFTEN` reader - CCR3 pwm shift enable"]
pub type CCR3SHIFTEN_R = crate::BitReader<bool>;
#[doc = "Field `CCR3SHIFTEN` writer - CCR3 pwm shift enable"]
pub type CCR3SHIFTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDER_SPEC, bool, O>;
#[doc = "Field `CCR4SHIFTEN` reader - CCR4 pwm shift enable"]
pub type CCR4SHIFTEN_R = crate::BitReader<bool>;
#[doc = "Field `CCR4SHIFTEN` writer - CCR4 pwm shift enable"]
pub type CCR4SHIFTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDER_SPEC, bool, O>;
#[doc = "Field `CCR5SHIFTEN` reader - CCR5 pwm shift enable"]
pub type CCR5SHIFTEN_R = crate::BitReader<bool>;
#[doc = "Field `CCR5SHIFTEN` writer - CCR5 pwm shift enable"]
pub type CCR5SHIFTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDER_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - DMA Update repeat mode select"]
    #[inline(always)]
    pub fn ccdreppe(&self) -> CCDREPPE_R {
        CCDREPPE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CCR1 pwm shift enable"]
    #[inline(always)]
    pub fn ccr1shiften(&self) -> CCR1SHIFTEN_R {
        CCR1SHIFTEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CCR2 pwm shift enable"]
    #[inline(always)]
    pub fn ccr2shiften(&self) -> CCR2SHIFTEN_R {
        CCR2SHIFTEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CCR3 pwm shift enable"]
    #[inline(always)]
    pub fn ccr3shiften(&self) -> CCR3SHIFTEN_R {
        CCR3SHIFTEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CCR4 pwm shift enable"]
    #[inline(always)]
    pub fn ccr4shiften(&self) -> CCR4SHIFTEN_R {
        CCR4SHIFTEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - CCR5 pwm shift enable"]
    #[inline(always)]
    pub fn ccr5shiften(&self) -> CCR5SHIFTEN_R {
        CCR5SHIFTEN_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMA Update repeat mode select"]
    #[inline(always)]
    #[must_use]
    pub fn ccdreppe(&mut self) -> CCDREPPE_W<0> {
        CCDREPPE_W::new(self)
    }
    #[doc = "Bit 1 - CCR1 pwm shift enable"]
    #[inline(always)]
    #[must_use]
    pub fn ccr1shiften(&mut self) -> CCR1SHIFTEN_W<1> {
        CCR1SHIFTEN_W::new(self)
    }
    #[doc = "Bit 2 - CCR2 pwm shift enable"]
    #[inline(always)]
    #[must_use]
    pub fn ccr2shiften(&mut self) -> CCR2SHIFTEN_W<2> {
        CCR2SHIFTEN_W::new(self)
    }
    #[doc = "Bit 3 - CCR3 pwm shift enable"]
    #[inline(always)]
    #[must_use]
    pub fn ccr3shiften(&mut self) -> CCR3SHIFTEN_W<3> {
        CCR3SHIFTEN_W::new(self)
    }
    #[doc = "Bit 4 - CCR4 pwm shift enable"]
    #[inline(always)]
    #[must_use]
    pub fn ccr4shiften(&mut self) -> CCR4SHIFTEN_W<4> {
        CCR4SHIFTEN_W::new(self)
    }
    #[doc = "Bit 5 - CCR5 pwm shift enable"]
    #[inline(always)]
    #[must_use]
    pub fn ccr5shiften(&mut self) -> CCR5SHIFTEN_W<5> {
        CCR5SHIFTEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM/DMA repeat enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pder](index.html) module"]
pub struct PDER_SPEC;
impl crate::RegisterSpec for PDER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pder::R](R) reader structure"]
impl crate::Readable for PDER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pder::W](W) writer structure"]
impl crate::Writable for PDER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PDER to value 0"]
impl crate::Resettable for PDER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
