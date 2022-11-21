#[doc = "Register `SR` reader"]
pub struct R(crate::R<SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PVU` reader - Watchdog prescaler value update"]
pub type PVU_R = crate::BitReader<bool>;
#[doc = "Field `RVU` reader - Watchdog counter reload value update"]
pub type RVU_R = crate::BitReader<bool>;
#[doc = "Field `IVU` reader - Watchdog interrupt generate value update"]
pub type IVU_R = crate::BitReader<bool>;
#[doc = "Field `UPDATE` reader - Watchdog Reloads Update Flag"]
pub type UPDATE_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Watchdog prescaler value update"]
    #[inline(always)]
    pub fn pvu(&self) -> PVU_R {
        PVU_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Watchdog counter reload value update"]
    #[inline(always)]
    pub fn rvu(&self) -> RVU_R {
        RVU_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Watchdog interrupt generate value update"]
    #[inline(always)]
    pub fn ivu(&self) -> IVU_R {
        IVU_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Watchdog Reloads Update Flag"]
    #[inline(always)]
    pub fn update(&self) -> UPDATE_R {
        UPDATE_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "Status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](index.html) module"]
pub struct SR_SPEC;
impl crate::RegisterSpec for SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sr::R](R) reader structure"]
impl crate::Readable for SR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
