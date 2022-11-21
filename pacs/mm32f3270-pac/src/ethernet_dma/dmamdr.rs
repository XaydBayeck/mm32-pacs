#[doc = "Register `DMAMDR` reader"]
pub struct R(crate::R<DMAMDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMAMDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMAMDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMAMDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMAMDR` writer"]
pub struct W(crate::W<DMAMDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMAMDR_SPEC>;
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
impl From<crate::W<DMAMDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMAMDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STR` reader - Start/Stop Receive"]
pub type STR_R = crate::BitReader<bool>;
#[doc = "Field `STR` writer - Start/Stop Receive"]
pub type STR_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMAMDR_SPEC, bool, O>;
#[doc = "Field `OSF` reader - Operate on second frame"]
pub type OSF_R = crate::BitReader<bool>;
#[doc = "Field `OSF` writer - Operate on second frame"]
pub type OSF_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMAMDR_SPEC, bool, O>;
#[doc = "Field `RTC` reader - Receive threshold control"]
pub type RTC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RTC` writer - Receive threshold control"]
pub type RTC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DMAMDR_SPEC, u8, u8, 2, O>;
#[doc = "Field `DGF` reader - RxFIFO discards jumbo frames"]
pub type DGF_R = crate::BitReader<bool>;
#[doc = "Field `DGF` writer - RxFIFO discards jumbo frames"]
pub type DGF_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMAMDR_SPEC, bool, O>;
#[doc = "Field `FUF` reader - Forward undersized goodframes"]
pub type FUF_R = crate::BitReader<bool>;
#[doc = "Field `FUF` writer - Forward undersized goodframes"]
pub type FUF_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMAMDR_SPEC, bool, O>;
#[doc = "Field `FEF` reader - For Error Frames"]
pub type FEF_R = crate::BitReader<bool>;
#[doc = "Field `FEF` writer - For Error Frames"]
pub type FEF_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMAMDR_SPEC, bool, O>;
#[doc = "Field `STT` reader - Start/stop transmission"]
pub type STT_R = crate::BitReader<bool>;
#[doc = "Field `STT` writer - Start/stop transmission"]
pub type STT_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMAMDR_SPEC, bool, O>;
#[doc = "Field `TTC` reader - Transmit threshold control"]
pub type TTC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TTC` writer - Transmit threshold control"]
pub type TTC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DMAMDR_SPEC, u8, u8, 3, O>;
#[doc = "Field `FTF` reader - Flush Transmit FIFO"]
pub type FTF_R = crate::BitReader<bool>;
#[doc = "Field `FTF` writer - Flush Transmit FIFO"]
pub type FTF_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMAMDR_SPEC, bool, O>;
#[doc = "Field `TSF` reader - Transmit Store and Forward"]
pub type TSF_R = crate::BitReader<bool>;
#[doc = "Field `TSF` writer - Transmit Store and Forward"]
pub type TSF_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMAMDR_SPEC, bool, O>;
#[doc = "Field `DFRF` reader - When this bit is set to 1, RxDMA will not refresh any frame because the receive descriptor/buffer is unavailable, please refer to the section \"Receiving Process Suspended\""]
pub type DFRF_R = crate::BitReader<bool>;
#[doc = "Field `DFRF` writer - When this bit is set to 1, RxDMA will not refresh any frame because the receive descriptor/buffer is unavailable, please refer to the section \"Receiving Process Suspended\""]
pub type DFRF_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMAMDR_SPEC, bool, O>;
#[doc = "Field `RSF` reader - After the RxFIFO writes a complete frame, it executes the read action while ignoring the RTC bit"]
pub type RSF_R = crate::BitReader<bool>;
#[doc = "Field `RSF` writer - After the RxFIFO writes a complete frame, it executes the read action while ignoring the RTC bit"]
pub type RSF_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMAMDR_SPEC, bool, O>;
#[doc = "Field `DTCOE` reader - If there are only errors detected by the receive checksum offload engine in the received frame, the kernel will not discard the frame"]
pub type DTCOE_R = crate::BitReader<bool>;
#[doc = "Field `DTCOE` writer - If there are only errors detected by the receive checksum offload engine in the received frame, the kernel will not discard the frame"]
pub type DTCOE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMAMDR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 1 - Start/Stop Receive"]
    #[inline(always)]
    pub fn str(&self) -> STR_R {
        STR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Operate on second frame"]
    #[inline(always)]
    pub fn osf(&self) -> OSF_R {
        OSF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - Receive threshold control"]
    #[inline(always)]
    pub fn rtc(&self) -> RTC_R {
        RTC_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5 - RxFIFO discards jumbo frames"]
    #[inline(always)]
    pub fn dgf(&self) -> DGF_R {
        DGF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Forward undersized goodframes"]
    #[inline(always)]
    pub fn fuf(&self) -> FUF_R {
        FUF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - For Error Frames"]
    #[inline(always)]
    pub fn fef(&self) -> FEF_R {
        FEF_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 13 - Start/stop transmission"]
    #[inline(always)]
    pub fn stt(&self) -> STT_R {
        STT_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:16 - Transmit threshold control"]
    #[inline(always)]
    pub fn ttc(&self) -> TTC_R {
        TTC_R::new(((self.bits >> 14) & 7) as u8)
    }
    #[doc = "Bit 20 - Flush Transmit FIFO"]
    #[inline(always)]
    pub fn ftf(&self) -> FTF_R {
        FTF_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Transmit Store and Forward"]
    #[inline(always)]
    pub fn tsf(&self) -> TSF_R {
        TSF_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 24 - When this bit is set to 1, RxDMA will not refresh any frame because the receive descriptor/buffer is unavailable, please refer to the section \"Receiving Process Suspended\""]
    #[inline(always)]
    pub fn dfrf(&self) -> DFRF_R {
        DFRF_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - After the RxFIFO writes a complete frame, it executes the read action while ignoring the RTC bit"]
    #[inline(always)]
    pub fn rsf(&self) -> RSF_R {
        RSF_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - If there are only errors detected by the receive checksum offload engine in the received frame, the kernel will not discard the frame"]
    #[inline(always)]
    pub fn dtcoe(&self) -> DTCOE_R {
        DTCOE_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Start/Stop Receive"]
    #[inline(always)]
    #[must_use]
    pub fn str(&mut self) -> STR_W<1> {
        STR_W::new(self)
    }
    #[doc = "Bit 2 - Operate on second frame"]
    #[inline(always)]
    #[must_use]
    pub fn osf(&mut self) -> OSF_W<2> {
        OSF_W::new(self)
    }
    #[doc = "Bits 3:4 - Receive threshold control"]
    #[inline(always)]
    #[must_use]
    pub fn rtc(&mut self) -> RTC_W<3> {
        RTC_W::new(self)
    }
    #[doc = "Bit 5 - RxFIFO discards jumbo frames"]
    #[inline(always)]
    #[must_use]
    pub fn dgf(&mut self) -> DGF_W<5> {
        DGF_W::new(self)
    }
    #[doc = "Bit 6 - Forward undersized goodframes"]
    #[inline(always)]
    #[must_use]
    pub fn fuf(&mut self) -> FUF_W<6> {
        FUF_W::new(self)
    }
    #[doc = "Bit 7 - For Error Frames"]
    #[inline(always)]
    #[must_use]
    pub fn fef(&mut self) -> FEF_W<7> {
        FEF_W::new(self)
    }
    #[doc = "Bit 13 - Start/stop transmission"]
    #[inline(always)]
    #[must_use]
    pub fn stt(&mut self) -> STT_W<13> {
        STT_W::new(self)
    }
    #[doc = "Bits 14:16 - Transmit threshold control"]
    #[inline(always)]
    #[must_use]
    pub fn ttc(&mut self) -> TTC_W<14> {
        TTC_W::new(self)
    }
    #[doc = "Bit 20 - Flush Transmit FIFO"]
    #[inline(always)]
    #[must_use]
    pub fn ftf(&mut self) -> FTF_W<20> {
        FTF_W::new(self)
    }
    #[doc = "Bit 21 - Transmit Store and Forward"]
    #[inline(always)]
    #[must_use]
    pub fn tsf(&mut self) -> TSF_W<21> {
        TSF_W::new(self)
    }
    #[doc = "Bit 24 - When this bit is set to 1, RxDMA will not refresh any frame because the receive descriptor/buffer is unavailable, please refer to the section \"Receiving Process Suspended\""]
    #[inline(always)]
    #[must_use]
    pub fn dfrf(&mut self) -> DFRF_W<24> {
        DFRF_W::new(self)
    }
    #[doc = "Bit 25 - After the RxFIFO writes a complete frame, it executes the read action while ignoring the RTC bit"]
    #[inline(always)]
    #[must_use]
    pub fn rsf(&mut self) -> RSF_W<25> {
        RSF_W::new(self)
    }
    #[doc = "Bit 26 - If there are only errors detected by the receive checksum offload engine in the received frame, the kernel will not discard the frame"]
    #[inline(always)]
    #[must_use]
    pub fn dtcoe(&mut self) -> DTCOE_W<26> {
        DTCOE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ethernet DMA operation mode register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmamdr](index.html) module"]
pub struct DMAMDR_SPEC;
impl crate::RegisterSpec for DMAMDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmamdr::R](R) reader structure"]
impl crate::Readable for DMAMDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmamdr::W](W) writer structure"]
impl crate::Writable for DMAMDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMAMDR to value 0"]
impl crate::Resettable for DMAMDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
