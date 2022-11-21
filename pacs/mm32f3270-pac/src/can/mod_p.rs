#[doc = "Register `MOD_P` reader"]
pub struct R(crate::R<MOD_P_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MOD_P_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MOD_P_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MOD_P_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MOD_P` writer"]
pub struct W(crate::W<MOD_P_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MOD_P_SPEC>;
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
impl From<crate::W<MOD_P_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MOD_P_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RM` reader - Reset mode"]
pub type RM_R = crate::BitReader<bool>;
#[doc = "Field `RM` writer - Reset mode"]
pub type RM_W<'a, const O: u8> = crate::BitWriter<'a, u32, MOD_P_SPEC, bool, O>;
#[doc = "Field `LOM` reader - Listen only mode"]
pub type LOM_R = crate::BitReader<bool>;
#[doc = "Field `LOM` writer - Listen only mode"]
pub type LOM_W<'a, const O: u8> = crate::BitWriter<'a, u32, MOD_P_SPEC, bool, O>;
#[doc = "Field `STM` reader - Self test mode"]
pub type STM_R = crate::BitReader<bool>;
#[doc = "Field `STM` writer - Self test mode"]
pub type STM_W<'a, const O: u8> = crate::BitWriter<'a, u32, MOD_P_SPEC, bool, O>;
#[doc = "Field `AFM` reader - Acceptance filter mode"]
pub type AFM_R = crate::BitReader<bool>;
#[doc = "Field `AFM` writer - Acceptance filter mode"]
pub type AFM_W<'a, const O: u8> = crate::BitWriter<'a, u32, MOD_P_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Reset mode"]
    #[inline(always)]
    pub fn rm(&self) -> RM_R {
        RM_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Listen only mode"]
    #[inline(always)]
    pub fn lom(&self) -> LOM_R {
        LOM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Self test mode"]
    #[inline(always)]
    pub fn stm(&self) -> STM_R {
        STM_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Acceptance filter mode"]
    #[inline(always)]
    pub fn afm(&self) -> AFM_R {
        AFM_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Reset mode"]
    #[inline(always)]
    #[must_use]
    pub fn rm(&mut self) -> RM_W<0> {
        RM_W::new(self)
    }
    #[doc = "Bit 1 - Listen only mode"]
    #[inline(always)]
    #[must_use]
    pub fn lom(&mut self) -> LOM_W<1> {
        LOM_W::new(self)
    }
    #[doc = "Bit 2 - Self test mode"]
    #[inline(always)]
    #[must_use]
    pub fn stm(&mut self) -> STM_W<2> {
        STM_W::new(self)
    }
    #[doc = "Bit 3 - Acceptance filter mode"]
    #[inline(always)]
    #[must_use]
    pub fn afm(&mut self) -> AFM_W<3> {
        AFM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Peli Mode register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mod_p](index.html) module"]
pub struct MOD_P_SPEC;
impl crate::RegisterSpec for MOD_P_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mod_p::R](R) reader structure"]
impl crate::Readable for MOD_P_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mod_p::W](W) writer structure"]
impl crate::Writable for MOD_P_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MOD_P to value 0x01"]
impl crate::Resettable for MOD_P_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
