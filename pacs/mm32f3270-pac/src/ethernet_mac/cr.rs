#[doc = "Register `CR` reader"]
pub struct R(crate::R<CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR` writer"]
pub struct W(crate::W<CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR_SPEC>;
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
impl From<crate::W<CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RE` reader - Receiver enable"]
pub type RE_R = crate::BitReader<bool>;
#[doc = "Field `RE` writer - Receiver enable"]
pub type RE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `TE` reader - Transmitter enable"]
pub type TE_R = crate::BitReader<bool>;
#[doc = "Field `TE` writer - Transmitter enable"]
pub type TE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `DLYC` reader - Deferral check"]
pub type DLYC_R = crate::BitReader<bool>;
#[doc = "Field `DLYC` writer - Deferral check"]
pub type DLYC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `BL` reader - Back-off limit"]
pub type BL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BL` writer - Back-off limit"]
pub type BL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u8, u8, 2, O>;
#[doc = "Field `APCS` reader - Automatic pad/CRC stripping"]
pub type APCS_R = crate::BitReader<bool>;
#[doc = "Field `APCS` writer - Automatic pad/CRC stripping"]
pub type APCS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `RETY` reader - Retry disable"]
pub type RETY_R = crate::BitReader<bool>;
#[doc = "Field `RETY` writer - Retry disable"]
pub type RETY_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `IPCO` reader - IPv4 Checksum Offload"]
pub type IPCO_R = crate::BitReader<bool>;
#[doc = "Field `IPCO` writer - IPv4 Checksum Offload"]
pub type IPCO_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `DM` reader - Duplex Mode"]
pub type DM_R = crate::BitReader<bool>;
#[doc = "Field `DM` writer - Duplex Mode"]
pub type DM_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `LM` reader - Loopback Mode"]
pub type LM_R = crate::BitReader<bool>;
#[doc = "Field `LM` writer - Loopback Mode"]
pub type LM_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `ROD` reader - Receive Own Disable"]
pub type ROD_R = crate::BitReader<bool>;
#[doc = "Field `ROD` writer - Receive Own Disable"]
pub type ROD_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `FES` reader - Fast Ethernet Speed"]
pub type FES_R = crate::BitReader<bool>;
#[doc = "Field `FES` writer - Fast Ethernet Speed"]
pub type FES_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `MCRS` reader - Carrier sense disable"]
pub type MCRS_R = crate::BitReader<bool>;
#[doc = "Field `MCRS` writer - Carrier sense disable"]
pub type MCRS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `IFG` reader - Inter Frame Gap"]
pub type IFG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IFG` writer - Inter Frame Gap"]
pub type IFG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u8, u8, 3, O>;
#[doc = "Field `JD` reader - Jabber Disable"]
pub type JD_R = crate::BitReader<bool>;
#[doc = "Field `JD` writer - Jabber Disable"]
pub type JD_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `WTD` reader - Watchdog Disable"]
pub type WTD_R = crate::BitReader<bool>;
#[doc = "Field `WTD` writer - Watchdog Disable"]
pub type WTD_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `CST` reader - CRC stripping for Type frames"]
pub type CST_R = crate::BitReader<bool>;
#[doc = "Field `CST` writer - CRC stripping for Type frames"]
pub type CST_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 2 - Receiver enable"]
    #[inline(always)]
    pub fn re(&self) -> RE_R {
        RE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmitter enable"]
    #[inline(always)]
    pub fn te(&self) -> TE_R {
        TE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Deferral check"]
    #[inline(always)]
    pub fn dlyc(&self) -> DLYC_R {
        DLYC_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - Back-off limit"]
    #[inline(always)]
    pub fn bl(&self) -> BL_R {
        BL_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - Automatic pad/CRC stripping"]
    #[inline(always)]
    pub fn apcs(&self) -> APCS_R {
        APCS_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - Retry disable"]
    #[inline(always)]
    pub fn rety(&self) -> RETY_R {
        RETY_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - IPv4 Checksum Offload"]
    #[inline(always)]
    pub fn ipco(&self) -> IPCO_R {
        IPCO_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Duplex Mode"]
    #[inline(always)]
    pub fn dm(&self) -> DM_R {
        DM_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Loopback Mode"]
    #[inline(always)]
    pub fn lm(&self) -> LM_R {
        LM_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Receive Own Disable"]
    #[inline(always)]
    pub fn rod(&self) -> ROD_R {
        ROD_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Fast Ethernet Speed"]
    #[inline(always)]
    pub fn fes(&self) -> FES_R {
        FES_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - Carrier sense disable"]
    #[inline(always)]
    pub fn mcrs(&self) -> MCRS_R {
        MCRS_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:19 - Inter Frame Gap"]
    #[inline(always)]
    pub fn ifg(&self) -> IFG_R {
        IFG_R::new(((self.bits >> 17) & 7) as u8)
    }
    #[doc = "Bit 22 - Jabber Disable"]
    #[inline(always)]
    pub fn jd(&self) -> JD_R {
        JD_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Watchdog Disable"]
    #[inline(always)]
    pub fn wtd(&self) -> WTD_R {
        WTD_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 25 - CRC stripping for Type frames"]
    #[inline(always)]
    pub fn cst(&self) -> CST_R {
        CST_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Receiver enable"]
    #[inline(always)]
    #[must_use]
    pub fn re(&mut self) -> RE_W<2> {
        RE_W::new(self)
    }
    #[doc = "Bit 3 - Transmitter enable"]
    #[inline(always)]
    #[must_use]
    pub fn te(&mut self) -> TE_W<3> {
        TE_W::new(self)
    }
    #[doc = "Bit 4 - Deferral check"]
    #[inline(always)]
    #[must_use]
    pub fn dlyc(&mut self) -> DLYC_W<4> {
        DLYC_W::new(self)
    }
    #[doc = "Bits 5:6 - Back-off limit"]
    #[inline(always)]
    #[must_use]
    pub fn bl(&mut self) -> BL_W<5> {
        BL_W::new(self)
    }
    #[doc = "Bit 7 - Automatic pad/CRC stripping"]
    #[inline(always)]
    #[must_use]
    pub fn apcs(&mut self) -> APCS_W<7> {
        APCS_W::new(self)
    }
    #[doc = "Bit 9 - Retry disable"]
    #[inline(always)]
    #[must_use]
    pub fn rety(&mut self) -> RETY_W<9> {
        RETY_W::new(self)
    }
    #[doc = "Bit 10 - IPv4 Checksum Offload"]
    #[inline(always)]
    #[must_use]
    pub fn ipco(&mut self) -> IPCO_W<10> {
        IPCO_W::new(self)
    }
    #[doc = "Bit 11 - Duplex Mode"]
    #[inline(always)]
    #[must_use]
    pub fn dm(&mut self) -> DM_W<11> {
        DM_W::new(self)
    }
    #[doc = "Bit 12 - Loopback Mode"]
    #[inline(always)]
    #[must_use]
    pub fn lm(&mut self) -> LM_W<12> {
        LM_W::new(self)
    }
    #[doc = "Bit 13 - Receive Own Disable"]
    #[inline(always)]
    #[must_use]
    pub fn rod(&mut self) -> ROD_W<13> {
        ROD_W::new(self)
    }
    #[doc = "Bit 14 - Fast Ethernet Speed"]
    #[inline(always)]
    #[must_use]
    pub fn fes(&mut self) -> FES_W<14> {
        FES_W::new(self)
    }
    #[doc = "Bit 16 - Carrier sense disable"]
    #[inline(always)]
    #[must_use]
    pub fn mcrs(&mut self) -> MCRS_W<16> {
        MCRS_W::new(self)
    }
    #[doc = "Bits 17:19 - Inter Frame Gap"]
    #[inline(always)]
    #[must_use]
    pub fn ifg(&mut self) -> IFG_W<17> {
        IFG_W::new(self)
    }
    #[doc = "Bit 22 - Jabber Disable"]
    #[inline(always)]
    #[must_use]
    pub fn jd(&mut self) -> JD_W<22> {
        JD_W::new(self)
    }
    #[doc = "Bit 23 - Watchdog Disable"]
    #[inline(always)]
    #[must_use]
    pub fn wtd(&mut self) -> WTD_W<23> {
        WTD_W::new(self)
    }
    #[doc = "Bit 25 - CRC stripping for Type frames"]
    #[inline(always)]
    #[must_use]
    pub fn cst(&mut self) -> CST_W<25> {
        CST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ethernet MAC configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](index.html) module"]
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr::R](R) reader structure"]
impl crate::Readable for CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr::W](W) writer structure"]
impl crate::Writable for CR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CR to value 0x8000"]
impl crate::Resettable for CR_SPEC {
    const RESET_VALUE: Self::Ux = 0x8000;
}
