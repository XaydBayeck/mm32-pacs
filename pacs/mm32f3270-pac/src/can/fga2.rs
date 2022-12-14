#[doc = "Register `FGA2` reader"]
pub struct R(crate::R<FGA2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FGA2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FGA2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FGA2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FGA2` writer"]
pub struct W(crate::W<FGA2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FGA2_SPEC>;
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
impl From<crate::W<FGA2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FGA2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FGA_19_16` reader - Filter group enable"]
pub type FGA_19_16_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FGA_19_16` writer - Filter group enable"]
pub type FGA_19_16_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FGA2_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3 - Filter group enable"]
    #[inline(always)]
    pub fn fga_19_16(&self) -> FGA_19_16_R {
        FGA_19_16_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Filter group enable"]
    #[inline(always)]
    #[must_use]
    pub fn fga_19_16(&mut self) -> FGA_19_16_W<0> {
        FGA_19_16_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Filter Group Enable Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fga2](index.html) module"]
pub struct FGA2_SPEC;
impl crate::RegisterSpec for FGA2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fga2::R](R) reader structure"]
impl crate::Readable for FGA2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fga2::W](W) writer structure"]
impl crate::Writable for FGA2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FGA2 to value 0"]
impl crate::Resettable for FGA2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
