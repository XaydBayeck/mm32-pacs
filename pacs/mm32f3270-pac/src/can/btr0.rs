#[doc = "Register `BTR0` reader"]
pub struct R(crate::R<BTR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BTR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BTR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BTR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BTR0` writer"]
pub struct W(crate::W<BTR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BTR0_SPEC>;
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
impl From<crate::W<BTR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BTR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BRP` reader - Baud rate prescaler"]
pub type BRP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BRP` writer - Baud rate prescaler"]
pub type BRP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BTR0_SPEC, u8, u8, 6, O>;
#[doc = "Field `SJW` reader - Synchronization jump width"]
pub type SJW_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SJW` writer - Synchronization jump width"]
pub type SJW_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BTR0_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:5 - Baud rate prescaler"]
    #[inline(always)]
    pub fn brp(&self) -> BRP_R {
        BRP_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:7 - Synchronization jump width"]
    #[inline(always)]
    pub fn sjw(&self) -> SJW_R {
        SJW_R::new(((self.bits >> 6) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Baud rate prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn brp(&mut self) -> BRP_W<0> {
        BRP_W::new(self)
    }
    #[doc = "Bits 6:7 - Synchronization jump width"]
    #[inline(always)]
    #[must_use]
    pub fn sjw(&mut self) -> SJW_W<6> {
        SJW_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Bus Timing register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [btr0](index.html) module"]
pub struct BTR0_SPEC;
impl crate::RegisterSpec for BTR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [btr0::R](R) reader structure"]
impl crate::Readable for BTR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [btr0::W](W) writer structure"]
impl crate::Writable for BTR0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BTR0 to value 0"]
impl crate::Resettable for BTR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
