#[doc = "Register `FGA0` reader"]
pub struct R(crate::R<FGA0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FGA0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FGA0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FGA0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FGA0` writer"]
pub struct W(crate::W<FGA0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FGA0_SPEC>;
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
impl From<crate::W<FGA0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FGA0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FGA_7_0` reader - Filter group enable"]
pub type FGA_7_0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FGA_7_0` writer - Filter group enable"]
pub type FGA_7_0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FGA0_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Filter group enable"]
    #[inline(always)]
    pub fn fga_7_0(&self) -> FGA_7_0_R {
        FGA_7_0_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Filter group enable"]
    #[inline(always)]
    #[must_use]
    pub fn fga_7_0(&mut self) -> FGA_7_0_W<0> {
        FGA_7_0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Filter Group Enable Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fga0](index.html) module"]
pub struct FGA0_SPEC;
impl crate::RegisterSpec for FGA0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fga0::R](R) reader structure"]
impl crate::Readable for FGA0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fga0::W](W) writer structure"]
impl crate::Writable for FGA0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FGA0 to value 0"]
impl crate::Resettable for FGA0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
