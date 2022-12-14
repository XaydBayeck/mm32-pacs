#[doc = "Register `TAR` reader"]
pub struct R(crate::R<TAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TAR` writer"]
pub struct W(crate::W<TAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TAR_SPEC>;
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
impl From<crate::W<TAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADDR` reader - This is the target address for any master transaction"]
pub type ADDR_R = crate::FieldReader<u16, u16>;
#[doc = "Field `ADDR` writer - This is the target address for any master transaction"]
pub type ADDR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TAR_SPEC, u16, u16, 10, O>;
#[doc = "Field `GC` reader - If bit 11(SPECIAL)is set to 1"]
pub type GC_R = crate::BitReader<bool>;
#[doc = "Field `GC` writer - If bit 11(SPECIAL)is set to 1"]
pub type GC_W<'a, const O: u8> = crate::BitWriter<'a, u32, TAR_SPEC, bool, O>;
#[doc = "Field `SPECIAL` reader - This bit indicates whether software performs a General Call or START BYTE conmmend"]
pub type SPECIAL_R = crate::BitReader<bool>;
#[doc = "Field `SPECIAL` writer - This bit indicates whether software performs a General Call or START BYTE conmmend"]
pub type SPECIAL_W<'a, const O: u8> = crate::BitWriter<'a, u32, TAR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:9 - This is the target address for any master transaction"]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 10 - If bit 11(SPECIAL)is set to 1"]
    #[inline(always)]
    pub fn gc(&self) -> GC_R {
        GC_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - This bit indicates whether software performs a General Call or START BYTE conmmend"]
    #[inline(always)]
    pub fn special(&self) -> SPECIAL_R {
        SPECIAL_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - This is the target address for any master transaction"]
    #[inline(always)]
    #[must_use]
    pub fn addr(&mut self) -> ADDR_W<0> {
        ADDR_W::new(self)
    }
    #[doc = "Bit 10 - If bit 11(SPECIAL)is set to 1"]
    #[inline(always)]
    #[must_use]
    pub fn gc(&mut self) -> GC_W<10> {
        GC_W::new(self)
    }
    #[doc = "Bit 11 - This bit indicates whether software performs a General Call or START BYTE conmmend"]
    #[inline(always)]
    #[must_use]
    pub fn special(&mut self) -> SPECIAL_W<11> {
        SPECIAL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Target Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tar](index.html) module"]
pub struct TAR_SPEC;
impl crate::RegisterSpec for TAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tar::R](R) reader structure"]
impl crate::Readable for TAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tar::W](W) writer structure"]
impl crate::Writable for TAR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TAR to value 0x55"]
impl crate::Resettable for TAR_SPEC {
    const RESET_VALUE: Self::Ux = 0x55;
}
