#[doc = "Register `COMP_CRV` reader"]
pub struct R(crate::R<COMP_CRV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COMP_CRV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COMP_CRV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COMP_CRV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `COMP_CRV` writer"]
pub struct W(crate::W<COMP_CRV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<COMP_CRV_SPEC>;
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
impl From<crate::W<COMP_CRV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<COMP_CRV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CRV_SEL` reader - Comparator external reference voltage select"]
pub type CRV_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CRV_SEL` writer - Comparator external reference voltage select"]
pub type CRV_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, COMP_CRV_SPEC, u8, u8, 4, O>;
#[doc = "Field `CRV_EN` reader - Comparator external reference voltage enable"]
pub type CRV_EN_R = crate::BitReader<bool>;
#[doc = "Field `CRV_EN` writer - Comparator external reference voltage enable"]
pub type CRV_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, COMP_CRV_SPEC, bool, O>;
#[doc = "Field `CRV_SRC` reader - Comparator external reference voltage source select"]
pub type CRV_SRC_R = crate::BitReader<bool>;
#[doc = "Field `CRV_SRC` writer - Comparator external reference voltage source select"]
pub type CRV_SRC_W<'a, const O: u8> = crate::BitWriter<'a, u32, COMP_CRV_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:3 - Comparator external reference voltage select"]
    #[inline(always)]
    pub fn crv_sel(&self) -> CRV_SEL_R {
        CRV_SEL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Comparator external reference voltage enable"]
    #[inline(always)]
    pub fn crv_en(&self) -> CRV_EN_R {
        CRV_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Comparator external reference voltage source select"]
    #[inline(always)]
    pub fn crv_src(&self) -> CRV_SRC_R {
        CRV_SRC_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Comparator external reference voltage select"]
    #[inline(always)]
    #[must_use]
    pub fn crv_sel(&mut self) -> CRV_SEL_W<0> {
        CRV_SEL_W::new(self)
    }
    #[doc = "Bit 4 - Comparator external reference voltage enable"]
    #[inline(always)]
    #[must_use]
    pub fn crv_en(&mut self) -> CRV_EN_W<4> {
        CRV_EN_W::new(self)
    }
    #[doc = "Bit 5 - Comparator external reference voltage source select"]
    #[inline(always)]
    #[must_use]
    pub fn crv_src(&mut self) -> CRV_SRC_W<5> {
        CRV_SRC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "COMP Extern Reference Voltage\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [comp_crv](index.html) module"]
pub struct COMP_CRV_SPEC;
impl crate::RegisterSpec for COMP_CRV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [comp_crv::R](R) reader structure"]
impl crate::Readable for COMP_CRV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [comp_crv::W](W) writer structure"]
impl crate::Writable for COMP_CRV_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets COMP_CRV to value 0"]
impl crate::Resettable for COMP_CRV_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
