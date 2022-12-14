#[doc = "Register `EP_CTL0` reader"]
pub struct R(crate::R<EP_CTL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EP_CTL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EP_CTL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EP_CTL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EP_CTL0` writer"]
pub struct W(crate::W<EP_CTL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EP_CTL0_SPEC>;
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
impl From<crate::W<EP_CTL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EP_CTL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EP_HSHK_0` reader - Setting this bit defines whether the endpoint performs a handshake to the endpoint during the transaction"]
pub type EP_HSHK_0_R = crate::BitReader<bool>;
#[doc = "Field `EP_HSHK_0` writer - Setting this bit defines whether the endpoint performs a handshake to the endpoint during the transaction"]
pub type EP_HSHK_0_W<'a, const O: u8> = crate::BitWriter<'a, u32, EP_CTL0_SPEC, bool, O>;
#[doc = "Field `EP_STALL_0` reader - Any access to this endpoint will cause the otg_fs to return a stall handshake"]
pub type EP_STALL_0_R = crate::BitReader<bool>;
#[doc = "Field `EP_STALL_0` writer - Any access to this endpoint will cause the otg_fs to return a stall handshake"]
pub type EP_STALL_0_W<'a, const O: u8> = crate::BitWriter<'a, u32, EP_CTL0_SPEC, bool, O>;
#[doc = "Field `EP_TX_EN_0` reader - These three bits define whether the endpoint is enabled and the direction of the endpoint"]
pub type EP_TX_EN_0_R = crate::BitReader<bool>;
#[doc = "Field `EP_TX_EN_0` writer - These three bits define whether the endpoint is enabled and the direction of the endpoint"]
pub type EP_TX_EN_0_W<'a, const O: u8> = crate::BitWriter<'a, u32, EP_CTL0_SPEC, bool, O>;
#[doc = "Field `EP_RX_EN_0` reader - These three bits define whether the endpoint is enabled and the direction of the endpoint"]
pub type EP_RX_EN_0_R = crate::BitReader<bool>;
#[doc = "Field `EP_RX_EN_0` writer - These three bits define whether the endpoint is enabled and the direction of the endpoint"]
pub type EP_RX_EN_0_W<'a, const O: u8> = crate::BitWriter<'a, u32, EP_CTL0_SPEC, bool, O>;
#[doc = "Field `EP_CTL_DIS_0` reader - These three bits define whether the endpoint is enabled and the direction of the endpoint"]
pub type EP_CTL_DIS_0_R = crate::BitReader<bool>;
#[doc = "Field `EP_CTL_DIS_0` writer - These three bits define whether the endpoint is enabled and the direction of the endpoint"]
pub type EP_CTL_DIS_0_W<'a, const O: u8> = crate::BitWriter<'a, u32, EP_CTL0_SPEC, bool, O>;
#[doc = "Field `RETRY_DIS_0` reader - This is a bit for master mode only and exists only in the control register of endpoint 0"]
pub type RETRY_DIS_0_R = crate::BitReader<bool>;
#[doc = "Field `RETRY_DIS_0` writer - This is a bit for master mode only and exists only in the control register of endpoint 0"]
pub type RETRY_DIS_0_W<'a, const O: u8> = crate::BitWriter<'a, u32, EP_CTL0_SPEC, bool, O>;
#[doc = "Field `HOST_WO_HUB_0` reader - This is a bit for master mode only and exists only in the control endpt0_rg register of endpoint 0."]
pub type HOST_WO_HUB_0_R = crate::BitReader<bool>;
#[doc = "Field `HOST_WO_HUB_0` writer - This is a bit for master mode only and exists only in the control endpt0_rg register of endpoint 0."]
pub type HOST_WO_HUB_0_W<'a, const O: u8> = crate::BitWriter<'a, u32, EP_CTL0_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Setting this bit defines whether the endpoint performs a handshake to the endpoint during the transaction"]
    #[inline(always)]
    pub fn ep_hshk_0(&self) -> EP_HSHK_0_R {
        EP_HSHK_0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Any access to this endpoint will cause the otg_fs to return a stall handshake"]
    #[inline(always)]
    pub fn ep_stall_0(&self) -> EP_STALL_0_R {
        EP_STALL_0_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - These three bits define whether the endpoint is enabled and the direction of the endpoint"]
    #[inline(always)]
    pub fn ep_tx_en_0(&self) -> EP_TX_EN_0_R {
        EP_TX_EN_0_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - These three bits define whether the endpoint is enabled and the direction of the endpoint"]
    #[inline(always)]
    pub fn ep_rx_en_0(&self) -> EP_RX_EN_0_R {
        EP_RX_EN_0_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - These three bits define whether the endpoint is enabled and the direction of the endpoint"]
    #[inline(always)]
    pub fn ep_ctl_dis_0(&self) -> EP_CTL_DIS_0_R {
        EP_CTL_DIS_0_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - This is a bit for master mode only and exists only in the control register of endpoint 0"]
    #[inline(always)]
    pub fn retry_dis_0(&self) -> RETRY_DIS_0_R {
        RETRY_DIS_0_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - This is a bit for master mode only and exists only in the control endpt0_rg register of endpoint 0."]
    #[inline(always)]
    pub fn host_wo_hub_0(&self) -> HOST_WO_HUB_0_R {
        HOST_WO_HUB_0_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Setting this bit defines whether the endpoint performs a handshake to the endpoint during the transaction"]
    #[inline(always)]
    #[must_use]
    pub fn ep_hshk_0(&mut self) -> EP_HSHK_0_W<0> {
        EP_HSHK_0_W::new(self)
    }
    #[doc = "Bit 1 - Any access to this endpoint will cause the otg_fs to return a stall handshake"]
    #[inline(always)]
    #[must_use]
    pub fn ep_stall_0(&mut self) -> EP_STALL_0_W<1> {
        EP_STALL_0_W::new(self)
    }
    #[doc = "Bit 2 - These three bits define whether the endpoint is enabled and the direction of the endpoint"]
    #[inline(always)]
    #[must_use]
    pub fn ep_tx_en_0(&mut self) -> EP_TX_EN_0_W<2> {
        EP_TX_EN_0_W::new(self)
    }
    #[doc = "Bit 3 - These three bits define whether the endpoint is enabled and the direction of the endpoint"]
    #[inline(always)]
    #[must_use]
    pub fn ep_rx_en_0(&mut self) -> EP_RX_EN_0_W<3> {
        EP_RX_EN_0_W::new(self)
    }
    #[doc = "Bit 4 - These three bits define whether the endpoint is enabled and the direction of the endpoint"]
    #[inline(always)]
    #[must_use]
    pub fn ep_ctl_dis_0(&mut self) -> EP_CTL_DIS_0_W<4> {
        EP_CTL_DIS_0_W::new(self)
    }
    #[doc = "Bit 6 - This is a bit for master mode only and exists only in the control register of endpoint 0"]
    #[inline(always)]
    #[must_use]
    pub fn retry_dis_0(&mut self) -> RETRY_DIS_0_W<6> {
        RETRY_DIS_0_W::new(self)
    }
    #[doc = "Bit 7 - This is a bit for master mode only and exists only in the control endpt0_rg register of endpoint 0."]
    #[inline(always)]
    #[must_use]
    pub fn host_wo_hub_0(&mut self) -> HOST_WO_HUB_0_W<7> {
        HOST_WO_HUB_0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Endpoint control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ep_ctl0](index.html) module"]
pub struct EP_CTL0_SPEC;
impl crate::RegisterSpec for EP_CTL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ep_ctl0::R](R) reader structure"]
impl crate::Readable for EP_CTL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ep_ctl0::W](W) writer structure"]
impl crate::Writable for EP_CTL0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EP_CTL0 to value 0"]
impl crate::Resettable for EP_CTL0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
