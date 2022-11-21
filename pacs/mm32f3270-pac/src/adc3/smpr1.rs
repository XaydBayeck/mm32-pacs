#[doc = "Register `SMPR1` reader"]
pub struct R(crate::R<SMPR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SMPR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SMPR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SMPR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SMPR1` writer"]
pub struct W(crate::W<SMPR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SMPR1_SPEC>;
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
impl From<crate::W<SMPR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SMPR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SAMCTL0` reader - Channel 0 Sample time selection"]
pub type SAMCTL0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SAMCTL0` writer - Channel 0 Sample time selection"]
pub type SAMCTL0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SMPR1_SPEC, u8, u8, 4, O>;
#[doc = "Field `SAMCTL1` reader - Channel 1 Sample time selection"]
pub type SAMCTL1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SAMCTL1` writer - Channel 1 Sample time selection"]
pub type SAMCTL1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SMPR1_SPEC, u8, u8, 4, O>;
#[doc = "Field `SAMCTL2` reader - Channel 2 Sample time selection"]
pub type SAMCTL2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SAMCTL2` writer - Channel 2 Sample time selection"]
pub type SAMCTL2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SMPR1_SPEC, u8, u8, 4, O>;
#[doc = "Field `SAMCTL3` reader - Channel 3 Sample time selection"]
pub type SAMCTL3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SAMCTL3` writer - Channel 3 Sample time selection"]
pub type SAMCTL3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SMPR1_SPEC, u8, u8, 4, O>;
#[doc = "Field `SAMCTL4` reader - Channel 4 Sample time selection"]
pub type SAMCTL4_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SAMCTL4` writer - Channel 4 Sample time selection"]
pub type SAMCTL4_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SMPR1_SPEC, u8, u8, 4, O>;
#[doc = "Field `SAMCTL5` reader - Channel 5 Sample time selection"]
pub type SAMCTL5_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SAMCTL5` writer - Channel 5 Sample time selection"]
pub type SAMCTL5_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SMPR1_SPEC, u8, u8, 4, O>;
#[doc = "Field `SAMCTL6` reader - Channel 6 Sample time selection"]
pub type SAMCTL6_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SAMCTL6` writer - Channel 6 Sample time selection"]
pub type SAMCTL6_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SMPR1_SPEC, u8, u8, 4, O>;
#[doc = "Field `SAMCTL7` reader - Channel 7 Sample time selection"]
pub type SAMCTL7_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SAMCTL7` writer - Channel 7 Sample time selection"]
pub type SAMCTL7_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SMPR1_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3 - Channel 0 Sample time selection"]
    #[inline(always)]
    pub fn samctl0(&self) -> SAMCTL0_R {
        SAMCTL0_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Channel 1 Sample time selection"]
    #[inline(always)]
    pub fn samctl1(&self) -> SAMCTL1_R {
        SAMCTL1_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Channel 2 Sample time selection"]
    #[inline(always)]
    pub fn samctl2(&self) -> SAMCTL2_R {
        SAMCTL2_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Channel 3 Sample time selection"]
    #[inline(always)]
    pub fn samctl3(&self) -> SAMCTL3_R {
        SAMCTL3_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Channel 4 Sample time selection"]
    #[inline(always)]
    pub fn samctl4(&self) -> SAMCTL4_R {
        SAMCTL4_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Channel 5 Sample time selection"]
    #[inline(always)]
    pub fn samctl5(&self) -> SAMCTL5_R {
        SAMCTL5_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Channel 6 Sample time selection"]
    #[inline(always)]
    pub fn samctl6(&self) -> SAMCTL6_R {
        SAMCTL6_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Channel 7 Sample time selection"]
    #[inline(always)]
    pub fn samctl7(&self) -> SAMCTL7_R {
        SAMCTL7_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Channel 0 Sample time selection"]
    #[inline(always)]
    #[must_use]
    pub fn samctl0(&mut self) -> SAMCTL0_W<0> {
        SAMCTL0_W::new(self)
    }
    #[doc = "Bits 4:7 - Channel 1 Sample time selection"]
    #[inline(always)]
    #[must_use]
    pub fn samctl1(&mut self) -> SAMCTL1_W<4> {
        SAMCTL1_W::new(self)
    }
    #[doc = "Bits 8:11 - Channel 2 Sample time selection"]
    #[inline(always)]
    #[must_use]
    pub fn samctl2(&mut self) -> SAMCTL2_W<8> {
        SAMCTL2_W::new(self)
    }
    #[doc = "Bits 12:15 - Channel 3 Sample time selection"]
    #[inline(always)]
    #[must_use]
    pub fn samctl3(&mut self) -> SAMCTL3_W<12> {
        SAMCTL3_W::new(self)
    }
    #[doc = "Bits 16:19 - Channel 4 Sample time selection"]
    #[inline(always)]
    #[must_use]
    pub fn samctl4(&mut self) -> SAMCTL4_W<16> {
        SAMCTL4_W::new(self)
    }
    #[doc = "Bits 20:23 - Channel 5 Sample time selection"]
    #[inline(always)]
    #[must_use]
    pub fn samctl5(&mut self) -> SAMCTL5_W<20> {
        SAMCTL5_W::new(self)
    }
    #[doc = "Bits 24:27 - Channel 6 Sample time selection"]
    #[inline(always)]
    #[must_use]
    pub fn samctl6(&mut self) -> SAMCTL6_W<24> {
        SAMCTL6_W::new(self)
    }
    #[doc = "Bits 28:31 - Channel 7 Sample time selection"]
    #[inline(always)]
    #[must_use]
    pub fn samctl7(&mut self) -> SAMCTL7_W<28> {
        SAMCTL7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Any channel configuration register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smpr1](index.html) module"]
pub struct SMPR1_SPEC;
impl crate::RegisterSpec for SMPR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [smpr1::R](R) reader structure"]
impl crate::Readable for SMPR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [smpr1::W](W) writer structure"]
impl crate::Writable for SMPR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SMPR1 to value 0"]
impl crate::Resettable for SMPR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
