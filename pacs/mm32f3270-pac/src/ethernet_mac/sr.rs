#[doc = "Register `SR` reader"]
pub struct R(crate::R<SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SR` writer"]
pub struct W(crate::W<SR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SR_SPEC>;
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
impl From<crate::W<SR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MRE` reader - VLAN tag identifier (for receive frames)"]
pub type MRE_R = crate::BitReader<bool>;
#[doc = "Field `MRE` writer - VLAN tag identifier (for receive frames)"]
pub type MRE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
#[doc = "Field `MRS` reader - The FIFO read/write controller in the MAC receiver is working"]
pub type MRS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MRS` writer - The FIFO read/write controller in the MAC receiver is working"]
pub type MRS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SR_SPEC, u8, u8, 2, O>;
#[doc = "Field `RXFWA` reader - RxFIFO write is valid, and the received frame is being transferred to RxFIFO"]
pub type RXFWA_R = crate::BitReader<bool>;
#[doc = "Field `RXFWA` writer - RxFIFO write is valid, and the received frame is being transferred to RxFIFO"]
pub type RXFWA_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
#[doc = "Field `RXFRS` reader - Refresh frame data and status"]
pub type RXFRS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RXFRS` writer - Refresh frame data and status"]
pub type RXFRS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SR_SPEC, u8, u8, 2, O>;
#[doc = "Field `RXFLL` reader - RX Fifo Full"]
pub type RXFLL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RXFLL` writer - RX Fifo Full"]
pub type RXFLL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SR_SPEC, u8, u8, 2, O>;
#[doc = "Field `MTE` reader - The MAC MII transmission engine is actively sending data but not in an idle state"]
pub type MTE_R = crate::BitReader<bool>;
#[doc = "Field `MTE` writer - The MAC MII transmission engine is actively sending data but not in an idle state"]
pub type MTE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
#[doc = "Field `MTS` reader - Transmit the input frame to be sent"]
pub type MTS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MTS` writer - Transmit the input frame to be sent"]
pub type MTS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SR_SPEC, u8, u8, 2, O>;
#[doc = "Field `MTP` reader - The MAC transmitter is in a paused state (when in full duplex mode), and no frame transmission is scheduled"]
pub type MTP_R = crate::BitReader<bool>;
#[doc = "Field `MTP` writer - The MAC transmitter is in a paused state (when in full duplex mode), and no frame transmission is scheduled"]
pub type MTP_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
#[doc = "Field `TXFRS` reader - Write the received Tx Status or refresh the TxFIFO"]
pub type TXFRS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TXFRS` writer - Write the received Tx Status or refresh the TxFIFO"]
pub type TXFRS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SR_SPEC, u8, u8, 2, O>;
#[doc = "Field `TXFWA` reader - The TxFIFO write controller is valid and is transferring data to the TxFIFO"]
pub type TXFWA_R = crate::BitReader<bool>;
#[doc = "Field `TXFWA` writer - The TxFIFO write controller is valid and is transferring data to the TxFIFO"]
pub type TXFWA_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
#[doc = "Field `TXFNE` reader - The TxFIFO is not empty, there are still unsent frames"]
pub type TXFNE_R = crate::BitReader<bool>;
#[doc = "Field `TXFNE` writer - The TxFIFO is not empty, there are still unsent frames"]
pub type TXFNE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
#[doc = "Field `TXFF` reader - The TxFIFO is full and can no longer receive frames sent"]
pub type TXFF_R = crate::BitReader<bool>;
#[doc = "Field `TXFF` writer - The TxFIFO is full and can no longer receive frames sent"]
pub type TXFF_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - VLAN tag identifier (for receive frames)"]
    #[inline(always)]
    pub fn mre(&self) -> MRE_R {
        MRE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - The FIFO read/write controller in the MAC receiver is working"]
    #[inline(always)]
    pub fn mrs(&self) -> MRS_R {
        MRS_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 4 - RxFIFO write is valid, and the received frame is being transferred to RxFIFO"]
    #[inline(always)]
    pub fn rxfwa(&self) -> RXFWA_R {
        RXFWA_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - Refresh frame data and status"]
    #[inline(always)]
    pub fn rxfrs(&self) -> RXFRS_R {
        RXFRS_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bits 8:9 - RX Fifo Full"]
    #[inline(always)]
    pub fn rxfll(&self) -> RXFLL_R {
        RXFLL_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 16 - The MAC MII transmission engine is actively sending data but not in an idle state"]
    #[inline(always)]
    pub fn mte(&self) -> MTE_R {
        MTE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:18 - Transmit the input frame to be sent"]
    #[inline(always)]
    pub fn mts(&self) -> MTS_R {
        MTS_R::new(((self.bits >> 17) & 3) as u8)
    }
    #[doc = "Bit 19 - The MAC transmitter is in a paused state (when in full duplex mode), and no frame transmission is scheduled"]
    #[inline(always)]
    pub fn mtp(&self) -> MTP_R {
        MTP_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:21 - Write the received Tx Status or refresh the TxFIFO"]
    #[inline(always)]
    pub fn txfrs(&self) -> TXFRS_R {
        TXFRS_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 22 - The TxFIFO write controller is valid and is transferring data to the TxFIFO"]
    #[inline(always)]
    pub fn txfwa(&self) -> TXFWA_R {
        TXFWA_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 24 - The TxFIFO is not empty, there are still unsent frames"]
    #[inline(always)]
    pub fn txfne(&self) -> TXFNE_R {
        TXFNE_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - The TxFIFO is full and can no longer receive frames sent"]
    #[inline(always)]
    pub fn txff(&self) -> TXFF_R {
        TXFF_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - VLAN tag identifier (for receive frames)"]
    #[inline(always)]
    #[must_use]
    pub fn mre(&mut self) -> MRE_W<0> {
        MRE_W::new(self)
    }
    #[doc = "Bits 1:2 - The FIFO read/write controller in the MAC receiver is working"]
    #[inline(always)]
    #[must_use]
    pub fn mrs(&mut self) -> MRS_W<1> {
        MRS_W::new(self)
    }
    #[doc = "Bit 4 - RxFIFO write is valid, and the received frame is being transferred to RxFIFO"]
    #[inline(always)]
    #[must_use]
    pub fn rxfwa(&mut self) -> RXFWA_W<4> {
        RXFWA_W::new(self)
    }
    #[doc = "Bits 5:6 - Refresh frame data and status"]
    #[inline(always)]
    #[must_use]
    pub fn rxfrs(&mut self) -> RXFRS_W<5> {
        RXFRS_W::new(self)
    }
    #[doc = "Bits 8:9 - RX Fifo Full"]
    #[inline(always)]
    #[must_use]
    pub fn rxfll(&mut self) -> RXFLL_W<8> {
        RXFLL_W::new(self)
    }
    #[doc = "Bit 16 - The MAC MII transmission engine is actively sending data but not in an idle state"]
    #[inline(always)]
    #[must_use]
    pub fn mte(&mut self) -> MTE_W<16> {
        MTE_W::new(self)
    }
    #[doc = "Bits 17:18 - Transmit the input frame to be sent"]
    #[inline(always)]
    #[must_use]
    pub fn mts(&mut self) -> MTS_W<17> {
        MTS_W::new(self)
    }
    #[doc = "Bit 19 - The MAC transmitter is in a paused state (when in full duplex mode), and no frame transmission is scheduled"]
    #[inline(always)]
    #[must_use]
    pub fn mtp(&mut self) -> MTP_W<19> {
        MTP_W::new(self)
    }
    #[doc = "Bits 20:21 - Write the received Tx Status or refresh the TxFIFO"]
    #[inline(always)]
    #[must_use]
    pub fn txfrs(&mut self) -> TXFRS_W<20> {
        TXFRS_W::new(self)
    }
    #[doc = "Bit 22 - The TxFIFO write controller is valid and is transferring data to the TxFIFO"]
    #[inline(always)]
    #[must_use]
    pub fn txfwa(&mut self) -> TXFWA_W<22> {
        TXFWA_W::new(self)
    }
    #[doc = "Bit 24 - The TxFIFO is not empty, there are still unsent frames"]
    #[inline(always)]
    #[must_use]
    pub fn txfne(&mut self) -> TXFNE_W<24> {
        TXFNE_W::new(self)
    }
    #[doc = "Bit 25 - The TxFIFO is full and can no longer receive frames sent"]
    #[inline(always)]
    #[must_use]
    pub fn txff(&mut self) -> TXFF_W<25> {
        TXFF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ethernet MAC VLAN tag register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](index.html) module"]
pub struct SR_SPEC;
impl crate::RegisterSpec for SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sr::R](R) reader structure"]
impl crate::Readable for SR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sr::W](W) writer structure"]
impl crate::Writable for SR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
