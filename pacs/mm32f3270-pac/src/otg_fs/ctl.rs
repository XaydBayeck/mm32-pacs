#[doc = "Register `CTL` reader"]
pub struct R(crate::R<CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTL` writer"]
pub struct W(crate::W<CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTL_SPEC>;
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
impl From<crate::W<CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USB_EN_SOF_EN` reader - Setting this bit will make otg_fs work, clearing it will disable otg_fs"]
pub type USB_EN_SOF_EN_R = crate::BitReader<bool>;
#[doc = "Field `USB_EN_SOF_EN` writer - Setting this bit will make otg_fs work, clearing it will disable otg_fs"]
pub type USB_EN_SOF_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL_SPEC, bool, O>;
#[doc = "Field `ODD_RST` reader - Setting this bit will reset all BDT odd Ping_Pong bits to 0 and then specify an even number of BDT libraries"]
pub type ODD_RST_R = crate::BitReader<bool>;
#[doc = "Field `ODD_RST` writer - Setting this bit will reset all BDT odd Ping_Pong bits to 0 and then specify an even number of BDT libraries"]
pub type ODD_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL_SPEC, bool, O>;
#[doc = "Field `RESUME` reader - Setting this bit will allow the otg_fs to perform the recovery signal"]
pub type RESUME_R = crate::BitReader<bool>;
#[doc = "Field `RESUME` writer - Setting this bit will allow the otg_fs to perform the recovery signal"]
pub type RESUME_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL_SPEC, bool, O>;
#[doc = "Field `HOST_MODE_EN` reader - Setting this bit causes the otg_fs to work in host mode"]
pub type HOST_MODE_EN_R = crate::BitReader<bool>;
#[doc = "Field `HOST_MODE_EN` writer - Setting this bit causes the otg_fs to work in host mode"]
pub type HOST_MODE_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL_SPEC, bool, O>;
#[doc = "Field `RESET` reader - Setting this bit will cause otg_fs to generate USB reset signal"]
pub type RESET_R = crate::BitReader<bool>;
#[doc = "Field `RESET` writer - Setting this bit will cause otg_fs to generate USB reset signal"]
pub type RESET_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL_SPEC, bool, O>;
#[doc = "Field `TXDSUSPEND_TOKENBUSY` reader - When otg_fs is in slave mode,it is TXD_ Suspend token when otg_fs is in host mode_ Busy"]
pub type TXDSUSPEND_TOKENBUSY_R = crate::BitReader<bool>;
#[doc = "Field `TXDSUSPEND_TOKENBUSY` writer - When otg_fs is in slave mode,it is TXD_ Suspend token when otg_fs is in host mode_ Busy"]
pub type TXDSUSPEND_TOKENBUSY_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL_SPEC, bool, O>;
#[doc = "Field `SE0` reader - SE0 signal received by USB differential receiver"]
pub type SE0_R = crate::BitReader<bool>;
#[doc = "Field `SE0` writer - SE0 signal received by USB differential receiver"]
pub type SE0_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL_SPEC, bool, O>;
#[doc = "Field `JSTATE` reader - USB differential receiver receives jstate signal"]
pub type JSTATE_R = crate::BitReader<bool>;
#[doc = "Field `JSTATE` writer - USB differential receiver receives jstate signal"]
pub type JSTATE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Setting this bit will make otg_fs work, clearing it will disable otg_fs"]
    #[inline(always)]
    pub fn usb_en_sof_en(&self) -> USB_EN_SOF_EN_R {
        USB_EN_SOF_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Setting this bit will reset all BDT odd Ping_Pong bits to 0 and then specify an even number of BDT libraries"]
    #[inline(always)]
    pub fn odd_rst(&self) -> ODD_RST_R {
        ODD_RST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Setting this bit will allow the otg_fs to perform the recovery signal"]
    #[inline(always)]
    pub fn resume(&self) -> RESUME_R {
        RESUME_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Setting this bit causes the otg_fs to work in host mode"]
    #[inline(always)]
    pub fn host_mode_en(&self) -> HOST_MODE_EN_R {
        HOST_MODE_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Setting this bit will cause otg_fs to generate USB reset signal"]
    #[inline(always)]
    pub fn reset(&self) -> RESET_R {
        RESET_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - When otg_fs is in slave mode,it is TXD_ Suspend token when otg_fs is in host mode_ Busy"]
    #[inline(always)]
    pub fn txdsuspend_tokenbusy(&self) -> TXDSUSPEND_TOKENBUSY_R {
        TXDSUSPEND_TOKENBUSY_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SE0 signal received by USB differential receiver"]
    #[inline(always)]
    pub fn se0(&self) -> SE0_R {
        SE0_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - USB differential receiver receives jstate signal"]
    #[inline(always)]
    pub fn jstate(&self) -> JSTATE_R {
        JSTATE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Setting this bit will make otg_fs work, clearing it will disable otg_fs"]
    #[inline(always)]
    #[must_use]
    pub fn usb_en_sof_en(&mut self) -> USB_EN_SOF_EN_W<0> {
        USB_EN_SOF_EN_W::new(self)
    }
    #[doc = "Bit 1 - Setting this bit will reset all BDT odd Ping_Pong bits to 0 and then specify an even number of BDT libraries"]
    #[inline(always)]
    #[must_use]
    pub fn odd_rst(&mut self) -> ODD_RST_W<1> {
        ODD_RST_W::new(self)
    }
    #[doc = "Bit 2 - Setting this bit will allow the otg_fs to perform the recovery signal"]
    #[inline(always)]
    #[must_use]
    pub fn resume(&mut self) -> RESUME_W<2> {
        RESUME_W::new(self)
    }
    #[doc = "Bit 3 - Setting this bit causes the otg_fs to work in host mode"]
    #[inline(always)]
    #[must_use]
    pub fn host_mode_en(&mut self) -> HOST_MODE_EN_W<3> {
        HOST_MODE_EN_W::new(self)
    }
    #[doc = "Bit 4 - Setting this bit will cause otg_fs to generate USB reset signal"]
    #[inline(always)]
    #[must_use]
    pub fn reset(&mut self) -> RESET_W<4> {
        RESET_W::new(self)
    }
    #[doc = "Bit 5 - When otg_fs is in slave mode,it is TXD_ Suspend token when otg_fs is in host mode_ Busy"]
    #[inline(always)]
    #[must_use]
    pub fn txdsuspend_tokenbusy(&mut self) -> TXDSUSPEND_TOKENBUSY_W<5> {
        TXDSUSPEND_TOKENBUSY_W::new(self)
    }
    #[doc = "Bit 6 - SE0 signal received by USB differential receiver"]
    #[inline(always)]
    #[must_use]
    pub fn se0(&mut self) -> SE0_W<6> {
        SE0_W::new(self)
    }
    #[doc = "Bit 7 - USB differential receiver receives jstate signal"]
    #[inline(always)]
    #[must_use]
    pub fn jstate(&mut self) -> JSTATE_W<7> {
        JSTATE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctl](index.html) module"]
pub struct CTL_SPEC;
impl crate::RegisterSpec for CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctl::R](R) reader structure"]
impl crate::Readable for CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctl::W](W) writer structure"]
impl crate::Writable for CTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTL to value 0x40"]
impl crate::Resettable for CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0x40;
}
