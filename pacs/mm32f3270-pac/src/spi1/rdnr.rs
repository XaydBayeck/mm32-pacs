#[doc = "Register `RDNR` reader"]
pub struct R(crate::R<RDNR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RDNR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RDNR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RDNR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RDNR` writer"]
pub struct W(crate::W<RDNR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RDNR_SPEC>;
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
impl From<crate::W<RDNR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RDNR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RDN` reader - The register is used to hold a count of to be received bytes in next receive process"]
pub type RDN_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RDN` writer - The register is used to hold a count of to be received bytes in next receive process"]
pub type RDN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RDNR_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - The register is used to hold a count of to be received bytes in next receive process"]
    #[inline(always)]
    pub fn rdn(&self) -> RDN_R {
        RDN_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - The register is used to hold a count of to be received bytes in next receive process"]
    #[inline(always)]
    #[must_use]
    pub fn rdn(&mut self) -> RDN_W<0> {
        RDN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Receive data count register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rdnr](index.html) module"]
pub struct RDNR_SPEC;
impl crate::RegisterSpec for RDNR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rdnr::R](R) reader structure"]
impl crate::Readable for RDNR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rdnr::W](W) writer structure"]
impl crate::Writable for RDNR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RDNR to value 0x01"]
impl crate::Resettable for RDNR_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
