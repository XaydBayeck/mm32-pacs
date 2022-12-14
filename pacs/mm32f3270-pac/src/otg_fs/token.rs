#[doc = "Register `TOKEN` reader"]
pub struct R(crate::R<TOKEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TOKEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TOKEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TOKEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TOKEN` writer"]
pub struct W(crate::W<TOKEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TOKEN_SPEC>;
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
impl From<crate::W<TOKEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TOKEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TOKEN_ENDPT` reader - This 4_bit value determines the endpoint address of the token command"]
pub type TOKEN_ENDPT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TOKEN_ENDPT` writer - This 4_bit value determines the endpoint address of the token command"]
pub type TOKEN_ENDPT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TOKEN_SPEC, u8, u8, 4, O>;
#[doc = "Field `TOKEN_PID` reader - This 4_bit value is the token type executed by Vsub"]
pub type TOKEN_PID_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TOKEN_PID` writer - This 4_bit value is the token type executed by Vsub"]
pub type TOKEN_PID_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TOKEN_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3 - This 4_bit value determines the endpoint address of the token command"]
    #[inline(always)]
    pub fn token_endpt(&self) -> TOKEN_ENDPT_R {
        TOKEN_ENDPT_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - This 4_bit value is the token type executed by Vsub"]
    #[inline(always)]
    pub fn token_pid(&self) -> TOKEN_PID_R {
        TOKEN_PID_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - This 4_bit value determines the endpoint address of the token command"]
    #[inline(always)]
    #[must_use]
    pub fn token_endpt(&mut self) -> TOKEN_ENDPT_W<0> {
        TOKEN_ENDPT_W::new(self)
    }
    #[doc = "Bits 4:7 - This 4_bit value is the token type executed by Vsub"]
    #[inline(always)]
    #[must_use]
    pub fn token_pid(&mut self) -> TOKEN_PID_W<4> {
        TOKEN_PID_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Token register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [token](index.html) module"]
pub struct TOKEN_SPEC;
impl crate::RegisterSpec for TOKEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [token::R](R) reader structure"]
impl crate::Readable for TOKEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [token::W](W) writer structure"]
impl crate::Writable for TOKEN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TOKEN to value 0"]
impl crate::Resettable for TOKEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
