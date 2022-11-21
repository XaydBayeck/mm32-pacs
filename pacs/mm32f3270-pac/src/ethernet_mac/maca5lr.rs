#[doc = "Register `MACA5LR` reader"]
pub struct R(crate::R<MACA5LR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MACA5LR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MACA5LR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MACA5LR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MACA5LR` writer"]
pub struct W(crate::W<MACA5LR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MACA5LR_SPEC>;
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
impl From<crate::W<MACA5LR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MACA5LR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADDL` reader - MAC address5 low \\[31:0\\]"]
pub type ADDL_R = crate::FieldReader<u32, u32>;
#[doc = "Field `ADDL` writer - MAC address5 low \\[31:0\\]"]
pub type ADDL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MACA5LR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - MAC address5 low \\[31:0\\]"]
    #[inline(always)]
    pub fn addl(&self) -> ADDL_R {
        ADDL_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - MAC address5 low \\[31:0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn addl(&mut self) -> ADDL_W<0> {
        ADDL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ethernet MAC address5 low register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [maca5lr](index.html) module"]
pub struct MACA5LR_SPEC;
impl crate::RegisterSpec for MACA5LR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [maca5lr::R](R) reader structure"]
impl crate::Readable for MACA5LR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [maca5lr::W](W) writer structure"]
impl crate::Writable for MACA5LR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MACA5LR to value 0"]
impl crate::Resettable for MACA5LR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
