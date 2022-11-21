#[doc = "Register `FCR` reader"]
pub struct R(crate::R<FCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FCR` writer"]
pub struct W(crate::W<FCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FCR_SPEC>;
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
impl From<crate::W<FCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FCBBPA` reader - Flow Control Busy/Back Pressure Activate"]
pub type FCBBPA_R = crate::BitReader<bool>;
#[doc = "Field `FCBBPA` writer - Flow Control Busy/Back Pressure Activate"]
pub type FCBBPA_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR_SPEC, bool, O>;
#[doc = "Field `FTE` reader - Transmit Flow Control Enable"]
pub type FTE_R = crate::BitReader<bool>;
#[doc = "Field `FTE` writer - Transmit Flow Control Enable"]
pub type FTE_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR_SPEC, bool, O>;
#[doc = "Field `FRE` reader - Receive Flow Control Enable"]
pub type FRE_R = crate::BitReader<bool>;
#[doc = "Field `FRE` writer - Receive Flow Control Enable"]
pub type FRE_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR_SPEC, bool, O>;
#[doc = "Field `UP` reader - Unicast Pause Frame Detect"]
pub type UP_R = crate::BitReader<bool>;
#[doc = "Field `UP` writer - Unicast Pause Frame Detect"]
pub type UP_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR_SPEC, bool, O>;
#[doc = "Field `PLT` reader - Pause Low Threshold"]
pub type PLT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PLT` writer - Pause Low Threshold"]
pub type PLT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FCR_SPEC, u8, u8, 2, O>;
#[doc = "Field `DZPQ` reader - When the flow control signal is valid, it is forbidden to generate a zero-slice pause control frame"]
pub type DZPQ_R = crate::BitReader<bool>;
#[doc = "Field `DZPQ` writer - When the flow control signal is valid, it is forbidden to generate a zero-slice pause control frame"]
pub type DZPQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR_SPEC, bool, O>;
#[doc = "Field `PSET` reader - Pause time"]
pub type PSET_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PSET` writer - Pause time"]
pub type PSET_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FCR_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bit 0 - Flow Control Busy/Back Pressure Activate"]
    #[inline(always)]
    pub fn fcbbpa(&self) -> FCBBPA_R {
        FCBBPA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit Flow Control Enable"]
    #[inline(always)]
    pub fn fte(&self) -> FTE_R {
        FTE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Receive Flow Control Enable"]
    #[inline(always)]
    pub fn fre(&self) -> FRE_R {
        FRE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Unicast Pause Frame Detect"]
    #[inline(always)]
    pub fn up(&self) -> UP_R {
        UP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Pause Low Threshold"]
    #[inline(always)]
    pub fn plt(&self) -> PLT_R {
        PLT_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 7 - When the flow control signal is valid, it is forbidden to generate a zero-slice pause control frame"]
    #[inline(always)]
    pub fn dzpq(&self) -> DZPQ_R {
        DZPQ_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 16:31 - Pause time"]
    #[inline(always)]
    pub fn pset(&self) -> PSET_R {
        PSET_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Flow Control Busy/Back Pressure Activate"]
    #[inline(always)]
    #[must_use]
    pub fn fcbbpa(&mut self) -> FCBBPA_W<0> {
        FCBBPA_W::new(self)
    }
    #[doc = "Bit 1 - Transmit Flow Control Enable"]
    #[inline(always)]
    #[must_use]
    pub fn fte(&mut self) -> FTE_W<1> {
        FTE_W::new(self)
    }
    #[doc = "Bit 2 - Receive Flow Control Enable"]
    #[inline(always)]
    #[must_use]
    pub fn fre(&mut self) -> FRE_W<2> {
        FRE_W::new(self)
    }
    #[doc = "Bit 3 - Unicast Pause Frame Detect"]
    #[inline(always)]
    #[must_use]
    pub fn up(&mut self) -> UP_W<3> {
        UP_W::new(self)
    }
    #[doc = "Bits 4:5 - Pause Low Threshold"]
    #[inline(always)]
    #[must_use]
    pub fn plt(&mut self) -> PLT_W<4> {
        PLT_W::new(self)
    }
    #[doc = "Bit 7 - When the flow control signal is valid, it is forbidden to generate a zero-slice pause control frame"]
    #[inline(always)]
    #[must_use]
    pub fn dzpq(&mut self) -> DZPQ_W<7> {
        DZPQ_W::new(self)
    }
    #[doc = "Bits 16:31 - Pause time"]
    #[inline(always)]
    #[must_use]
    pub fn pset(&mut self) -> PSET_W<16> {
        PSET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ethernet MAC flow control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fcr](index.html) module"]
pub struct FCR_SPEC;
impl crate::RegisterSpec for FCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fcr::R](R) reader structure"]
impl crate::Readable for FCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fcr::W](W) writer structure"]
impl crate::Writable for FCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FCR to value 0"]
impl crate::Resettable for FCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
