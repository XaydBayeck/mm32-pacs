#[doc = "Register `ANY_CFG` reader"]
pub struct R(crate::R<ANY_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ANY_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ANY_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ANY_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ANY_CFG` writer"]
pub struct W(crate::W<ANY_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ANY_CFG_SPEC>;
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
impl From<crate::W<ANY_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ANY_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CHANY_NUM` reader - channel number configuration"]
pub type CHANY_NUM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CHANY_NUM` writer - channel number configuration"]
pub type CHANY_NUM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ANY_CFG_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3 - channel number configuration"]
    #[inline(always)]
    pub fn chany_num(&self) -> CHANY_NUM_R {
        CHANY_NUM_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - channel number configuration"]
    #[inline(always)]
    #[must_use]
    pub fn chany_num(&mut self) -> CHANY_NUM_W<0> {
        CHANY_NUM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Arbitrary channel configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [any_cfg](index.html) module"]
pub struct ANY_CFG_SPEC;
impl crate::RegisterSpec for ANY_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [any_cfg::R](R) reader structure"]
impl crate::Readable for ANY_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [any_cfg::W](W) writer structure"]
impl crate::Writable for ANY_CFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ANY_CFG to value 0"]
impl crate::Resettable for ANY_CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
