#[doc = "Register `BTR1` reader"]
pub struct R(crate::R<BTR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BTR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BTR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BTR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BTR1` writer"]
pub struct W(crate::W<BTR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BTR1_SPEC>;
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
impl From<crate::W<BTR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BTR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TSEG1` reader - Time segment 1"]
pub type TSEG1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TSEG1` writer - Time segment 1"]
pub type TSEG1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BTR1_SPEC, u8, u8, 4, O>;
#[doc = "Field `TSEG2` reader - Time segment 2"]
pub type TSEG2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TSEG2` writer - Time segment 2"]
pub type TSEG2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BTR1_SPEC, u8, u8, 3, O>;
#[doc = "Field `SAM` reader - Sampling"]
pub type SAM_R = crate::BitReader<bool>;
#[doc = "Field `SAM` writer - Sampling"]
pub type SAM_W<'a, const O: u8> = crate::BitWriter<'a, u32, BTR1_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:3 - Time segment 1"]
    #[inline(always)]
    pub fn tseg1(&self) -> TSEG1_R {
        TSEG1_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:6 - Time segment 2"]
    #[inline(always)]
    pub fn tseg2(&self) -> TSEG2_R {
        TSEG2_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - Sampling"]
    #[inline(always)]
    pub fn sam(&self) -> SAM_R {
        SAM_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Time segment 1"]
    #[inline(always)]
    #[must_use]
    pub fn tseg1(&mut self) -> TSEG1_W<0> {
        TSEG1_W::new(self)
    }
    #[doc = "Bits 4:6 - Time segment 2"]
    #[inline(always)]
    #[must_use]
    pub fn tseg2(&mut self) -> TSEG2_W<4> {
        TSEG2_W::new(self)
    }
    #[doc = "Bit 7 - Sampling"]
    #[inline(always)]
    #[must_use]
    pub fn sam(&mut self) -> SAM_W<7> {
        SAM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Bus Timing register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [btr1](index.html) module"]
pub struct BTR1_SPEC;
impl crate::RegisterSpec for BTR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [btr1::R](R) reader structure"]
impl crate::Readable for BTR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [btr1::W](W) writer structure"]
impl crate::Writable for BTR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BTR1 to value 0"]
impl crate::Resettable for BTR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
