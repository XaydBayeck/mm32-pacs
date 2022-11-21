#[doc = "Register `FSHR` reader"]
pub struct R(crate::R<FSHR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FSHR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FSHR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FSHR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FSHR` writer"]
pub struct W(crate::W<FSHR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FSHR_SPEC>;
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
impl From<crate::W<FSHR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FSHR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CNT` reader - This register sets the SCL clock high_period count for standard speed"]
pub type CNT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CNT` writer - This register sets the SCL clock high_period count for standard speed"]
pub type CNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FSHR_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - This register sets the SCL clock high_period count for standard speed"]
    #[inline(always)]
    pub fn cnt(&self) -> CNT_R {
        CNT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - This register sets the SCL clock high_period count for standard speed"]
    #[inline(always)]
    #[must_use]
    pub fn cnt(&mut self) -> CNT_W<0> {
        CNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SCL High Period Count for Fast Speed Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fshr](index.html) module"]
pub struct FSHR_SPEC;
impl crate::RegisterSpec for FSHR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fshr::R](R) reader structure"]
impl crate::Readable for FSHR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fshr::W](W) writer structure"]
impl crate::Writable for FSHR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FSHR to value 0x3c"]
impl crate::Resettable for FSHR_SPEC {
    const RESET_VALUE: Self::Ux = 0x3c;
}
