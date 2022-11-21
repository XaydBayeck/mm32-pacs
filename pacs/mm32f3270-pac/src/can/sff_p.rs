#[doc = "Register `SFF_P` reader"]
pub struct R(crate::R<SFF_P_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SFF_P_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SFF_P_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SFF_P_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SFF_P` writer"]
pub struct W(crate::W<SFF_P_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SFF_P_SPEC>;
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
impl From<crate::W<SFF_P_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SFF_P_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DLC` reader - Data length code bit"]
pub type DLC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DLC` writer - Data length code bit"]
pub type DLC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SFF_P_SPEC, u8, u8, 4, O>;
#[doc = "Field `RTR` reader - Remote transmission request"]
pub type RTR_R = crate::BitReader<bool>;
#[doc = "Field `RTR` writer - Remote transmission request"]
pub type RTR_W<'a, const O: u8> = crate::BitWriter<'a, u32, SFF_P_SPEC, bool, O>;
#[doc = "Field `FF` reader - Frame format"]
pub type FF_R = crate::BitReader<bool>;
#[doc = "Field `FF` writer - Frame format"]
pub type FF_W<'a, const O: u8> = crate::BitWriter<'a, u32, SFF_P_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:3 - Data length code bit"]
    #[inline(always)]
    pub fn dlc(&self) -> DLC_R {
        DLC_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 6 - Remote transmission request"]
    #[inline(always)]
    pub fn rtr(&self) -> RTR_R {
        RTR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Frame format"]
    #[inline(always)]
    pub fn ff(&self) -> FF_R {
        FF_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Data length code bit"]
    #[inline(always)]
    #[must_use]
    pub fn dlc(&mut self) -> DLC_W<0> {
        DLC_W::new(self)
    }
    #[doc = "Bit 6 - Remote transmission request"]
    #[inline(always)]
    #[must_use]
    pub fn rtr(&mut self) -> RTR_W<6> {
        RTR_W::new(self)
    }
    #[doc = "Bit 7 - Frame format"]
    #[inline(always)]
    #[must_use]
    pub fn ff(&mut self) -> FF_W<7> {
        FF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Peli Send Frame Format register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sff_p](index.html) module"]
pub struct SFF_P_SPEC;
impl crate::RegisterSpec for SFF_P_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sff_p::R](R) reader structure"]
impl crate::Readable for SFF_P_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sff_p::W](W) writer structure"]
impl crate::Writable for SFF_P_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SFF_P to value 0"]
impl crate::Resettable for SFF_P_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
