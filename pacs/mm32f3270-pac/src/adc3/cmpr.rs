#[doc = "Register `CMPR` reader"]
pub struct R(crate::R<CMPR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMPR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMPR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMPR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CMPR` writer"]
pub struct W(crate::W<CMPR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMPR_SPEC>;
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
impl From<crate::W<CMPR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMPR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMPLDATA` reader - ADC 12bit window compare DOWN LEVEL DATA"]
pub type CMPLDATA_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CMPLDATA` writer - ADC 12bit window compare DOWN LEVEL DATA"]
pub type CMPLDATA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CMPR_SPEC, u16, u16, 12, O>;
#[doc = "Field `CMPHDATA` reader - ADC 12bit window compare UP LEVEL DATA"]
pub type CMPHDATA_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CMPHDATA` writer - ADC 12bit window compare UP LEVEL DATA"]
pub type CMPHDATA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CMPR_SPEC, u16, u16, 12, O>;
impl R {
    #[doc = "Bits 0:11 - ADC 12bit window compare DOWN LEVEL DATA"]
    #[inline(always)]
    pub fn cmpldata(&self) -> CMPLDATA_R {
        CMPLDATA_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - ADC 12bit window compare UP LEVEL DATA"]
    #[inline(always)]
    pub fn cmphdata(&self) -> CMPHDATA_R {
        CMPHDATA_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - ADC 12bit window compare DOWN LEVEL DATA"]
    #[inline(always)]
    #[must_use]
    pub fn cmpldata(&mut self) -> CMPLDATA_W<0> {
        CMPLDATA_W::new(self)
    }
    #[doc = "Bits 16:27 - ADC 12bit window compare UP LEVEL DATA"]
    #[inline(always)]
    #[must_use]
    pub fn cmphdata(&mut self) -> CMPHDATA_W<16> {
        CMPHDATA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Compare register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmpr](index.html) module"]
pub struct CMPR_SPEC;
impl crate::RegisterSpec for CMPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmpr::R](R) reader structure"]
impl crate::Readable for CMPR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cmpr::W](W) writer structure"]
impl crate::Writable for CMPR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CMPR to value 0"]
impl crate::Resettable for CMPR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
