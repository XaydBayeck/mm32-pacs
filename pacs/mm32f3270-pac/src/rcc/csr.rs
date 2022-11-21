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
#[doc = "Field `LSION` reader - Internal low-speed oscillator enable"]
pub type LSION_R = crate::BitReader<bool>;
#[doc = "Field `LSION` writer - Internal low-speed oscillator enable"]
pub type LSION_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, bool, O>;
#[doc = "Field `LSIRDY` reader - Internal low-speed oscillator ready"]
pub type LSIRDY_R = crate::BitReader<bool>;
#[doc = "Field `LSIOENLV` reader - LSI output enable lower voltage"]
pub type LSIOENLV_R = crate::BitReader<bool>;
#[doc = "Field `LSIOENLV` writer - LSI output enable lower voltage"]
pub type LSIOENLV_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, bool, O>;
#[doc = "Field `PVDRSTEN` reader - PVD reset enable"]
pub type PVDRSTEN_R = crate::BitReader<bool>;
#[doc = "Field `PVDRSTEN` writer - PVD reset enable"]
pub type PVDRSTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, bool, O>;
#[doc = "Field `LOCKUPEN` reader - CPU lockup reset enable"]
pub type LOCKUPEN_R = crate::BitReader<bool>;
#[doc = "Field `LOCKUPEN` writer - CPU lockup reset enable"]
pub type LOCKUPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, bool, O>;
#[doc = "Field `VDTRSTNEN` reader - Voltage detect reset enable"]
pub type VDTRSTNEN_R = crate::BitReader<bool>;
#[doc = "Field `VDTRSTNEN` writer - Voltage detect reset enable"]
pub type VDTRSTNEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, bool, O>;
#[doc = "Field `VDTRSTF` reader - Voltage detect reset flag"]
pub type VDTRSTF_R = crate::BitReader<bool>;
#[doc = "Field `PVDRSTF` reader - PVD reset flag"]
pub type PVDRSTF_R = crate::BitReader<bool>;
#[doc = "Field `LOCKUPF` reader - CPU lockup reset flag"]
pub type LOCKUPF_R = crate::BitReader<bool>;
#[doc = "Field `RMVF` writer - Remove reset flag"]
pub type RMVF_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, bool, O>;
#[doc = "Field `PINRSTF` reader - PIN reset flag"]
pub type PINRSTF_R = crate::BitReader<bool>;
#[doc = "Field `PORRSTF` reader - POR/PDR reset flag"]
pub type PORRSTF_R = crate::BitReader<bool>;
#[doc = "Field `SFTRSTF` reader - Software reset flag"]
pub type SFTRSTF_R = crate::BitReader<bool>;
#[doc = "Field `IWDGRSTF` reader - Independent watchdog reset flag"]
pub type IWDGRSTF_R = crate::BitReader<bool>;
#[doc = "Field `WWDGRSTF` reader - Window watchdog reset flag"]
pub type WWDGRSTF_R = crate::BitReader<bool>;
#[doc = "Field `LPWRRSTF` reader - *D31"]
pub type LPWRRSTF_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Internal low-speed oscillator enable"]
    #[inline(always)]
    pub fn lsion(&self) -> LSION_R {
        LSION_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Internal low-speed oscillator ready"]
    #[inline(always)]
    pub fn lsirdy(&self) -> LSIRDY_R {
        LSIRDY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 5 - LSI output enable lower voltage"]
    #[inline(always)]
    pub fn lsioenlv(&self) -> LSIOENLV_R {
        LSIOENLV_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PVD reset enable"]
    #[inline(always)]
    pub fn pvdrsten(&self) -> PVDRSTEN_R {
        PVDRSTEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - CPU lockup reset enable"]
    #[inline(always)]
    pub fn lockupen(&self) -> LOCKUPEN_R {
        LOCKUPEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Voltage detect reset enable"]
    #[inline(always)]
    pub fn vdtrstnen(&self) -> VDTRSTNEN_R {
        VDTRSTNEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 21 - Voltage detect reset flag"]
    #[inline(always)]
    pub fn vdtrstf(&self) -> VDTRSTF_R {
        VDTRSTF_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - PVD reset flag"]
    #[inline(always)]
    pub fn pvdrstf(&self) -> PVDRSTF_R {
        PVDRSTF_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - CPU lockup reset flag"]
    #[inline(always)]
    pub fn lockupf(&self) -> LOCKUPF_R {
        LOCKUPF_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 26 - PIN reset flag"]
    #[inline(always)]
    pub fn pinrstf(&self) -> PINRSTF_R {
        PINRSTF_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - POR/PDR reset flag"]
    #[inline(always)]
    pub fn porrstf(&self) -> PORRSTF_R {
        PORRSTF_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Software reset flag"]
    #[inline(always)]
    pub fn sftrstf(&self) -> SFTRSTF_R {
        SFTRSTF_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Independent watchdog reset flag"]
    #[inline(always)]
    pub fn iwdgrstf(&self) -> IWDGRSTF_R {
        IWDGRSTF_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Window watchdog reset flag"]
    #[inline(always)]
    pub fn wwdgrstf(&self) -> WWDGRSTF_R {
        WWDGRSTF_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - *D31"]
    #[inline(always)]
    pub fn lpwrrstf(&self) -> LPWRRSTF_R {
        LPWRRSTF_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Internal low-speed oscillator enable"]
    #[inline(always)]
    #[must_use]
    pub fn lsion(&mut self) -> LSION_W<0> {
        LSION_W::new(self)
    }
    #[doc = "Bit 5 - LSI output enable lower voltage"]
    #[inline(always)]
    #[must_use]
    pub fn lsioenlv(&mut self) -> LSIOENLV_W<5> {
        LSIOENLV_W::new(self)
    }
    #[doc = "Bit 6 - PVD reset enable"]
    #[inline(always)]
    #[must_use]
    pub fn pvdrsten(&mut self) -> PVDRSTEN_W<6> {
        PVDRSTEN_W::new(self)
    }
    #[doc = "Bit 7 - CPU lockup reset enable"]
    #[inline(always)]
    #[must_use]
    pub fn lockupen(&mut self) -> LOCKUPEN_W<7> {
        LOCKUPEN_W::new(self)
    }
    #[doc = "Bit 8 - Voltage detect reset enable"]
    #[inline(always)]
    #[must_use]
    pub fn vdtrstnen(&mut self) -> VDTRSTNEN_W<8> {
        VDTRSTNEN_W::new(self)
    }
    #[doc = "Bit 24 - Remove reset flag"]
    #[inline(always)]
    #[must_use]
    pub fn rmvf(&mut self) -> RMVF_W<24> {
        RMVF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csr](index.html) module"]
pub struct CSR_SPEC;
impl crate::RegisterSpec for CSR_SPEC {
    type Ux = u32;
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
#[doc = "`reset()` method sets CSR to value 0x0c00_0000"]
impl crate::Resettable for CSR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0c00_0000;
}
