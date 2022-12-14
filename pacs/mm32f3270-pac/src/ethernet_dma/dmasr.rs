#[doc = "Register `DMASR` reader"]
pub struct R(crate::R<DMASR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMASR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMASR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMASR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMASR` writer"]
pub struct W(crate::W<DMASR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMASR_SPEC>;
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
impl From<crate::W<DMASR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMASR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIS` reader - Transmit Interrupt Status"]
pub type TIS_R = crate::BitReader<bool>;
#[doc = "Field `TIS` writer - Transmit Interrupt Status"]
pub type TIS_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, DMASR_SPEC, bool, O>;
#[doc = "Field `TSS` reader - Transmit Process Stopped Status"]
pub type TSS_R = crate::BitReader<bool>;
#[doc = "Field `TSS` writer - Transmit Process Stopped Status"]
pub type TSS_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, DMASR_SPEC, bool, O>;
#[doc = "Field `TUS` reader - Transmit Buffer Unavailable Status"]
pub type TUS_R = crate::BitReader<bool>;
#[doc = "Field `TUS` writer - Transmit Buffer Unavailable Status"]
pub type TUS_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, DMASR_SPEC, bool, O>;
#[doc = "Field `TJS` reader - Transmit Jabber Timeout Status"]
pub type TJS_R = crate::BitReader<bool>;
#[doc = "Field `TJS` writer - Transmit Jabber Timeout Status"]
pub type TJS_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, DMASR_SPEC, bool, O>;
#[doc = "Field `OVS` reader - Receive Overflow Status"]
pub type OVS_R = crate::BitReader<bool>;
#[doc = "Field `OVS` writer - Receive Overflow Status"]
pub type OVS_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, DMASR_SPEC, bool, O>;
#[doc = "Field `UNS` reader - Transmit Underflow Status"]
pub type UNS_R = crate::BitReader<bool>;
#[doc = "Field `UNS` writer - Transmit Underflow Status"]
pub type UNS_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, DMASR_SPEC, bool, O>;
#[doc = "Field `RIS` reader - Receive Status"]
pub type RIS_R = crate::BitReader<bool>;
#[doc = "Field `RIS` writer - Receive Status"]
pub type RIS_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, DMASR_SPEC, bool, O>;
#[doc = "Field `RUS` reader - Receive Buffer Unavailable Status"]
pub type RUS_R = crate::BitReader<bool>;
#[doc = "Field `RUS` writer - Receive Buffer Unavailable Status"]
pub type RUS_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, DMASR_SPEC, bool, O>;
#[doc = "Field `RSS` reader - Receive Process Stopped Status"]
pub type RSS_R = crate::BitReader<bool>;
#[doc = "Field `RSS` writer - Receive Process Stopped Status"]
pub type RSS_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, DMASR_SPEC, bool, O>;
#[doc = "Field `RWS` reader - Receive Watchdog Timeout Status"]
pub type RWS_R = crate::BitReader<bool>;
#[doc = "Field `RWS` writer - Receive Watchdog Timeout Status"]
pub type RWS_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, DMASR_SPEC, bool, O>;
#[doc = "Field `ETS` reader - Early Transmit Interrupt Status"]
pub type ETS_R = crate::BitReader<bool>;
#[doc = "Field `ETS` writer - Early Transmit Interrupt Status"]
pub type ETS_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, DMASR_SPEC, bool, O>;
#[doc = "Field `FBS` reader - Fatal Bus Error Interrupt Status"]
pub type FBS_R = crate::BitReader<bool>;
#[doc = "Field `FBS` writer - Fatal Bus Error Interrupt Status"]
pub type FBS_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, DMASR_SPEC, bool, O>;
#[doc = "Field `ERS` reader - Early Receive Interrupt Status"]
pub type ERS_R = crate::BitReader<bool>;
#[doc = "Field `ERS` writer - Early Receive Interrupt Status"]
pub type ERS_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, DMASR_SPEC, bool, O>;
#[doc = "Field `AIS` reader - Abnormal Interrupt Summary"]
pub type AIS_R = crate::BitReader<bool>;
#[doc = "Field `AIS` writer - Abnormal Interrupt Summary"]
pub type AIS_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, DMASR_SPEC, bool, O>;
#[doc = "Field `NIS` reader - Normal Interrupt Summary"]
pub type NIS_R = crate::BitReader<bool>;
#[doc = "Field `NIS` writer - Normal Interrupt Summary"]
pub type NIS_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, DMASR_SPEC, bool, O>;
#[doc = "Field `RPS` reader - Receive Process State"]
pub type RPS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TPS` reader - Transmit Process State"]
pub type TPS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EBUS` reader - Error Bits Status"]
pub type EBUS_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bit 0 - Transmit Interrupt Status"]
    #[inline(always)]
    pub fn tis(&self) -> TIS_R {
        TIS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit Process Stopped Status"]
    #[inline(always)]
    pub fn tss(&self) -> TSS_R {
        TSS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmit Buffer Unavailable Status"]
    #[inline(always)]
    pub fn tus(&self) -> TUS_R {
        TUS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmit Jabber Timeout Status"]
    #[inline(always)]
    pub fn tjs(&self) -> TJS_R {
        TJS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Receive Overflow Status"]
    #[inline(always)]
    pub fn ovs(&self) -> OVS_R {
        OVS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Transmit Underflow Status"]
    #[inline(always)]
    pub fn uns(&self) -> UNS_R {
        UNS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Receive Status"]
    #[inline(always)]
    pub fn ris(&self) -> RIS_R {
        RIS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Receive Buffer Unavailable Status"]
    #[inline(always)]
    pub fn rus(&self) -> RUS_R {
        RUS_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Receive Process Stopped Status"]
    #[inline(always)]
    pub fn rss(&self) -> RSS_R {
        RSS_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Receive Watchdog Timeout Status"]
    #[inline(always)]
    pub fn rws(&self) -> RWS_R {
        RWS_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Early Transmit Interrupt Status"]
    #[inline(always)]
    pub fn ets(&self) -> ETS_R {
        ETS_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 13 - Fatal Bus Error Interrupt Status"]
    #[inline(always)]
    pub fn fbs(&self) -> FBS_R {
        FBS_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Early Receive Interrupt Status"]
    #[inline(always)]
    pub fn ers(&self) -> ERS_R {
        ERS_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Abnormal Interrupt Summary"]
    #[inline(always)]
    pub fn ais(&self) -> AIS_R {
        AIS_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Normal Interrupt Summary"]
    #[inline(always)]
    pub fn nis(&self) -> NIS_R {
        NIS_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:19 - Receive Process State"]
    #[inline(always)]
    pub fn rps(&self) -> RPS_R {
        RPS_R::new(((self.bits >> 17) & 7) as u8)
    }
    #[doc = "Bits 20:22 - Transmit Process State"]
    #[inline(always)]
    pub fn tps(&self) -> TPS_R {
        TPS_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 23:25 - Error Bits Status"]
    #[inline(always)]
    pub fn ebus(&self) -> EBUS_R {
        EBUS_R::new(((self.bits >> 23) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Transmit Interrupt Status"]
    #[inline(always)]
    #[must_use]
    pub fn tis(&mut self) -> TIS_W<0> {
        TIS_W::new(self)
    }
    #[doc = "Bit 1 - Transmit Process Stopped Status"]
    #[inline(always)]
    #[must_use]
    pub fn tss(&mut self) -> TSS_W<1> {
        TSS_W::new(self)
    }
    #[doc = "Bit 2 - Transmit Buffer Unavailable Status"]
    #[inline(always)]
    #[must_use]
    pub fn tus(&mut self) -> TUS_W<2> {
        TUS_W::new(self)
    }
    #[doc = "Bit 3 - Transmit Jabber Timeout Status"]
    #[inline(always)]
    #[must_use]
    pub fn tjs(&mut self) -> TJS_W<3> {
        TJS_W::new(self)
    }
    #[doc = "Bit 4 - Receive Overflow Status"]
    #[inline(always)]
    #[must_use]
    pub fn ovs(&mut self) -> OVS_W<4> {
        OVS_W::new(self)
    }
    #[doc = "Bit 5 - Transmit Underflow Status"]
    #[inline(always)]
    #[must_use]
    pub fn uns(&mut self) -> UNS_W<5> {
        UNS_W::new(self)
    }
    #[doc = "Bit 6 - Receive Status"]
    #[inline(always)]
    #[must_use]
    pub fn ris(&mut self) -> RIS_W<6> {
        RIS_W::new(self)
    }
    #[doc = "Bit 7 - Receive Buffer Unavailable Status"]
    #[inline(always)]
    #[must_use]
    pub fn rus(&mut self) -> RUS_W<7> {
        RUS_W::new(self)
    }
    #[doc = "Bit 8 - Receive Process Stopped Status"]
    #[inline(always)]
    #[must_use]
    pub fn rss(&mut self) -> RSS_W<8> {
        RSS_W::new(self)
    }
    #[doc = "Bit 9 - Receive Watchdog Timeout Status"]
    #[inline(always)]
    #[must_use]
    pub fn rws(&mut self) -> RWS_W<9> {
        RWS_W::new(self)
    }
    #[doc = "Bit 10 - Early Transmit Interrupt Status"]
    #[inline(always)]
    #[must_use]
    pub fn ets(&mut self) -> ETS_W<10> {
        ETS_W::new(self)
    }
    #[doc = "Bit 13 - Fatal Bus Error Interrupt Status"]
    #[inline(always)]
    #[must_use]
    pub fn fbs(&mut self) -> FBS_W<13> {
        FBS_W::new(self)
    }
    #[doc = "Bit 14 - Early Receive Interrupt Status"]
    #[inline(always)]
    #[must_use]
    pub fn ers(&mut self) -> ERS_W<14> {
        ERS_W::new(self)
    }
    #[doc = "Bit 15 - Abnormal Interrupt Summary"]
    #[inline(always)]
    #[must_use]
    pub fn ais(&mut self) -> AIS_W<15> {
        AIS_W::new(self)
    }
    #[doc = "Bit 16 - Normal Interrupt Summary"]
    #[inline(always)]
    #[must_use]
    pub fn nis(&mut self) -> NIS_W<16> {
        NIS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ethernet DMA status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmasr](index.html) module"]
pub struct DMASR_SPEC;
impl crate::RegisterSpec for DMASR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmasr::R](R) reader structure"]
impl crate::Readable for DMASR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmasr::W](W) writer structure"]
impl crate::Writable for DMASR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x0001_e7ff;
}
#[doc = "`reset()` method sets DMASR to value 0"]
impl crate::Resettable for DMASR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
