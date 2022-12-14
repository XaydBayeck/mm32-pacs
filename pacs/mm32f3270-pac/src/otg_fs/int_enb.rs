#[doc = "Register `INT_ENB` reader"]
pub struct R(crate::R<INT_ENB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INT_ENB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INT_ENB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INT_ENB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INT_ENB` writer"]
pub struct W(crate::W<INT_ENB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INT_ENB_SPEC>;
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
impl From<crate::W<INT_ENB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INT_ENB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USB_RST_EN` reader - Setting this bit will enable USB_ RST interrupt"]
pub type USB_RST_EN_R = crate::BitReader<bool>;
#[doc = "Field `USB_RST_EN` writer - Setting this bit will enable USB_ RST interrupt"]
pub type USB_RST_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_ENB_SPEC, bool, O>;
#[doc = "Field `ERROR_EN` reader - Setting this bit enables the ERROR interrupt"]
pub type ERROR_EN_R = crate::BitReader<bool>;
#[doc = "Field `ERROR_EN` writer - Setting this bit enables the ERROR interrupt"]
pub type ERROR_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_ENB_SPEC, bool, O>;
#[doc = "Field `SOF_TOK_EN` reader - This position bit will enable TOK_END interrupt"]
pub type SOF_TOK_EN_R = crate::BitReader<bool>;
#[doc = "Field `SOF_TOK_EN` writer - This position bit will enable TOK_END interrupt"]
pub type SOF_TOK_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_ENB_SPEC, bool, O>;
#[doc = "Field `TOK_DNE_EN` reader - This position bit will enable SOF_ Tok interrupt"]
pub type TOK_DNE_EN_R = crate::BitReader<bool>;
#[doc = "Field `TOK_DNE_EN` writer - This position bit will enable SOF_ Tok interrupt"]
pub type TOK_DNE_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_ENB_SPEC, bool, O>;
#[doc = "Field `SLEEP_EN` reader - This position bit will enable SLEEP interrupt"]
pub type SLEEP_EN_R = crate::BitReader<bool>;
#[doc = "Field `SLEEP_EN` writer - This position bit will enable SLEEP interrupt"]
pub type SLEEP_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_ENB_SPEC, bool, O>;
#[doc = "Field `RESUME_EN` reader - This position bit will enable RESUME interrupt"]
pub type RESUME_EN_R = crate::BitReader<bool>;
#[doc = "Field `RESUME_EN` writer - This position bit will enable RESUME interrupt"]
pub type RESUME_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_ENB_SPEC, bool, O>;
#[doc = "Field `ATTACH_EN` reader - This position bit will enable ATTACH interrupt"]
pub type ATTACH_EN_R = crate::BitReader<bool>;
#[doc = "Field `ATTACH_EN` writer - This position bit will enable ATTACH interrupt"]
pub type ATTACH_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_ENB_SPEC, bool, O>;
#[doc = "Field `STALL_EN` reader - This position bit will enable STALL interrupt."]
pub type STALL_EN_R = crate::BitReader<bool>;
#[doc = "Field `STALL_EN` writer - This position bit will enable STALL interrupt."]
pub type STALL_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_ENB_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Setting this bit will enable USB_ RST interrupt"]
    #[inline(always)]
    pub fn usb_rst_en(&self) -> USB_RST_EN_R {
        USB_RST_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Setting this bit enables the ERROR interrupt"]
    #[inline(always)]
    pub fn error_en(&self) -> ERROR_EN_R {
        ERROR_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - This position bit will enable TOK_END interrupt"]
    #[inline(always)]
    pub fn sof_tok_en(&self) -> SOF_TOK_EN_R {
        SOF_TOK_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - This position bit will enable SOF_ Tok interrupt"]
    #[inline(always)]
    pub fn tok_dne_en(&self) -> TOK_DNE_EN_R {
        TOK_DNE_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - This position bit will enable SLEEP interrupt"]
    #[inline(always)]
    pub fn sleep_en(&self) -> SLEEP_EN_R {
        SLEEP_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - This position bit will enable RESUME interrupt"]
    #[inline(always)]
    pub fn resume_en(&self) -> RESUME_EN_R {
        RESUME_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - This position bit will enable ATTACH interrupt"]
    #[inline(always)]
    pub fn attach_en(&self) -> ATTACH_EN_R {
        ATTACH_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - This position bit will enable STALL interrupt."]
    #[inline(always)]
    pub fn stall_en(&self) -> STALL_EN_R {
        STALL_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Setting this bit will enable USB_ RST interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn usb_rst_en(&mut self) -> USB_RST_EN_W<0> {
        USB_RST_EN_W::new(self)
    }
    #[doc = "Bit 1 - Setting this bit enables the ERROR interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn error_en(&mut self) -> ERROR_EN_W<1> {
        ERROR_EN_W::new(self)
    }
    #[doc = "Bit 2 - This position bit will enable TOK_END interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn sof_tok_en(&mut self) -> SOF_TOK_EN_W<2> {
        SOF_TOK_EN_W::new(self)
    }
    #[doc = "Bit 3 - This position bit will enable SOF_ Tok interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn tok_dne_en(&mut self) -> TOK_DNE_EN_W<3> {
        TOK_DNE_EN_W::new(self)
    }
    #[doc = "Bit 4 - This position bit will enable SLEEP interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn sleep_en(&mut self) -> SLEEP_EN_W<4> {
        SLEEP_EN_W::new(self)
    }
    #[doc = "Bit 5 - This position bit will enable RESUME interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn resume_en(&mut self) -> RESUME_EN_W<5> {
        RESUME_EN_W::new(self)
    }
    #[doc = "Bit 6 - This position bit will enable ATTACH interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn attach_en(&mut self) -> ATTACH_EN_W<6> {
        ATTACH_EN_W::new(self)
    }
    #[doc = "Bit 7 - This position bit will enable STALL interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn stall_en(&mut self) -> STALL_EN_W<7> {
        STALL_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_enb](index.html) module"]
pub struct INT_ENB_SPEC;
impl crate::RegisterSpec for INT_ENB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [int_enb::R](R) reader structure"]
impl crate::Readable for INT_ENB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [int_enb::W](W) writer structure"]
impl crate::Writable for INT_ENB_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INT_ENB to value 0"]
impl crate::Resettable for INT_ENB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
