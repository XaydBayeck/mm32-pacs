#[doc = "Register `CCR` reader"]
pub struct R(crate::R<CCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CCR` writer"]
pub struct W(crate::W<CCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCR_SPEC>;
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
impl From<crate::W<CCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EN` reader - Enable cache"]
pub type EN_R = crate::BitReader<bool>;
#[doc = "Field `EN` writer - Enable cache"]
pub type EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR_SPEC, bool, O>;
#[doc = "Field `INV_REQ` reader - Invalid manual set request"]
pub type INV_REQ_R = crate::BitReader<bool>;
#[doc = "Field `INV_REQ` writer - Invalid manual set request"]
pub type INV_REQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR_SPEC, bool, O>;
#[doc = "Field `POW_REQ` reader - Manual SRAM power request"]
pub type POW_REQ_R = crate::BitReader<bool>;
#[doc = "Field `POW_REQ` writer - Manual SRAM power request"]
pub type POW_REQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR_SPEC, bool, O>;
#[doc = "Field `SET_MAN_POW` reader - Set manual or automatic SRAM power request"]
pub type SET_MAN_POW_R = crate::BitReader<bool>;
#[doc = "Field `SET_MAN_POW` writer - Set manual or automatic SRAM power request"]
pub type SET_MAN_POW_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR_SPEC, bool, O>;
#[doc = "Field `SET_MAN_INV` reader - Set manual or automatic invalid"]
pub type SET_MAN_INV_R = crate::BitReader<bool>;
#[doc = "Field `SET_MAN_INV` writer - Set manual or automatic invalid"]
pub type SET_MAN_INV_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR_SPEC, bool, O>;
#[doc = "Field `SET_PREFETCH` reader - Prefetch value"]
pub type SET_PREFETCH_R = crate::BitReader<bool>;
#[doc = "Field `SET_PREFETCH` writer - Prefetch value"]
pub type SET_PREFETCH_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR_SPEC, bool, O>;
#[doc = "Field `STATISTIC_EN` reader - Enable statistics"]
pub type STATISTIC_EN_R = crate::BitReader<bool>;
#[doc = "Field `STATISTIC_EN` writer - Enable statistics"]
pub type STATISTIC_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Enable cache"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Invalid manual set request"]
    #[inline(always)]
    pub fn inv_req(&self) -> INV_REQ_R {
        INV_REQ_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Manual SRAM power request"]
    #[inline(always)]
    pub fn pow_req(&self) -> POW_REQ_R {
        POW_REQ_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Set manual or automatic SRAM power request"]
    #[inline(always)]
    pub fn set_man_pow(&self) -> SET_MAN_POW_R {
        SET_MAN_POW_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Set manual or automatic invalid"]
    #[inline(always)]
    pub fn set_man_inv(&self) -> SET_MAN_INV_R {
        SET_MAN_INV_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Prefetch value"]
    #[inline(always)]
    pub fn set_prefetch(&self) -> SET_PREFETCH_R {
        SET_PREFETCH_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable statistics"]
    #[inline(always)]
    pub fn statistic_en(&self) -> STATISTIC_EN_R {
        STATISTIC_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable cache"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<0> {
        EN_W::new(self)
    }
    #[doc = "Bit 1 - Invalid manual set request"]
    #[inline(always)]
    #[must_use]
    pub fn inv_req(&mut self) -> INV_REQ_W<1> {
        INV_REQ_W::new(self)
    }
    #[doc = "Bit 2 - Manual SRAM power request"]
    #[inline(always)]
    #[must_use]
    pub fn pow_req(&mut self) -> POW_REQ_W<2> {
        POW_REQ_W::new(self)
    }
    #[doc = "Bit 3 - Set manual or automatic SRAM power request"]
    #[inline(always)]
    #[must_use]
    pub fn set_man_pow(&mut self) -> SET_MAN_POW_W<3> {
        SET_MAN_POW_W::new(self)
    }
    #[doc = "Bit 4 - Set manual or automatic invalid"]
    #[inline(always)]
    #[must_use]
    pub fn set_man_inv(&mut self) -> SET_MAN_INV_W<4> {
        SET_MAN_INV_W::new(self)
    }
    #[doc = "Bit 5 - Prefetch value"]
    #[inline(always)]
    #[must_use]
    pub fn set_prefetch(&mut self) -> SET_PREFETCH_W<5> {
        SET_PREFETCH_W::new(self)
    }
    #[doc = "Bit 6 - Enable statistics"]
    #[inline(always)]
    #[must_use]
    pub fn statistic_en(&mut self) -> STATISTIC_EN_W<6> {
        STATISTIC_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configuration and control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccr](index.html) module"]
pub struct CCR_SPEC;
impl crate::RegisterSpec for CCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ccr::R](R) reader structure"]
impl crate::Readable for CCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ccr::W](W) writer structure"]
impl crate::Writable for CCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CCR to value 0x40"]
impl crate::Resettable for CCR_SPEC {
    const RESET_VALUE: Self::Ux = 0x40;
}
