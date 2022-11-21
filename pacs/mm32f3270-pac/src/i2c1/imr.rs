#[doc = "Register `IMR` reader"]
pub struct R(crate::R<IMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IMR` writer"]
pub struct W(crate::W<IMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IMR_SPEC>;
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
impl From<crate::W<IMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RX_UNDER` reader - Receive buffer under"]
pub type RX_UNDER_R = crate::BitReader<bool>;
#[doc = "Field `RX_UNDER` writer - Receive buffer under"]
pub type RX_UNDER_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR_SPEC, bool, O>;
#[doc = "Field `RX_OVER` reader - Receive buffer over"]
pub type RX_OVER_R = crate::BitReader<bool>;
#[doc = "Field `RX_OVER` writer - Receive buffer over"]
pub type RX_OVER_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR_SPEC, bool, O>;
#[doc = "Field `RX_FULL` reader - Receive buffer not empty"]
pub type RX_FULL_R = crate::BitReader<bool>;
#[doc = "Field `RX_FULL` writer - Receive buffer not empty"]
pub type RX_FULL_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR_SPEC, bool, O>;
#[doc = "Field `TX_OVER` reader - Transmit buffer over"]
pub type TX_OVER_R = crate::BitReader<bool>;
#[doc = "Field `TX_OVER` writer - Transmit buffer over"]
pub type TX_OVER_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR_SPEC, bool, O>;
#[doc = "Field `TX_EMPTY` reader - Transmit buffer empty"]
pub type TX_EMPTY_R = crate::BitReader<bool>;
#[doc = "Field `TX_EMPTY` writer - Transmit buffer empty"]
pub type TX_EMPTY_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR_SPEC, bool, O>;
#[doc = "Field `RD_REQ` reader - Read request"]
pub type RD_REQ_R = crate::BitReader<bool>;
#[doc = "Field `RD_REQ` writer - Read request"]
pub type RD_REQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR_SPEC, bool, O>;
#[doc = "Field `TX_ABRT` reader - Transmit abort"]
pub type TX_ABRT_R = crate::BitReader<bool>;
#[doc = "Field `TX_ABRT` writer - Transmit abort"]
pub type TX_ABRT_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR_SPEC, bool, O>;
#[doc = "Field `RX_DONE` reader - Transmit done"]
pub type RX_DONE_R = crate::BitReader<bool>;
#[doc = "Field `RX_DONE` writer - Transmit done"]
pub type RX_DONE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR_SPEC, bool, O>;
#[doc = "Field `ACTIV` reader - This bit captures DW_spb_i2c acticity and stays set until it is cleared"]
pub type ACTIV_R = crate::BitReader<bool>;
#[doc = "Field `ACTIV` writer - This bit captures DW_spb_i2c acticity and stays set until it is cleared"]
pub type ACTIV_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR_SPEC, bool, O>;
#[doc = "Field `STOP` reader - Stop condition detection"]
pub type STOP_R = crate::BitReader<bool>;
#[doc = "Field `STOP` writer - Stop condition detection"]
pub type STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR_SPEC, bool, O>;
#[doc = "Field `START` reader - Start condition detection"]
pub type START_R = crate::BitReader<bool>;
#[doc = "Field `START` writer - Start condition detection"]
pub type START_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR_SPEC, bool, O>;
#[doc = "Field `GC` reader - General call"]
pub type GC_R = crate::BitReader<bool>;
#[doc = "Field `GC` writer - General call"]
pub type GC_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Receive buffer under"]
    #[inline(always)]
    pub fn rx_under(&self) -> RX_UNDER_R {
        RX_UNDER_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Receive buffer over"]
    #[inline(always)]
    pub fn rx_over(&self) -> RX_OVER_R {
        RX_OVER_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Receive buffer not empty"]
    #[inline(always)]
    pub fn rx_full(&self) -> RX_FULL_R {
        RX_FULL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmit buffer over"]
    #[inline(always)]
    pub fn tx_over(&self) -> TX_OVER_R {
        TX_OVER_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Transmit buffer empty"]
    #[inline(always)]
    pub fn tx_empty(&self) -> TX_EMPTY_R {
        TX_EMPTY_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Read request"]
    #[inline(always)]
    pub fn rd_req(&self) -> RD_REQ_R {
        RD_REQ_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Transmit abort"]
    #[inline(always)]
    pub fn tx_abrt(&self) -> TX_ABRT_R {
        TX_ABRT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Transmit done"]
    #[inline(always)]
    pub fn rx_done(&self) -> RX_DONE_R {
        RX_DONE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - This bit captures DW_spb_i2c acticity and stays set until it is cleared"]
    #[inline(always)]
    pub fn activ(&self) -> ACTIV_R {
        ACTIV_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Stop condition detection"]
    #[inline(always)]
    pub fn stop(&self) -> STOP_R {
        STOP_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Start condition detection"]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - General call"]
    #[inline(always)]
    pub fn gc(&self) -> GC_R {
        GC_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Receive buffer under"]
    #[inline(always)]
    #[must_use]
    pub fn rx_under(&mut self) -> RX_UNDER_W<0> {
        RX_UNDER_W::new(self)
    }
    #[doc = "Bit 1 - Receive buffer over"]
    #[inline(always)]
    #[must_use]
    pub fn rx_over(&mut self) -> RX_OVER_W<1> {
        RX_OVER_W::new(self)
    }
    #[doc = "Bit 2 - Receive buffer not empty"]
    #[inline(always)]
    #[must_use]
    pub fn rx_full(&mut self) -> RX_FULL_W<2> {
        RX_FULL_W::new(self)
    }
    #[doc = "Bit 3 - Transmit buffer over"]
    #[inline(always)]
    #[must_use]
    pub fn tx_over(&mut self) -> TX_OVER_W<3> {
        TX_OVER_W::new(self)
    }
    #[doc = "Bit 4 - Transmit buffer empty"]
    #[inline(always)]
    #[must_use]
    pub fn tx_empty(&mut self) -> TX_EMPTY_W<4> {
        TX_EMPTY_W::new(self)
    }
    #[doc = "Bit 5 - Read request"]
    #[inline(always)]
    #[must_use]
    pub fn rd_req(&mut self) -> RD_REQ_W<5> {
        RD_REQ_W::new(self)
    }
    #[doc = "Bit 6 - Transmit abort"]
    #[inline(always)]
    #[must_use]
    pub fn tx_abrt(&mut self) -> TX_ABRT_W<6> {
        TX_ABRT_W::new(self)
    }
    #[doc = "Bit 7 - Transmit done"]
    #[inline(always)]
    #[must_use]
    pub fn rx_done(&mut self) -> RX_DONE_W<7> {
        RX_DONE_W::new(self)
    }
    #[doc = "Bit 8 - This bit captures DW_spb_i2c acticity and stays set until it is cleared"]
    #[inline(always)]
    #[must_use]
    pub fn activ(&mut self) -> ACTIV_W<8> {
        ACTIV_W::new(self)
    }
    #[doc = "Bit 9 - Stop condition detection"]
    #[inline(always)]
    #[must_use]
    pub fn stop(&mut self) -> STOP_W<9> {
        STOP_W::new(self)
    }
    #[doc = "Bit 10 - Start condition detection"]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> START_W<10> {
        START_W::new(self)
    }
    #[doc = "Bit 11 - General call"]
    #[inline(always)]
    #[must_use]
    pub fn gc(&mut self) -> GC_W<11> {
        GC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [imr](index.html) module"]
pub struct IMR_SPEC;
impl crate::RegisterSpec for IMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [imr::R](R) reader structure"]
impl crate::Readable for IMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [imr::W](W) writer structure"]
impl crate::Writable for IMR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IMR to value 0x08ff"]
impl crate::Resettable for IMR_SPEC {
    const RESET_VALUE: Self::Ux = 0x08ff;
}
