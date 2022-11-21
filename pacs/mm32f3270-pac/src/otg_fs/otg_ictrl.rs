#[doc = "Register `OTG_ICTRL` reader"]
pub struct R(crate::R<OTG_ICTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OTG_ICTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OTG_ICTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OTG_ICTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OTG_ICTRL` writer"]
pub struct W(crate::W<OTG_ICTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OTG_ICTRL_SPEC>;
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
impl From<crate::W<OTG_ICTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OTG_ICTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `A_VBUS_VLD_EN` reader - Enable a device VBUS valid interrupt"]
pub type A_VBUS_VLD_EN_R = crate::BitReader<bool>;
#[doc = "Field `A_VBUS_VLD_EN` writer - Enable a device VBUS valid interrupt"]
pub type A_VBUS_VLD_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTG_ICTRL_SPEC, bool, O>;
#[doc = "Field `B_SESS_END_EN` reader - Enable b device VBUS valid interrupt"]
pub type B_SESS_END_EN_R = crate::BitReader<bool>;
#[doc = "Field `B_SESS_END_EN` writer - Enable b device VBUS valid interrupt"]
pub type B_SESS_END_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTG_ICTRL_SPEC, bool, O>;
#[doc = "Field `SESS_VLD_EN` reader - Enables the session to be effectively interrupted"]
pub type SESS_VLD_EN_R = crate::BitReader<bool>;
#[doc = "Field `SESS_VLD_EN` writer - Enables the session to be effectively interrupted"]
pub type SESS_VLD_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTG_ICTRL_SPEC, bool, O>;
#[doc = "Field `LINE_STATE_EN` reader - Enable line state change interrupt."]
pub type LINE_STATE_EN_R = crate::BitReader<bool>;
#[doc = "Field `LINE_STATE_EN` writer - Enable line state change interrupt."]
pub type LINE_STATE_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTG_ICTRL_SPEC, bool, O>;
#[doc = "Field `_1MSEC_EN` reader - Enable 1 ms timer interrupt."]
pub type _1MSEC_EN_R = crate::BitReader<bool>;
#[doc = "Field `_1MSEC_EN` writer - Enable 1 ms timer interrupt."]
pub type _1MSEC_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTG_ICTRL_SPEC, bool, O>;
#[doc = "Field `ID_EN` reader - Enable ID signal interrupt"]
pub type ID_EN_R = crate::BitReader<bool>;
#[doc = "Field `ID_EN` writer - Enable ID signal interrupt"]
pub type ID_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTG_ICTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Enable a device VBUS valid interrupt"]
    #[inline(always)]
    pub fn a_vbus_vld_en(&self) -> A_VBUS_VLD_EN_R {
        A_VBUS_VLD_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Enable b device VBUS valid interrupt"]
    #[inline(always)]
    pub fn b_sess_end_en(&self) -> B_SESS_END_EN_R {
        B_SESS_END_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enables the session to be effectively interrupted"]
    #[inline(always)]
    pub fn sess_vld_en(&self) -> SESS_VLD_EN_R {
        SESS_VLD_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable line state change interrupt."]
    #[inline(always)]
    pub fn line_state_en(&self) -> LINE_STATE_EN_R {
        LINE_STATE_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable 1 ms timer interrupt."]
    #[inline(always)]
    pub fn _1msec_en(&self) -> _1MSEC_EN_R {
        _1MSEC_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable ID signal interrupt"]
    #[inline(always)]
    pub fn id_en(&self) -> ID_EN_R {
        ID_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable a device VBUS valid interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn a_vbus_vld_en(&mut self) -> A_VBUS_VLD_EN_W<0> {
        A_VBUS_VLD_EN_W::new(self)
    }
    #[doc = "Bit 2 - Enable b device VBUS valid interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn b_sess_end_en(&mut self) -> B_SESS_END_EN_W<2> {
        B_SESS_END_EN_W::new(self)
    }
    #[doc = "Bit 3 - Enables the session to be effectively interrupted"]
    #[inline(always)]
    #[must_use]
    pub fn sess_vld_en(&mut self) -> SESS_VLD_EN_W<3> {
        SESS_VLD_EN_W::new(self)
    }
    #[doc = "Bit 5 - Enable line state change interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn line_state_en(&mut self) -> LINE_STATE_EN_W<5> {
        LINE_STATE_EN_W::new(self)
    }
    #[doc = "Bit 6 - Enable 1 ms timer interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn _1msec_en(&mut self) -> _1MSEC_EN_W<6> {
        _1MSEC_EN_W::new(self)
    }
    #[doc = "Bit 7 - Enable ID signal interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn id_en(&mut self) -> ID_EN_W<7> {
        ID_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OTG Interrupt Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_ictrl](index.html) module"]
pub struct OTG_ICTRL_SPEC;
impl crate::RegisterSpec for OTG_ICTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [otg_ictrl::R](R) reader structure"]
impl crate::Readable for OTG_ICTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [otg_ictrl::W](W) writer structure"]
impl crate::Writable for OTG_ICTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OTG_ICTRL to value 0"]
impl crate::Resettable for OTG_ICTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
