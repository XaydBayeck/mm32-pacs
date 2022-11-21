#[doc = "Register `JOFR3` reader"]
pub struct R(crate::R<JOFR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<JOFR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<JOFR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<JOFR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `JOFR3` writer"]
pub struct W(crate::W<JOFR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<JOFR3_SPEC>;
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
impl From<crate::W<JOFR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<JOFR3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `JOFR` reader - The A_D conversion result of injection channel 3 is compensated After subtracting the compensation value, the conversion result is obtained and stored in jdata"]
pub type JOFR_R = crate::FieldReader<u16, u16>;
#[doc = "Field `JOFR` writer - The A_D conversion result of injection channel 3 is compensated After subtracting the compensation value, the conversion result is obtained and stored in jdata"]
pub type JOFR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, JOFR3_SPEC, u16, u16, 12, O>;
impl R {
    #[doc = "Bits 0:11 - The A_D conversion result of injection channel 3 is compensated After subtracting the compensation value, the conversion result is obtained and stored in jdata"]
    #[inline(always)]
    pub fn jofr(&self) -> JOFR_R {
        JOFR_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - The A_D conversion result of injection channel 3 is compensated After subtracting the compensation value, the conversion result is obtained and stored in jdata"]
    #[inline(always)]
    #[must_use]
    pub fn jofr(&mut self) -> JOFR_W<0> {
        JOFR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Injection channe 3 data compensation register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [jofr3](index.html) module"]
pub struct JOFR3_SPEC;
impl crate::RegisterSpec for JOFR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [jofr3::R](R) reader structure"]
impl crate::Readable for JOFR3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [jofr3::W](W) writer structure"]
impl crate::Writable for JOFR3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets JOFR3 to value 0"]
impl crate::Resettable for JOFR3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
