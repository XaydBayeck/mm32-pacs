#[doc = "Register `HTHR` reader"]
pub struct R(crate::R<HTHR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HTHR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HTHR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HTHR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HTHR` writer"]
pub struct W(crate::W<HTHR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HTHR_SPEC>;
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
impl From<crate::W<HTHR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HTHR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HTABH` reader - Hash Table High"]
pub type HTABH_R = crate::FieldReader<u32, u32>;
#[doc = "Field `HTABH` writer - Hash Table High"]
pub type HTABH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HTHR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Hash Table High"]
    #[inline(always)]
    pub fn htabh(&self) -> HTABH_R {
        HTABH_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Hash Table High"]
    #[inline(always)]
    #[must_use]
    pub fn htabh(&mut self) -> HTABH_W<0> {
        HTABH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ethernet MAC hash table high register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hthr](index.html) module"]
pub struct HTHR_SPEC;
impl crate::RegisterSpec for HTHR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hthr::R](R) reader structure"]
impl crate::Readable for HTHR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hthr::W](W) writer structure"]
impl crate::Writable for HTHR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HTHR to value 0"]
impl crate::Resettable for HTHR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
