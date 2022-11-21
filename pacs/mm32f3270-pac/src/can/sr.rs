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
#[doc = "Field `RBS` reader - Receive buffer status"]
pub type RBS_R = crate::BitReader<bool>;
#[doc = "Field `RBS` writer - Receive buffer status"]
pub type RBS_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
#[doc = "Field `DOS` reader - Data overrun status"]
pub type DOS_R = crate::BitReader<bool>;
#[doc = "Field `DOS` writer - Data overrun status"]
pub type DOS_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
#[doc = "Field `TBS` reader - Transmit buffer status"]
pub type TBS_R = crate::BitReader<bool>;
#[doc = "Field `TBS` writer - Transmit buffer status"]
pub type TBS_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
#[doc = "Field `TCS` reader - Transmission complete status"]
pub type TCS_R = crate::BitReader<bool>;
#[doc = "Field `TCS` writer - Transmission complete status"]
pub type TCS_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
#[doc = "Field `RS` reader - Receive status"]
pub type RS_R = crate::BitReader<bool>;
#[doc = "Field `RS` writer - Receive status"]
pub type RS_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
#[doc = "Field `TS` reader - Transmit status"]
pub type TS_R = crate::BitReader<bool>;
#[doc = "Field `TS` writer - Transmit status"]
pub type TS_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
#[doc = "Field `ES` reader - Error status"]
pub type ES_R = crate::BitReader<bool>;
#[doc = "Field `ES` writer - Error status"]
pub type ES_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
#[doc = "Field `BS` reader - Bus status"]
pub type BS_R = crate::BitReader<bool>;
#[doc = "Field `BS` writer - Bus status"]
pub type BS_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Receive buffer status"]
    #[inline(always)]
    pub fn rbs(&self) -> RBS_R {
        RBS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Data overrun status"]
    #[inline(always)]
    pub fn dos(&self) -> DOS_R {
        DOS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmit buffer status"]
    #[inline(always)]
    pub fn tbs(&self) -> TBS_R {
        TBS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmission complete status"]
    #[inline(always)]
    pub fn tcs(&self) -> TCS_R {
        TCS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Receive status"]
    #[inline(always)]
    pub fn rs(&self) -> RS_R {
        RS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Transmit status"]
    #[inline(always)]
    pub fn ts(&self) -> TS_R {
        TS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Error status"]
    #[inline(always)]
    pub fn es(&self) -> ES_R {
        ES_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Bus status"]
    #[inline(always)]
    pub fn bs(&self) -> BS_R {
        BS_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Receive buffer status"]
    #[inline(always)]
    #[must_use]
    pub fn rbs(&mut self) -> RBS_W<0> {
        RBS_W::new(self)
    }
    #[doc = "Bit 1 - Data overrun status"]
    #[inline(always)]
    #[must_use]
    pub fn dos(&mut self) -> DOS_W<1> {
        DOS_W::new(self)
    }
    #[doc = "Bit 2 - Transmit buffer status"]
    #[inline(always)]
    #[must_use]
    pub fn tbs(&mut self) -> TBS_W<2> {
        TBS_W::new(self)
    }
    #[doc = "Bit 3 - Transmission complete status"]
    #[inline(always)]
    #[must_use]
    pub fn tcs(&mut self) -> TCS_W<3> {
        TCS_W::new(self)
    }
    #[doc = "Bit 4 - Receive status"]
    #[inline(always)]
    #[must_use]
    pub fn rs(&mut self) -> RS_W<4> {
        RS_W::new(self)
    }
    #[doc = "Bit 5 - Transmit status"]
    #[inline(always)]
    #[must_use]
    pub fn ts(&mut self) -> TS_W<5> {
        TS_W::new(self)
    }
    #[doc = "Bit 6 - Error status"]
    #[inline(always)]
    #[must_use]
    pub fn es(&mut self) -> ES_W<6> {
        ES_W::new(self)
    }
    #[doc = "Bit 7 - Bus status"]
    #[inline(always)]
    #[must_use]
    pub fn bs(&mut self) -> BS_W<7> {
        BS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](index.html) module"]
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
#[doc = "`reset()` method sets SR to value 0x0c"]
impl crate::Resettable for SR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0c;
}
