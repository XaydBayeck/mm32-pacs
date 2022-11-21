#[doc = "Register `INT_STAT` reader"]
pub struct R(crate::R<INT_STAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INT_STAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INT_STAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INT_STAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INT_STAT` writer"]
pub struct W(crate::W<INT_STAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INT_STAT_SPEC>;
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
impl From<crate::W<INT_STAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INT_STAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USB_RST` reader - When otg_fs decodes valid USB reset, this position bit"]
pub type USB_RST_R = crate::BitReader<bool>;
#[doc = "Field `USB_RST` writer - When otg_fs decodes valid USB reset, this position bit"]
pub type USB_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_STAT_SPEC, bool, O>;
#[doc = "Field `ERROR` reader - This position bit when any error condition in the ERR_STAT register occurs"]
pub type ERROR_R = crate::BitReader<bool>;
#[doc = "Field `ERROR` writer - This position bit when any error condition in the ERR_STAT register occurs"]
pub type ERROR_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_STAT_SPEC, bool, O>;
#[doc = "Field `SOF_TOK` reader - If the otg_fs receives a sof token, this location bit"]
pub type SOF_TOK_R = crate::BitReader<bool>;
#[doc = "Field `SOF_TOK` writer - If the otg_fs receives a sof token, this location bit"]
pub type SOF_TOK_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_STAT_SPEC, bool, O>;
#[doc = "Field `TOK_DNE` reader - This bit is set when the token being processed completes"]
pub type TOK_DNE_R = crate::BitReader<bool>;
#[doc = "Field `TOK_DNE` writer - This bit is set when the token being processed completes"]
pub type TOK_DNE_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_STAT_SPEC, bool, O>;
#[doc = "Field `SLEEP` reader - If otg_fs detects 3MS idle on the USB bus signal, then Position 1"]
pub type SLEEP_R = crate::BitReader<bool>;
#[doc = "Field `SLEEP` writer - If otg_fs detects 3MS idle on the USB bus signal, then Position 1"]
pub type SLEEP_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_STAT_SPEC, bool, O>;
#[doc = "Field `RESUME` reader - When the K state is observed on the DP_DM signal line within 2.5 microseconds, this position 1"]
pub type RESUME_R = crate::BitReader<bool>;
#[doc = "Field `RESUME` writer - When the K state is observed on the DP_DM signal line within 2.5 microseconds, this position 1"]
pub type RESUME_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_STAT_SPEC, bool, O>;
#[doc = "Field `ATTACH` reader - If OTG_FS detects a connection to a USB peripheral, the location bit"]
pub type ATTACH_R = crate::BitReader<bool>;
#[doc = "Field `ATTACH` writer - If OTG_FS detects a connection to a USB peripheral, the location bit"]
pub type ATTACH_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_STAT_SPEC, bool, O>;
#[doc = "Field `STALL` reader - Stall interrupt is used in slave mode and master mode. In slave mode,SIE set when sending stall handshake packet."]
pub type STALL_R = crate::BitReader<bool>;
#[doc = "Field `STALL` writer - Stall interrupt is used in slave mode and master mode. In slave mode,SIE set when sending stall handshake packet."]
pub type STALL_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_STAT_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - When otg_fs decodes valid USB reset, this position bit"]
    #[inline(always)]
    pub fn usb_rst(&self) -> USB_RST_R {
        USB_RST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This position bit when any error condition in the ERR_STAT register occurs"]
    #[inline(always)]
    pub fn error(&self) -> ERROR_R {
        ERROR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - If the otg_fs receives a sof token, this location bit"]
    #[inline(always)]
    pub fn sof_tok(&self) -> SOF_TOK_R {
        SOF_TOK_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - This bit is set when the token being processed completes"]
    #[inline(always)]
    pub fn tok_dne(&self) -> TOK_DNE_R {
        TOK_DNE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - If otg_fs detects 3MS idle on the USB bus signal, then Position 1"]
    #[inline(always)]
    pub fn sleep(&self) -> SLEEP_R {
        SLEEP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - When the K state is observed on the DP_DM signal line within 2.5 microseconds, this position 1"]
    #[inline(always)]
    pub fn resume(&self) -> RESUME_R {
        RESUME_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - If OTG_FS detects a connection to a USB peripheral, the location bit"]
    #[inline(always)]
    pub fn attach(&self) -> ATTACH_R {
        ATTACH_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Stall interrupt is used in slave mode and master mode. In slave mode,SIE set when sending stall handshake packet."]
    #[inline(always)]
    pub fn stall(&self) -> STALL_R {
        STALL_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - When otg_fs decodes valid USB reset, this position bit"]
    #[inline(always)]
    #[must_use]
    pub fn usb_rst(&mut self) -> USB_RST_W<0> {
        USB_RST_W::new(self)
    }
    #[doc = "Bit 1 - This position bit when any error condition in the ERR_STAT register occurs"]
    #[inline(always)]
    #[must_use]
    pub fn error(&mut self) -> ERROR_W<1> {
        ERROR_W::new(self)
    }
    #[doc = "Bit 2 - If the otg_fs receives a sof token, this location bit"]
    #[inline(always)]
    #[must_use]
    pub fn sof_tok(&mut self) -> SOF_TOK_W<2> {
        SOF_TOK_W::new(self)
    }
    #[doc = "Bit 3 - This bit is set when the token being processed completes"]
    #[inline(always)]
    #[must_use]
    pub fn tok_dne(&mut self) -> TOK_DNE_W<3> {
        TOK_DNE_W::new(self)
    }
    #[doc = "Bit 4 - If otg_fs detects 3MS idle on the USB bus signal, then Position 1"]
    #[inline(always)]
    #[must_use]
    pub fn sleep(&mut self) -> SLEEP_W<4> {
        SLEEP_W::new(self)
    }
    #[doc = "Bit 5 - When the K state is observed on the DP_DM signal line within 2.5 microseconds, this position 1"]
    #[inline(always)]
    #[must_use]
    pub fn resume(&mut self) -> RESUME_W<5> {
        RESUME_W::new(self)
    }
    #[doc = "Bit 6 - If OTG_FS detects a connection to a USB peripheral, the location bit"]
    #[inline(always)]
    #[must_use]
    pub fn attach(&mut self) -> ATTACH_W<6> {
        ATTACH_W::new(self)
    }
    #[doc = "Bit 7 - Stall interrupt is used in slave mode and master mode. In slave mode,SIE set when sending stall handshake packet."]
    #[inline(always)]
    #[must_use]
    pub fn stall(&mut self) -> STALL_W<7> {
        STALL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_stat](index.html) module"]
pub struct INT_STAT_SPEC;
impl crate::RegisterSpec for INT_STAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [int_stat::R](R) reader structure"]
impl crate::Readable for INT_STAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [int_stat::W](W) writer structure"]
impl crate::Writable for INT_STAT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INT_STAT to value 0x01"]
impl crate::Resettable for INT_STAT_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
