#[doc = "Register `CFGR1` reader"]
pub struct R(crate::R<CFGR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFGR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFGR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFGR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFGR1` writer"]
pub struct W(crate::W<CFGR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFGR1_SPEC>;
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
impl From<crate::W<CFGR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFGR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MEM_MODE` reader - Memory selection bit is set and cleared by software"]
pub type MEM_MODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MEM_MODE` writer - Memory selection bit is set and cleared by software"]
pub type MEM_MODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFGR1_SPEC, u8, u8, 2, O>;
#[doc = "Field `FC_SYNC_EN` reader - FSMC synchronization enable"]
pub type FC_SYNC_EN_R = crate::BitReader<bool>;
#[doc = "Field `FC_SYNC_EN` writer - FSMC synchronization enable"]
pub type FC_SYNC_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR1_SPEC, bool, O>;
#[doc = "Field `FC_ODATA_EN` reader - FSMC address data multiplexing pin can only be used as data"]
pub type FC_ODATA_EN_R = crate::BitReader<bool>;
#[doc = "Field `FC_ODATA_EN` writer - FSMC address data multiplexing pin can only be used as data"]
pub type FC_ODATA_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR1_SPEC, bool, O>;
#[doc = "Field `MODESEL` reader - FSMC mode selection"]
pub type MODESEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MODESEL` writer - FSMC mode selection"]
pub type MODESEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFGR1_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:1 - Memory selection bit is set and cleared by software"]
    #[inline(always)]
    pub fn mem_mode(&self) -> MEM_MODE_R {
        MEM_MODE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 27 - FSMC synchronization enable"]
    #[inline(always)]
    pub fn fc_sync_en(&self) -> FC_SYNC_EN_R {
        FC_SYNC_EN_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - FSMC address data multiplexing pin can only be used as data"]
    #[inline(always)]
    pub fn fc_odata_en(&self) -> FC_ODATA_EN_R {
        FC_ODATA_EN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bits 29:30 - FSMC mode selection"]
    #[inline(always)]
    pub fn modesel(&self) -> MODESEL_R {
        MODESEL_R::new(((self.bits >> 29) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Memory selection bit is set and cleared by software"]
    #[inline(always)]
    #[must_use]
    pub fn mem_mode(&mut self) -> MEM_MODE_W<0> {
        MEM_MODE_W::new(self)
    }
    #[doc = "Bit 27 - FSMC synchronization enable"]
    #[inline(always)]
    #[must_use]
    pub fn fc_sync_en(&mut self) -> FC_SYNC_EN_W<27> {
        FC_SYNC_EN_W::new(self)
    }
    #[doc = "Bit 28 - FSMC address data multiplexing pin can only be used as data"]
    #[inline(always)]
    #[must_use]
    pub fn fc_odata_en(&mut self) -> FC_ODATA_EN_W<28> {
        FC_ODATA_EN_W::new(self)
    }
    #[doc = "Bits 29:30 - FSMC mode selection"]
    #[inline(always)]
    #[must_use]
    pub fn modesel(&mut self) -> MODESEL_W<29> {
        MODESEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SYSCFG configuration register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfgr1](index.html) module"]
pub struct CFGR1_SPEC;
impl crate::RegisterSpec for CFGR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfgr1::R](R) reader structure"]
impl crate::Readable for CFGR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfgr1::W](W) writer structure"]
impl crate::Writable for CFGR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFGR1 to value 0"]
impl crate::Resettable for CFGR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
