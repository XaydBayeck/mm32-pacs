#[doc = "Register `CHANY1` reader"]
pub struct R(crate::R<CHANY1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHANY1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHANY1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHANY1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CHANY1` writer"]
pub struct W(crate::W<CHANY1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHANY1_SPEC>;
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
impl From<crate::W<CHANY1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CHANY1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CHANY_SEL8` reader - Can be configured as any channel from ch0 to 9, 14 to 15."]
pub type CHANY_SEL8_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CHANY_SEL8` writer - Can be configured as any channel from ch0 to 9, 14 to 15."]
pub type CHANY_SEL8_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CHANY1_SPEC, u8, u8, 4, O>;
#[doc = "Field `CHANY_SEL9` reader - Can be configured as any channel from ch0 to 9, 14 to 15."]
pub type CHANY_SEL9_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CHANY_SEL9` writer - Can be configured as any channel from ch0 to 9, 14 to 15."]
pub type CHANY_SEL9_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CHANY1_SPEC, u8, u8, 4, O>;
#[doc = "Field `CHANY_SEL14` reader - Can be configured as any channel from ch0 to 9, 14 to 15."]
pub type CHANY_SEL14_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CHANY_SEL14` writer - Can be configured as any channel from ch0 to 9, 14 to 15."]
pub type CHANY_SEL14_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CHANY1_SPEC, u8, u8, 4, O>;
#[doc = "Field `CHANY_SEL15` reader - Can be configured as any channel from ch0 to 9, 14 to 15."]
pub type CHANY_SEL15_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CHANY_SEL15` writer - Can be configured as any channel from ch0 to 9, 14 to 15."]
pub type CHANY_SEL15_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CHANY1_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3 - Can be configured as any channel from ch0 to 9, 14 to 15."]
    #[inline(always)]
    pub fn chany_sel8(&self) -> CHANY_SEL8_R {
        CHANY_SEL8_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Can be configured as any channel from ch0 to 9, 14 to 15."]
    #[inline(always)]
    pub fn chany_sel9(&self) -> CHANY_SEL9_R {
        CHANY_SEL9_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Can be configured as any channel from ch0 to 9, 14 to 15."]
    #[inline(always)]
    pub fn chany_sel14(&self) -> CHANY_SEL14_R {
        CHANY_SEL14_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Can be configured as any channel from ch0 to 9, 14 to 15."]
    #[inline(always)]
    pub fn chany_sel15(&self) -> CHANY_SEL15_R {
        CHANY_SEL15_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Can be configured as any channel from ch0 to 9, 14 to 15."]
    #[inline(always)]
    #[must_use]
    pub fn chany_sel8(&mut self) -> CHANY_SEL8_W<0> {
        CHANY_SEL8_W::new(self)
    }
    #[doc = "Bits 4:7 - Can be configured as any channel from ch0 to 9, 14 to 15."]
    #[inline(always)]
    #[must_use]
    pub fn chany_sel9(&mut self) -> CHANY_SEL9_W<4> {
        CHANY_SEL9_W::new(self)
    }
    #[doc = "Bits 24:27 - Can be configured as any channel from ch0 to 9, 14 to 15."]
    #[inline(always)]
    #[must_use]
    pub fn chany_sel14(&mut self) -> CHANY_SEL14_W<24> {
        CHANY_SEL14_W::new(self)
    }
    #[doc = "Bits 28:31 - Can be configured as any channel from ch0 to 9, 14 to 15."]
    #[inline(always)]
    #[must_use]
    pub fn chany_sel15(&mut self) -> CHANY_SEL15_W<28> {
        CHANY_SEL15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Arbitrary channel channel selection register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chany1](index.html) module"]
pub struct CHANY1_SPEC;
impl crate::RegisterSpec for CHANY1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [chany1::R](R) reader structure"]
impl crate::Readable for CHANY1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [chany1::W](W) writer structure"]
impl crate::Writable for CHANY1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CHANY1 to value 0"]
impl crate::Resettable for CHANY1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
