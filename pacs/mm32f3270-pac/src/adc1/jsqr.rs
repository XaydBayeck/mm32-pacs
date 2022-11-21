#[doc = "Register `JSQR` reader"]
pub struct R(crate::R<JSQR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<JSQR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<JSQR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<JSQR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `JSQR` writer"]
pub struct W(crate::W<JSQR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<JSQR_SPEC>;
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
impl From<crate::W<JSQR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<JSQR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `JSQ0` reader - The first conversion channel in the injection sequence"]
pub type JSQ0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `JSQ0` writer - The first conversion channel in the injection sequence"]
pub type JSQ0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, JSQR_SPEC, u8, u8, 5, O>;
#[doc = "Field `JSQ1` reader - The second conversion channel in the injection sequence"]
pub type JSQ1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `JSQ1` writer - The second conversion channel in the injection sequence"]
pub type JSQ1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, JSQR_SPEC, u8, u8, 5, O>;
#[doc = "Field `JSQ2` reader - The third conversion channel number in the injection sequence"]
pub type JSQ2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `JSQ2` writer - The third conversion channel number in the injection sequence"]
pub type JSQ2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, JSQR_SPEC, u8, u8, 5, O>;
#[doc = "Field `JSQ3` reader - The fourth conversion channel number in the injection sequence"]
pub type JSQ3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `JSQ3` writer - The fourth conversion channel number in the injection sequence"]
pub type JSQ3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, JSQR_SPEC, u8, u8, 5, O>;
#[doc = "Field `JNUMM` reader - Injection channel sequence length"]
pub type JNUMM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `JNUMM` writer - Injection channel sequence length"]
pub type JNUMM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, JSQR_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:4 - The first conversion channel in the injection sequence"]
    #[inline(always)]
    pub fn jsq0(&self) -> JSQ0_R {
        JSQ0_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - The second conversion channel in the injection sequence"]
    #[inline(always)]
    pub fn jsq1(&self) -> JSQ1_R {
        JSQ1_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 10:14 - The third conversion channel number in the injection sequence"]
    #[inline(always)]
    pub fn jsq2(&self) -> JSQ2_R {
        JSQ2_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bits 15:19 - The fourth conversion channel number in the injection sequence"]
    #[inline(always)]
    pub fn jsq3(&self) -> JSQ3_R {
        JSQ3_R::new(((self.bits >> 15) & 0x1f) as u8)
    }
    #[doc = "Bits 20:21 - Injection channel sequence length"]
    #[inline(always)]
    pub fn jnumm(&self) -> JNUMM_R {
        JNUMM_R::new(((self.bits >> 20) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - The first conversion channel in the injection sequence"]
    #[inline(always)]
    #[must_use]
    pub fn jsq0(&mut self) -> JSQ0_W<0> {
        JSQ0_W::new(self)
    }
    #[doc = "Bits 5:9 - The second conversion channel in the injection sequence"]
    #[inline(always)]
    #[must_use]
    pub fn jsq1(&mut self) -> JSQ1_W<5> {
        JSQ1_W::new(self)
    }
    #[doc = "Bits 10:14 - The third conversion channel number in the injection sequence"]
    #[inline(always)]
    #[must_use]
    pub fn jsq2(&mut self) -> JSQ2_W<10> {
        JSQ2_W::new(self)
    }
    #[doc = "Bits 15:19 - The fourth conversion channel number in the injection sequence"]
    #[inline(always)]
    #[must_use]
    pub fn jsq3(&mut self) -> JSQ3_W<15> {
        JSQ3_W::new(self)
    }
    #[doc = "Bits 20:21 - Injection channel sequence length"]
    #[inline(always)]
    #[must_use]
    pub fn jnumm(&mut self) -> JNUMM_W<20> {
        JNUMM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Injection sequence register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [jsqr](index.html) module"]
pub struct JSQR_SPEC;
impl crate::RegisterSpec for JSQR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [jsqr::R](R) reader structure"]
impl crate::Readable for JSQR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [jsqr::W](W) writer structure"]
impl crate::Writable for JSQR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets JSQR to value 0"]
impl crate::Resettable for JSQR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
