#[doc = "Register `AFRH` reader"]
pub struct R(crate::R<AFRH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AFRH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AFRH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AFRH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AFRH` writer"]
pub struct W(crate::W<AFRH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AFRH_SPEC>;
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
impl From<crate::W<AFRH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AFRH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AFR8` reader - Multiplexing function selection for bit 8 of portx"]
pub type AFR8_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AFR8` writer - Multiplexing function selection for bit 8 of portx"]
pub type AFR8_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AFRH_SPEC, u8, u8, 4, O>;
#[doc = "Field `AFR9` reader - Multiplexing function selection for bit 9 of portx"]
pub type AFR9_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AFR9` writer - Multiplexing function selection for bit 9 of portx"]
pub type AFR9_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AFRH_SPEC, u8, u8, 4, O>;
#[doc = "Field `AFR10` reader - Multiplexing function selection for bit 10 of portx"]
pub type AFR10_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AFR10` writer - Multiplexing function selection for bit 10 of portx"]
pub type AFR10_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AFRH_SPEC, u8, u8, 4, O>;
#[doc = "Field `AFR11` reader - Multiplexing function selection for bit 11 of portx"]
pub type AFR11_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AFR11` writer - Multiplexing function selection for bit 11 of portx"]
pub type AFR11_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AFRH_SPEC, u8, u8, 4, O>;
#[doc = "Field `AFR12` reader - Multiplexing function selection for bit 12 of portx"]
pub type AFR12_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AFR12` writer - Multiplexing function selection for bit 12 of portx"]
pub type AFR12_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AFRH_SPEC, u8, u8, 4, O>;
#[doc = "Field `AFR13` reader - Multiplexing function selection for bit 13 of portx"]
pub type AFR13_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AFR13` writer - Multiplexing function selection for bit 13 of portx"]
pub type AFR13_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AFRH_SPEC, u8, u8, 4, O>;
#[doc = "Field `AFR14` reader - Multiplexing function selection for bit 14 of portx"]
pub type AFR14_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AFR14` writer - Multiplexing function selection for bit 14 of portx"]
pub type AFR14_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AFRH_SPEC, u8, u8, 4, O>;
#[doc = "Field `AFR15` reader - Multiplexing function selection for bit 15 of portx"]
pub type AFR15_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AFR15` writer - Multiplexing function selection for bit 15 of portx"]
pub type AFR15_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AFRH_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3 - Multiplexing function selection for bit 8 of portx"]
    #[inline(always)]
    pub fn afr8(&self) -> AFR8_R {
        AFR8_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Multiplexing function selection for bit 9 of portx"]
    #[inline(always)]
    pub fn afr9(&self) -> AFR9_R {
        AFR9_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Multiplexing function selection for bit 10 of portx"]
    #[inline(always)]
    pub fn afr10(&self) -> AFR10_R {
        AFR10_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Multiplexing function selection for bit 11 of portx"]
    #[inline(always)]
    pub fn afr11(&self) -> AFR11_R {
        AFR11_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Multiplexing function selection for bit 12 of portx"]
    #[inline(always)]
    pub fn afr12(&self) -> AFR12_R {
        AFR12_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Multiplexing function selection for bit 13 of portx"]
    #[inline(always)]
    pub fn afr13(&self) -> AFR13_R {
        AFR13_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Multiplexing function selection for bit 14 of portx"]
    #[inline(always)]
    pub fn afr14(&self) -> AFR14_R {
        AFR14_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Multiplexing function selection for bit 15 of portx"]
    #[inline(always)]
    pub fn afr15(&self) -> AFR15_R {
        AFR15_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Multiplexing function selection for bit 8 of portx"]
    #[inline(always)]
    #[must_use]
    pub fn afr8(&mut self) -> AFR8_W<0> {
        AFR8_W::new(self)
    }
    #[doc = "Bits 4:7 - Multiplexing function selection for bit 9 of portx"]
    #[inline(always)]
    #[must_use]
    pub fn afr9(&mut self) -> AFR9_W<4> {
        AFR9_W::new(self)
    }
    #[doc = "Bits 8:11 - Multiplexing function selection for bit 10 of portx"]
    #[inline(always)]
    #[must_use]
    pub fn afr10(&mut self) -> AFR10_W<8> {
        AFR10_W::new(self)
    }
    #[doc = "Bits 12:15 - Multiplexing function selection for bit 11 of portx"]
    #[inline(always)]
    #[must_use]
    pub fn afr11(&mut self) -> AFR11_W<12> {
        AFR11_W::new(self)
    }
    #[doc = "Bits 16:19 - Multiplexing function selection for bit 12 of portx"]
    #[inline(always)]
    #[must_use]
    pub fn afr12(&mut self) -> AFR12_W<16> {
        AFR12_W::new(self)
    }
    #[doc = "Bits 20:23 - Multiplexing function selection for bit 13 of portx"]
    #[inline(always)]
    #[must_use]
    pub fn afr13(&mut self) -> AFR13_W<20> {
        AFR13_W::new(self)
    }
    #[doc = "Bits 24:27 - Multiplexing function selection for bit 14 of portx"]
    #[inline(always)]
    #[must_use]
    pub fn afr14(&mut self) -> AFR14_W<24> {
        AFR14_W::new(self)
    }
    #[doc = "Bits 28:31 - Multiplexing function selection for bit 15 of portx"]
    #[inline(always)]
    #[must_use]
    pub fn afr15(&mut self) -> AFR15_W<28> {
        AFR15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port Multiplexing Function High Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [afrh](index.html) module"]
pub struct AFRH_SPEC;
impl crate::RegisterSpec for AFRH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [afrh::R](R) reader structure"]
impl crate::Readable for AFRH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [afrh::W](W) writer structure"]
impl crate::Writable for AFRH_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AFRH to value 0xf00f_ffff"]
impl crate::Resettable for AFRH_SPEC {
    const RESET_VALUE: Self::Ux = 0xf00f_ffff;
}
