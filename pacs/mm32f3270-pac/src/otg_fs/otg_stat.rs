#[doc = "Register `OTG_STAT` reader"]
pub struct R(crate::R<OTG_STAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OTG_STAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OTG_STAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OTG_STAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OTG_STAT` writer"]
pub struct W(crate::W<OTG_STAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OTG_STAT_SPEC>;
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
impl From<crate::W<OTG_STAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OTG_STAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `A_VBUS_VLD` reader - When the VBUS voltage is higher than the VBUS effective threshold of a device, this bit will be set bit"]
pub type A_VBUS_VLD_R = crate::BitReader<bool>;
#[doc = "Field `A_VBUS_VLD` writer - When the VBUS voltage is higher than the VBUS effective threshold of a device, this bit will be set bit"]
pub type A_VBUS_VLD_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTG_STAT_SPEC, bool, O>;
#[doc = "Field `B_SESS_END` reader - This bit is set when the VBUS voltage is below the B device session end threshold"]
pub type B_SESS_END_R = crate::BitReader<bool>;
#[doc = "Field `B_SESS_END` writer - This bit is set when the VBUS voltage is below the B device session end threshold"]
pub type B_SESS_END_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTG_STAT_SPEC, bool, O>;
#[doc = "Field `SESS_VLD` reader - This bit is set when the VBUS voltage is higher than the valid threshold of the B device session bit"]
pub type SESS_VLD_R = crate::BitReader<bool>;
#[doc = "Field `SESS_VLD` writer - This bit is set when the VBUS voltage is higher than the valid threshold of the B device session bit"]
pub type SESS_VLD_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTG_STAT_SPEC, bool, O>;
#[doc = "Field `LINE_STATE_STABLE` reader - Dithering"]
pub type LINE_STATE_STABLE_R = crate::BitReader<bool>;
#[doc = "Field `LINE_STATE_STABLE` writer - Dithering"]
pub type LINE_STATE_STABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTG_STAT_SPEC, bool, O>;
#[doc = "Field `_1MSEC` reader - This bit is reserved for the 1 ms counter and is not valid for software"]
pub type _1MSEC_R = crate::BitReader<bool>;
#[doc = "Field `_1MSEC` writer - This bit is reserved for the 1 ms counter and is not valid for software"]
pub type _1MSEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTG_STAT_SPEC, bool, O>;
#[doc = "Field `ID` reader - Indicates the current status of the ID pin on the USB connector"]
pub type ID_R = crate::BitReader<bool>;
#[doc = "Field `ID` writer - Indicates the current status of the ID pin on the USB connector"]
pub type ID_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTG_STAT_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - When the VBUS voltage is higher than the VBUS effective threshold of a device, this bit will be set bit"]
    #[inline(always)]
    pub fn a_vbus_vld(&self) -> A_VBUS_VLD_R {
        A_VBUS_VLD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - This bit is set when the VBUS voltage is below the B device session end threshold"]
    #[inline(always)]
    pub fn b_sess_end(&self) -> B_SESS_END_R {
        B_SESS_END_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - This bit is set when the VBUS voltage is higher than the valid threshold of the B device session bit"]
    #[inline(always)]
    pub fn sess_vld(&self) -> SESS_VLD_R {
        SESS_VLD_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - Dithering"]
    #[inline(always)]
    pub fn line_state_stable(&self) -> LINE_STATE_STABLE_R {
        LINE_STATE_STABLE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - This bit is reserved for the 1 ms counter and is not valid for software"]
    #[inline(always)]
    pub fn _1msec(&self) -> _1MSEC_R {
        _1MSEC_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Indicates the current status of the ID pin on the USB connector"]
    #[inline(always)]
    pub fn id(&self) -> ID_R {
        ID_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - When the VBUS voltage is higher than the VBUS effective threshold of a device, this bit will be set bit"]
    #[inline(always)]
    #[must_use]
    pub fn a_vbus_vld(&mut self) -> A_VBUS_VLD_W<0> {
        A_VBUS_VLD_W::new(self)
    }
    #[doc = "Bit 2 - This bit is set when the VBUS voltage is below the B device session end threshold"]
    #[inline(always)]
    #[must_use]
    pub fn b_sess_end(&mut self) -> B_SESS_END_W<2> {
        B_SESS_END_W::new(self)
    }
    #[doc = "Bit 3 - This bit is set when the VBUS voltage is higher than the valid threshold of the B device session bit"]
    #[inline(always)]
    #[must_use]
    pub fn sess_vld(&mut self) -> SESS_VLD_W<3> {
        SESS_VLD_W::new(self)
    }
    #[doc = "Bit 5 - Dithering"]
    #[inline(always)]
    #[must_use]
    pub fn line_state_stable(&mut self) -> LINE_STATE_STABLE_W<5> {
        LINE_STATE_STABLE_W::new(self)
    }
    #[doc = "Bit 6 - This bit is reserved for the 1 ms counter and is not valid for software"]
    #[inline(always)]
    #[must_use]
    pub fn _1msec(&mut self) -> _1MSEC_W<6> {
        _1MSEC_W::new(self)
    }
    #[doc = "Bit 7 - Indicates the current status of the ID pin on the USB connector"]
    #[inline(always)]
    #[must_use]
    pub fn id(&mut self) -> ID_W<7> {
        ID_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OTG Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_stat](index.html) module"]
pub struct OTG_STAT_SPEC;
impl crate::RegisterSpec for OTG_STAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [otg_stat::R](R) reader structure"]
impl crate::Readable for OTG_STAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [otg_stat::W](W) writer structure"]
impl crate::Writable for OTG_STAT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OTG_STAT to value 0xa8"]
impl crate::Resettable for OTG_STAT_SPEC {
    const RESET_VALUE: Self::Ux = 0xa8;
}
