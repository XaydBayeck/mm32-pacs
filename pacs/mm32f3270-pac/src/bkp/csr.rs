#[doc = "Register `CSR` reader"]
pub struct R(crate::R<CSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CSR` writer"]
pub struct W(crate::W<CSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSR_SPEC>;
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
impl From<crate::W<CSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CTE` writer - Clear tamper event"]
pub type CTE_W<'a, const O: u8> = crate::BitWriter<'a, u16, CSR_SPEC, bool, O>;
#[doc = "Field `CTI` writer - Clear tamper interrupt"]
pub type CTI_W<'a, const O: u8> = crate::BitWriter<'a, u16, CSR_SPEC, bool, O>;
#[doc = "Field `TPIE` reader - TAMPER pin interrupt enable"]
pub type TPIE_R = crate::BitReader<bool>;
#[doc = "Field `TPIE` writer - TAMPER pin interrupt enable"]
pub type TPIE_W<'a, const O: u8> = crate::BitWriter<'a, u16, CSR_SPEC, bool, O>;
#[doc = "Field `TEF` reader - Tamper event flag"]
pub type TEF_R = crate::BitReader<bool>;
#[doc = "Field `TIF` reader - Tamper interrupt flag"]
pub type TIF_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 2 - TAMPER pin interrupt enable"]
    #[inline(always)]
    pub fn tpie(&self) -> TPIE_R {
        TPIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 7 - Tamper event flag"]
    #[inline(always)]
    pub fn tef(&self) -> TEF_R {
        TEF_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Tamper interrupt flag"]
    #[inline(always)]
    pub fn tif(&self) -> TIF_R {
        TIF_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Clear tamper event"]
    #[inline(always)]
    #[must_use]
    pub fn cte(&mut self) -> CTE_W<0> {
        CTE_W::new(self)
    }
    #[doc = "Bit 1 - Clear tamper interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn cti(&mut self) -> CTI_W<1> {
        CTI_W::new(self)
    }
    #[doc = "Bit 2 - TAMPER pin interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn tpie(&mut self) -> TPIE_W<2> {
        TPIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "BKP control/status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csr](index.html) module"]
pub struct CSR_SPEC;
impl crate::RegisterSpec for CSR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [csr::R](R) reader structure"]
impl crate::Readable for CSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [csr::W](W) writer structure"]
impl crate::Writable for CSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CSR to value 0"]
impl crate::Resettable for CSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
