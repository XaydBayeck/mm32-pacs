#[doc = "Register `IMR` reader"]
pub struct R(crate::R<IMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IMR` writer"]
pub struct W(crate::W<IMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IMR_SPEC>;
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
impl From<crate::W<IMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `POW_ERR` reader - Power error interrupt request mask"]
pub type POW_ERR_R = crate::BitReader<bool>;
#[doc = "Field `POW_ERR` writer - Power error interrupt request mask"]
pub type POW_ERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR_SPEC, bool, O>;
#[doc = "Field `MAN_INV_ERR` reader - Invalid manual error interrupt request mask"]
pub type MAN_INV_ERR_R = crate::BitReader<bool>;
#[doc = "Field `MAN_INV_ERR` writer - Invalid manual error interrupt request mask"]
pub type MAN_INV_ERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Power error interrupt request mask"]
    #[inline(always)]
    pub fn pow_err(&self) -> POW_ERR_R {
        POW_ERR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Invalid manual error interrupt request mask"]
    #[inline(always)]
    pub fn man_inv_err(&self) -> MAN_INV_ERR_R {
        MAN_INV_ERR_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Power error interrupt request mask"]
    #[inline(always)]
    #[must_use]
    pub fn pow_err(&mut self) -> POW_ERR_W<0> {
        POW_ERR_W::new(self)
    }
    #[doc = "Bit 1 - Invalid manual error interrupt request mask"]
    #[inline(always)]
    #[must_use]
    pub fn man_inv_err(&mut self) -> MAN_INV_ERR_W<1> {
        MAN_INV_ERR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [imr](index.html) module"]
pub struct IMR_SPEC;
impl crate::RegisterSpec for IMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [imr::R](R) reader structure"]
impl crate::Readable for IMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [imr::W](W) writer structure"]
impl crate::Writable for IMR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IMR to value 0"]
impl crate::Resettable for IMR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
