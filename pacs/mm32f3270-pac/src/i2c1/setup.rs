#[doc = "Register `SETUP` reader"]
pub struct R(crate::R<SETUP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SETUP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SETUP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SETUP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SETUP` writer"]
pub struct W(crate::W<SETUP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SETUP_SPEC>;
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
impl From<crate::W<SETUP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SETUP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CNT` reader - SDA setup"]
pub type CNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CNT` writer - SDA setup"]
pub type CNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SETUP_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - SDA setup"]
    #[inline(always)]
    pub fn cnt(&self) -> CNT_R {
        CNT_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - SDA setup"]
    #[inline(always)]
    #[must_use]
    pub fn cnt(&mut self) -> CNT_W<0> {
        CNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SDA Setup Time Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [setup](index.html) module"]
pub struct SETUP_SPEC;
impl crate::RegisterSpec for SETUP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [setup::R](R) reader structure"]
impl crate::Readable for SETUP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [setup::W](W) writer structure"]
impl crate::Writable for SETUP_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SETUP to value 0x64"]
impl crate::Resettable for SETUP_SPEC {
    const RESET_VALUE: Self::Ux = 0x64;
}
