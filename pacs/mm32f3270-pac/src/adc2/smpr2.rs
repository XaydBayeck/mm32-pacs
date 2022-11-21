#[doc = "Register `SMPR2` reader"]
pub struct R(crate::R<SMPR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SMPR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SMPR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SMPR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SMPR2` writer"]
pub struct W(crate::W<SMPR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SMPR2_SPEC>;
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
impl From<crate::W<SMPR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SMPR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SAMCTL8` reader - Channel 8 Sample time selection"]
pub type SAMCTL8_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SAMCTL8` writer - Channel 8 Sample time selection"]
pub type SAMCTL8_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SMPR2_SPEC, u8, u8, 4, O>;
#[doc = "Field `SAMCTL9` reader - Channel 9 Sample time selection"]
pub type SAMCTL9_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SAMCTL9` writer - Channel 9 Sample time selection"]
pub type SAMCTL9_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SMPR2_SPEC, u8, u8, 4, O>;
#[doc = "Field `SAMCTL10` reader - Channel 10 Sample time selection"]
pub type SAMCTL10_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SAMCTL10` writer - Channel 10 Sample time selection"]
pub type SAMCTL10_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SMPR2_SPEC, u8, u8, 4, O>;
#[doc = "Field `SAMCTL11` reader - Channel 11 Sample time selection"]
pub type SAMCTL11_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SAMCTL11` writer - Channel 11 Sample time selection"]
pub type SAMCTL11_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SMPR2_SPEC, u8, u8, 4, O>;
#[doc = "Field `SAMCTL12` reader - Channel 12 Sample time selection"]
pub type SAMCTL12_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SAMCTL12` writer - Channel 12 Sample time selection"]
pub type SAMCTL12_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SMPR2_SPEC, u8, u8, 4, O>;
#[doc = "Field `SAMCTL13` reader - Channel 13 Sample time selection"]
pub type SAMCTL13_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SAMCTL13` writer - Channel 13 Sample time selection"]
pub type SAMCTL13_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SMPR2_SPEC, u8, u8, 4, O>;
#[doc = "Field `SAMCTL14` reader - Channel 14 Sample time selection"]
pub type SAMCTL14_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SAMCTL14` writer - Channel 14 Sample time selection"]
pub type SAMCTL14_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SMPR2_SPEC, u8, u8, 4, O>;
#[doc = "Field `SAMCTL15` reader - Channel 15 Sample time selection"]
pub type SAMCTL15_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SAMCTL15` writer - Channel 15 Sample time selection"]
pub type SAMCTL15_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SMPR2_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3 - Channel 8 Sample time selection"]
    #[inline(always)]
    pub fn samctl8(&self) -> SAMCTL8_R {
        SAMCTL8_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Channel 9 Sample time selection"]
    #[inline(always)]
    pub fn samctl9(&self) -> SAMCTL9_R {
        SAMCTL9_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Channel 10 Sample time selection"]
    #[inline(always)]
    pub fn samctl10(&self) -> SAMCTL10_R {
        SAMCTL10_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Channel 11 Sample time selection"]
    #[inline(always)]
    pub fn samctl11(&self) -> SAMCTL11_R {
        SAMCTL11_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Channel 12 Sample time selection"]
    #[inline(always)]
    pub fn samctl12(&self) -> SAMCTL12_R {
        SAMCTL12_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Channel 13 Sample time selection"]
    #[inline(always)]
    pub fn samctl13(&self) -> SAMCTL13_R {
        SAMCTL13_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Channel 14 Sample time selection"]
    #[inline(always)]
    pub fn samctl14(&self) -> SAMCTL14_R {
        SAMCTL14_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Channel 15 Sample time selection"]
    #[inline(always)]
    pub fn samctl15(&self) -> SAMCTL15_R {
        SAMCTL15_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Channel 8 Sample time selection"]
    #[inline(always)]
    #[must_use]
    pub fn samctl8(&mut self) -> SAMCTL8_W<0> {
        SAMCTL8_W::new(self)
    }
    #[doc = "Bits 4:7 - Channel 9 Sample time selection"]
    #[inline(always)]
    #[must_use]
    pub fn samctl9(&mut self) -> SAMCTL9_W<4> {
        SAMCTL9_W::new(self)
    }
    #[doc = "Bits 8:11 - Channel 10 Sample time selection"]
    #[inline(always)]
    #[must_use]
    pub fn samctl10(&mut self) -> SAMCTL10_W<8> {
        SAMCTL10_W::new(self)
    }
    #[doc = "Bits 12:15 - Channel 11 Sample time selection"]
    #[inline(always)]
    #[must_use]
    pub fn samctl11(&mut self) -> SAMCTL11_W<12> {
        SAMCTL11_W::new(self)
    }
    #[doc = "Bits 16:19 - Channel 12 Sample time selection"]
    #[inline(always)]
    #[must_use]
    pub fn samctl12(&mut self) -> SAMCTL12_W<16> {
        SAMCTL12_W::new(self)
    }
    #[doc = "Bits 20:23 - Channel 13 Sample time selection"]
    #[inline(always)]
    #[must_use]
    pub fn samctl13(&mut self) -> SAMCTL13_W<20> {
        SAMCTL13_W::new(self)
    }
    #[doc = "Bits 24:27 - Channel 14 Sample time selection"]
    #[inline(always)]
    #[must_use]
    pub fn samctl14(&mut self) -> SAMCTL14_W<24> {
        SAMCTL14_W::new(self)
    }
    #[doc = "Bits 28:31 - Channel 15 Sample time selection"]
    #[inline(always)]
    #[must_use]
    pub fn samctl15(&mut self) -> SAMCTL15_W<28> {
        SAMCTL15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Any channel configuration register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smpr2](index.html) module"]
pub struct SMPR2_SPEC;
impl crate::RegisterSpec for SMPR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [smpr2::R](R) reader structure"]
impl crate::Readable for SMPR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [smpr2::W](W) writer structure"]
impl crate::Writable for SMPR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SMPR2 to value 0"]
impl crate::Resettable for SMPR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
