#[doc = "Register `MIIDR` reader"]
pub struct R(crate::R<MIIDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MIIDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MIIDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MIIDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MIIDR` writer"]
pub struct W(crate::W<MIIDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MIIDR_SPEC>;
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
impl From<crate::W<MIIDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MIIDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MD` reader - MII data"]
pub type MD_R = crate::FieldReader<u16, u16>;
#[doc = "Field `MD` writer - MII data"]
pub type MD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MIIDR_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - MII data"]
    #[inline(always)]
    pub fn md(&self) -> MD_R {
        MD_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - MII data"]
    #[inline(always)]
    #[must_use]
    pub fn md(&mut self) -> MD_W<0> {
        MD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ethernet MAC MII data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [miidr](index.html) module"]
pub struct MIIDR_SPEC;
impl crate::RegisterSpec for MIIDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [miidr::R](R) reader structure"]
impl crate::Readable for MIIDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [miidr::W](W) writer structure"]
impl crate::Writable for MIIDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MIIDR to value 0"]
impl crate::Resettable for MIIDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
