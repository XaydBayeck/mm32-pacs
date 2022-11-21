#[doc = "Register `SLVMASK` reader"]
pub struct R(crate::R<SLVMASK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SLVMASK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SLVMASK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SLVMASK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SLVMASK` writer"]
pub struct W(crate::W<SLVMASK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SLVMASK_SPEC>;
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
impl From<crate::W<SLVMASK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SLVMASK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `Mask` reader - Slave Address Mask"]
pub type MASK_R = crate::FieldReader<u16, u16>;
#[doc = "Field `Mask` writer - Slave Address Mask"]
pub type MASK_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SLVMASK_SPEC, u16, u16, 10, O>;
impl R {
    #[doc = "Bits 0:9 - Slave Address Mask"]
    #[inline(always)]
    pub fn mask(&self) -> MASK_R {
        MASK_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Slave Address Mask"]
    #[inline(always)]
    #[must_use]
    pub fn mask(&mut self) -> MASK_W<0> {
        MASK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Slave Address Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [slvmask](index.html) module"]
pub struct SLVMASK_SPEC;
impl crate::RegisterSpec for SLVMASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [slvmask::R](R) reader structure"]
impl crate::Readable for SLVMASK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [slvmask::W](W) writer structure"]
impl crate::Writable for SLVMASK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SLVMASK to value 0x03ff"]
impl crate::Resettable for SLVMASK_SPEC {
    const RESET_VALUE: Self::Ux = 0x03ff;
}
