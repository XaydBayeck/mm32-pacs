#[doc = "Register `AFRL` reader"]
pub struct R(crate::R<AFRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AFRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AFRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AFRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AFRL` writer"]
pub struct W(crate::W<AFRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AFRL_SPEC>;
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
impl From<crate::W<AFRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AFRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AFR0` reader - Multiplexing function selection for bit 0 of portx"]
pub type AFR0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AFR0` writer - Multiplexing function selection for bit 0 of portx"]
pub type AFR0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AFRL_SPEC, u8, u8, 4, O>;
#[doc = "Field `AFR1` reader - Multiplexing function selection for bit 1 of portx"]
pub type AFR1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AFR1` writer - Multiplexing function selection for bit 1 of portx"]
pub type AFR1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AFRL_SPEC, u8, u8, 4, O>;
#[doc = "Field `AFR2` reader - Multiplexing function selection for bit 2 of portx"]
pub type AFR2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AFR2` writer - Multiplexing function selection for bit 2 of portx"]
pub type AFR2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AFRL_SPEC, u8, u8, 4, O>;
#[doc = "Field `AFR3` reader - Multiplexing function selection for bit 3 of portx"]
pub type AFR3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AFR3` writer - Multiplexing function selection for bit 3 of portx"]
pub type AFR3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AFRL_SPEC, u8, u8, 4, O>;
#[doc = "Field `AFR4` reader - Multiplexing function selection for bit 4 of portx"]
pub type AFR4_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AFR4` writer - Multiplexing function selection for bit 4 of portx"]
pub type AFR4_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AFRL_SPEC, u8, u8, 4, O>;
#[doc = "Field `AFR5` reader - Multiplexing function selection for bit 5 of portx"]
pub type AFR5_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AFR5` writer - Multiplexing function selection for bit 5 of portx"]
pub type AFR5_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AFRL_SPEC, u8, u8, 4, O>;
#[doc = "Field `AFR6` reader - Multiplexing function selection for bit 6 of portx"]
pub type AFR6_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AFR6` writer - Multiplexing function selection for bit 6 of portx"]
pub type AFR6_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AFRL_SPEC, u8, u8, 4, O>;
#[doc = "Field `AFR7` reader - Multiplexing function selection for bit 7 of portx"]
pub type AFR7_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AFR7` writer - Multiplexing function selection for bit 7 of portx"]
pub type AFR7_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AFRL_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3 - Multiplexing function selection for bit 0 of portx"]
    #[inline(always)]
    pub fn afr0(&self) -> AFR0_R {
        AFR0_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Multiplexing function selection for bit 1 of portx"]
    #[inline(always)]
    pub fn afr1(&self) -> AFR1_R {
        AFR1_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Multiplexing function selection for bit 2 of portx"]
    #[inline(always)]
    pub fn afr2(&self) -> AFR2_R {
        AFR2_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Multiplexing function selection for bit 3 of portx"]
    #[inline(always)]
    pub fn afr3(&self) -> AFR3_R {
        AFR3_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Multiplexing function selection for bit 4 of portx"]
    #[inline(always)]
    pub fn afr4(&self) -> AFR4_R {
        AFR4_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Multiplexing function selection for bit 5 of portx"]
    #[inline(always)]
    pub fn afr5(&self) -> AFR5_R {
        AFR5_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Multiplexing function selection for bit 6 of portx"]
    #[inline(always)]
    pub fn afr6(&self) -> AFR6_R {
        AFR6_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Multiplexing function selection for bit 7 of portx"]
    #[inline(always)]
    pub fn afr7(&self) -> AFR7_R {
        AFR7_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Multiplexing function selection for bit 0 of portx"]
    #[inline(always)]
    #[must_use]
    pub fn afr0(&mut self) -> AFR0_W<0> {
        AFR0_W::new(self)
    }
    #[doc = "Bits 4:7 - Multiplexing function selection for bit 1 of portx"]
    #[inline(always)]
    #[must_use]
    pub fn afr1(&mut self) -> AFR1_W<4> {
        AFR1_W::new(self)
    }
    #[doc = "Bits 8:11 - Multiplexing function selection for bit 2 of portx"]
    #[inline(always)]
    #[must_use]
    pub fn afr2(&mut self) -> AFR2_W<8> {
        AFR2_W::new(self)
    }
    #[doc = "Bits 12:15 - Multiplexing function selection for bit 3 of portx"]
    #[inline(always)]
    #[must_use]
    pub fn afr3(&mut self) -> AFR3_W<12> {
        AFR3_W::new(self)
    }
    #[doc = "Bits 16:19 - Multiplexing function selection for bit 4 of portx"]
    #[inline(always)]
    #[must_use]
    pub fn afr4(&mut self) -> AFR4_W<16> {
        AFR4_W::new(self)
    }
    #[doc = "Bits 20:23 - Multiplexing function selection for bit 5 of portx"]
    #[inline(always)]
    #[must_use]
    pub fn afr5(&mut self) -> AFR5_W<20> {
        AFR5_W::new(self)
    }
    #[doc = "Bits 24:27 - Multiplexing function selection for bit 6 of portx"]
    #[inline(always)]
    #[must_use]
    pub fn afr6(&mut self) -> AFR6_W<24> {
        AFR6_W::new(self)
    }
    #[doc = "Bits 28:31 - Multiplexing function selection for bit 7 of portx"]
    #[inline(always)]
    #[must_use]
    pub fn afr7(&mut self) -> AFR7_W<28> {
        AFR7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port Multiplexing Function Low Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [afrl](index.html) module"]
pub struct AFRL_SPEC;
impl crate::RegisterSpec for AFRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [afrl::R](R) reader structure"]
impl crate::Readable for AFRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [afrl::W](W) writer structure"]
impl crate::Writable for AFRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AFRL to value 0xfff0_0fff"]
impl crate::Resettable for AFRL_SPEC {
    const RESET_VALUE: Self::Ux = 0xfff0_0fff;
}
