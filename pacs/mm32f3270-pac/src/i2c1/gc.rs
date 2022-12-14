#[doc = "Register `GC` reader"]
pub struct R(crate::R<GC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `GC` reader - Read this register to clear the GEN_CALL interrupt(bit 11)of the RAWISR register"]
pub type GC_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Read this register to clear the GEN_CALL interrupt(bit 11)of the RAWISR register"]
    #[inline(always)]
    pub fn gc(&self) -> GC_R {
        GC_R::new((self.bits & 1) != 0)
    }
}
#[doc = "Clear GEN_CALL Interrupt Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gc](index.html) module\n\nThe register is **cleared** (set to zero) following a read operation."]
pub struct GC_SPEC;
impl crate::RegisterSpec for GC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gc::R](R) reader structure"]
impl crate::Readable for GC_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets GC to value 0"]
impl crate::Resettable for GC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
