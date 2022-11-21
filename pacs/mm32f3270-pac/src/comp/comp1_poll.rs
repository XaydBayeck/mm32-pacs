#[doc = "Register `COMP1_POLL` reader"]
pub struct R(crate::R<COMP1_POLL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COMP1_POLL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COMP1_POLL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COMP1_POLL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `COMP1_POLL` writer"]
pub struct W(crate::W<COMP1_POLL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<COMP1_POLL_SPEC>;
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
impl From<crate::W<COMP1_POLL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<COMP1_POLL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EN` reader - Comparator 1 polling enable"]
pub type EN_R = crate::BitReader<bool>;
#[doc = "Field `EN` writer - Comparator 1 polling enable"]
pub type EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, COMP1_POLL_SPEC, bool, O>;
#[doc = "Field `CH` reader - Comparator 1 polling channel"]
pub type CH_R = crate::BitReader<bool>;
#[doc = "Field `CH` writer - Comparator 1 polling channel"]
pub type CH_W<'a, const O: u8> = crate::BitWriter<'a, u32, COMP1_POLL_SPEC, bool, O>;
#[doc = "Field `FIXN` reader - Comparator 1 Polling inverting input fix"]
pub type FIXN_R = crate::BitReader<bool>;
#[doc = "Field `FIXN` writer - Comparator 1 Polling inverting input fix"]
pub type FIXN_W<'a, const O: u8> = crate::BitWriter<'a, u32, COMP1_POLL_SPEC, bool, O>;
#[doc = "Field `PERIOD` reader - Comparator 1 polling wait cycle"]
pub type PERIOD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PERIOD` writer - Comparator 1 polling wait cycle"]
pub type PERIOD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, COMP1_POLL_SPEC, u8, u8, 3, O>;
#[doc = "Field `POUT` reader - Comparator 1 Polling output"]
pub type POUT_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bit 0 - Comparator 1 polling enable"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Comparator 1 polling channel"]
    #[inline(always)]
    pub fn ch(&self) -> CH_R {
        CH_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Comparator 1 Polling inverting input fix"]
    #[inline(always)]
    pub fn fixn(&self) -> FIXN_R {
        FIXN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Comparator 1 polling wait cycle"]
    #[inline(always)]
    pub fn period(&self) -> PERIOD_R {
        PERIOD_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - Comparator 1 Polling output"]
    #[inline(always)]
    pub fn pout(&self) -> POUT_R {
        POUT_R::new(((self.bits >> 8) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Comparator 1 polling enable"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<0> {
        EN_W::new(self)
    }
    #[doc = "Bit 1 - Comparator 1 polling channel"]
    #[inline(always)]
    #[must_use]
    pub fn ch(&mut self) -> CH_W<1> {
        CH_W::new(self)
    }
    #[doc = "Bit 2 - Comparator 1 Polling inverting input fix"]
    #[inline(always)]
    #[must_use]
    pub fn fixn(&mut self) -> FIXN_W<2> {
        FIXN_W::new(self)
    }
    #[doc = "Bits 4:6 - Comparator 1 polling wait cycle"]
    #[inline(always)]
    #[must_use]
    pub fn period(&mut self) -> PERIOD_W<4> {
        PERIOD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "COMP1 Polling Output Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [comp1_poll](index.html) module"]
pub struct COMP1_POLL_SPEC;
impl crate::RegisterSpec for COMP1_POLL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [comp1_poll::R](R) reader structure"]
impl crate::Readable for COMP1_POLL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [comp1_poll::W](W) writer structure"]
impl crate::Writable for COMP1_POLL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets COMP1_POLL to value 0"]
impl crate::Resettable for COMP1_POLL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
