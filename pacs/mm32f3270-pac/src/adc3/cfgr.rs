#[doc = "Register `CFGR` reader"]
pub struct R(crate::R<CFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFGR` writer"]
pub struct W(crate::W<CFGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFGR_SPEC>;
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
impl From<crate::W<CFGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADEN` reader - ADC enable"]
pub type ADEN_R = crate::BitReader<bool>;
#[doc = "Field `ADEN` writer - ADC enable"]
pub type ADEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR_SPEC, bool, O>;
#[doc = "Field `ADWEN` reader - ADC window comparison enable"]
pub type ADWEN_R = crate::BitReader<bool>;
#[doc = "Field `ADWEN` writer - ADC window comparison enable"]
pub type ADWEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR_SPEC, bool, O>;
#[doc = "Field `TSEN` reader - Temperature sensor enable"]
pub type TSEN_R = crate::BitReader<bool>;
#[doc = "Field `TSEN` writer - Temperature sensor enable"]
pub type TSEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR_SPEC, bool, O>;
#[doc = "Field `VSEN` reader - Voltage Sensor enable"]
pub type VSEN_R = crate::BitReader<bool>;
#[doc = "Field `VSEN` writer - Voltage Sensor enable"]
pub type VSEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR_SPEC, bool, O>;
#[doc = "Field `ADCPRE_H` reader - ADC prescaler_h"]
pub type ADCPRE_H_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADCPRE_H` writer - ADC prescaler_h"]
pub type ADCPRE_H_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFGR_SPEC, u8, u8, 3, O>;
#[doc = "Field `RSLTCTL` reader - Resolution"]
pub type RSLTCTL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RSLTCTL` writer - Resolution"]
pub type RSLTCTL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFGR_SPEC, u8, u8, 3, O>;
#[doc = "Field `ADCPRE_L` reader - ADC prescaler_l"]
pub type ADCPRE_L_R = crate::BitReader<bool>;
#[doc = "Field `ADCPRE_L` writer - ADC prescaler_l"]
pub type ADCPRE_L_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR_SPEC, bool, O>;
#[doc = "Field `JADWEN` reader - Inject ADC conversion window comparison enable"]
pub type JADWEN_R = crate::BitReader<bool>;
#[doc = "Field `JADWEN` writer - Inject ADC conversion window comparison enable"]
pub type JADWEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - ADC enable"]
    #[inline(always)]
    pub fn aden(&self) -> ADEN_R {
        ADEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ADC window comparison enable"]
    #[inline(always)]
    pub fn adwen(&self) -> ADWEN_R {
        ADWEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Temperature sensor enable"]
    #[inline(always)]
    pub fn tsen(&self) -> TSEN_R {
        TSEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Voltage Sensor enable"]
    #[inline(always)]
    pub fn vsen(&self) -> VSEN_R {
        VSEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - ADC prescaler_h"]
    #[inline(always)]
    pub fn adcpre_h(&self) -> ADCPRE_H_R {
        ADCPRE_H_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 7:9 - Resolution"]
    #[inline(always)]
    pub fn rsltctl(&self) -> RSLTCTL_R {
        RSLTCTL_R::new(((self.bits >> 7) & 7) as u8)
    }
    #[doc = "Bit 14 - ADC prescaler_l"]
    #[inline(always)]
    pub fn adcpre_l(&self) -> ADCPRE_L_R {
        ADCPRE_L_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - Inject ADC conversion window comparison enable"]
    #[inline(always)]
    pub fn jadwen(&self) -> JADWEN_R {
        JADWEN_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ADC enable"]
    #[inline(always)]
    #[must_use]
    pub fn aden(&mut self) -> ADEN_W<0> {
        ADEN_W::new(self)
    }
    #[doc = "Bit 1 - ADC window comparison enable"]
    #[inline(always)]
    #[must_use]
    pub fn adwen(&mut self) -> ADWEN_W<1> {
        ADWEN_W::new(self)
    }
    #[doc = "Bit 2 - Temperature sensor enable"]
    #[inline(always)]
    #[must_use]
    pub fn tsen(&mut self) -> TSEN_W<2> {
        TSEN_W::new(self)
    }
    #[doc = "Bit 3 - Voltage Sensor enable"]
    #[inline(always)]
    #[must_use]
    pub fn vsen(&mut self) -> VSEN_W<3> {
        VSEN_W::new(self)
    }
    #[doc = "Bits 4:6 - ADC prescaler_h"]
    #[inline(always)]
    #[must_use]
    pub fn adcpre_h(&mut self) -> ADCPRE_H_W<4> {
        ADCPRE_H_W::new(self)
    }
    #[doc = "Bits 7:9 - Resolution"]
    #[inline(always)]
    #[must_use]
    pub fn rsltctl(&mut self) -> RSLTCTL_W<7> {
        RSLTCTL_W::new(self)
    }
    #[doc = "Bit 14 - ADC prescaler_l"]
    #[inline(always)]
    #[must_use]
    pub fn adcpre_l(&mut self) -> ADCPRE_L_W<14> {
        ADCPRE_L_W::new(self)
    }
    #[doc = "Bit 16 - Inject ADC conversion window comparison enable"]
    #[inline(always)]
    #[must_use]
    pub fn jadwen(&mut self) -> JADWEN_W<16> {
        JADWEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configure register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfgr](index.html) module"]
pub struct CFGR_SPEC;
impl crate::RegisterSpec for CFGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfgr::R](R) reader structure"]
impl crate::Readable for CFGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfgr::W](W) writer structure"]
impl crate::Writable for CFGR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFGR to value 0"]
impl crate::Resettable for CFGR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
