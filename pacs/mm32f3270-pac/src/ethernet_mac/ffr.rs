#[doc = "Register `FFR` reader"]
pub struct R(crate::R<FFR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FFR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FFR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FFR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FFR` writer"]
pub struct W(crate::W<FFR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FFR_SPEC>;
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
impl From<crate::W<FFR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FFR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PM` reader - Promiscuous Mode"]
pub type PM_R = crate::BitReader<bool>;
#[doc = "Field `PM` writer - Promiscuous Mode"]
pub type PM_W<'a, const O: u8> = crate::BitWriter<'a, u32, FFR_SPEC, bool, O>;
#[doc = "Field `HU` reader - Hash Unicast"]
pub type HU_R = crate::BitReader<bool>;
#[doc = "Field `HU` writer - Hash Unicast"]
pub type HU_W<'a, const O: u8> = crate::BitWriter<'a, u32, FFR_SPEC, bool, O>;
#[doc = "Field `HM` reader - Hash Multicast"]
pub type HM_R = crate::BitReader<bool>;
#[doc = "Field `HM` writer - Hash Multicast"]
pub type HM_W<'a, const O: u8> = crate::BitWriter<'a, u32, FFR_SPEC, bool, O>;
#[doc = "Field `DAIF` reader - Destination address inverse filter-ing"]
pub type DAIF_R = crate::BitReader<bool>;
#[doc = "Field `DAIF` writer - Destination address inverse filter-ing"]
pub type DAIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, FFR_SPEC, bool, O>;
#[doc = "Field `PMF` reader - Pass All Multicast"]
pub type PMF_R = crate::BitReader<bool>;
#[doc = "Field `PMF` writer - Pass All Multicast"]
pub type PMF_W<'a, const O: u8> = crate::BitWriter<'a, u32, FFR_SPEC, bool, O>;
#[doc = "Field `DBF` reader - Broadcast Frames Disable"]
pub type DBF_R = crate::BitReader<bool>;
#[doc = "Field `DBF` writer - Broadcast Frames Disable"]
pub type DBF_W<'a, const O: u8> = crate::BitWriter<'a, u32, FFR_SPEC, bool, O>;
#[doc = "Field `PCF` reader - Pass Control Frames"]
pub type PCF_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PCF` writer - Pass Control Frames"]
pub type PCF_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FFR_SPEC, u8, u8, 2, O>;
#[doc = "Field `SAIF` reader - Source Address Inverse Filtering"]
pub type SAIF_R = crate::BitReader<bool>;
#[doc = "Field `SAIF` writer - Source Address Inverse Filtering"]
pub type SAIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, FFR_SPEC, bool, O>;
#[doc = "Field `SAF` reader - Source Address Filter"]
pub type SAF_R = crate::BitReader<bool>;
#[doc = "Field `SAF` writer - Source Address Filter"]
pub type SAF_W<'a, const O: u8> = crate::BitWriter<'a, u32, FFR_SPEC, bool, O>;
#[doc = "Field `HPF` reader - Hash or Perfect Filter"]
pub type HPF_R = crate::BitReader<bool>;
#[doc = "Field `HPF` writer - Hash or Perfect Filter"]
pub type HPF_W<'a, const O: u8> = crate::BitWriter<'a, u32, FFR_SPEC, bool, O>;
#[doc = "Field `VTFE` reader - VLAN Filter Enable"]
pub type VTFE_R = crate::BitReader<bool>;
#[doc = "Field `VTFE` writer - VLAN Filter Enable"]
pub type VTFE_W<'a, const O: u8> = crate::BitWriter<'a, u32, FFR_SPEC, bool, O>;
#[doc = "Field `DNTU` reader - The MAC receiver discards all IP packets without TCP or UDP fields"]
pub type DNTU_R = crate::BitReader<bool>;
#[doc = "Field `DNTU` writer - The MAC receiver discards all IP packets without TCP or UDP fields"]
pub type DNTU_W<'a, const O: u8> = crate::BitWriter<'a, u32, FFR_SPEC, bool, O>;
#[doc = "Field `RALL` reader - Receive All"]
pub type RALL_R = crate::BitReader<bool>;
#[doc = "Field `RALL` writer - Receive All"]
pub type RALL_W<'a, const O: u8> = crate::BitWriter<'a, u32, FFR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Promiscuous Mode"]
    #[inline(always)]
    pub fn pm(&self) -> PM_R {
        PM_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Hash Unicast"]
    #[inline(always)]
    pub fn hu(&self) -> HU_R {
        HU_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Hash Multicast"]
    #[inline(always)]
    pub fn hm(&self) -> HM_R {
        HM_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Destination address inverse filter-ing"]
    #[inline(always)]
    pub fn daif(&self) -> DAIF_R {
        DAIF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Pass All Multicast"]
    #[inline(always)]
    pub fn pmf(&self) -> PMF_R {
        PMF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Broadcast Frames Disable"]
    #[inline(always)]
    pub fn dbf(&self) -> DBF_R {
        DBF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Pass Control Frames"]
    #[inline(always)]
    pub fn pcf(&self) -> PCF_R {
        PCF_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - Source Address Inverse Filtering"]
    #[inline(always)]
    pub fn saif(&self) -> SAIF_R {
        SAIF_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Source Address Filter"]
    #[inline(always)]
    pub fn saf(&self) -> SAF_R {
        SAF_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Hash or Perfect Filter"]
    #[inline(always)]
    pub fn hpf(&self) -> HPF_R {
        HPF_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 16 - VLAN Filter Enable"]
    #[inline(always)]
    pub fn vtfe(&self) -> VTFE_R {
        VTFE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 21 - The MAC receiver discards all IP packets without TCP or UDP fields"]
    #[inline(always)]
    pub fn dntu(&self) -> DNTU_R {
        DNTU_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 31 - Receive All"]
    #[inline(always)]
    pub fn rall(&self) -> RALL_R {
        RALL_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Promiscuous Mode"]
    #[inline(always)]
    #[must_use]
    pub fn pm(&mut self) -> PM_W<0> {
        PM_W::new(self)
    }
    #[doc = "Bit 1 - Hash Unicast"]
    #[inline(always)]
    #[must_use]
    pub fn hu(&mut self) -> HU_W<1> {
        HU_W::new(self)
    }
    #[doc = "Bit 2 - Hash Multicast"]
    #[inline(always)]
    #[must_use]
    pub fn hm(&mut self) -> HM_W<2> {
        HM_W::new(self)
    }
    #[doc = "Bit 3 - Destination address inverse filter-ing"]
    #[inline(always)]
    #[must_use]
    pub fn daif(&mut self) -> DAIF_W<3> {
        DAIF_W::new(self)
    }
    #[doc = "Bit 4 - Pass All Multicast"]
    #[inline(always)]
    #[must_use]
    pub fn pmf(&mut self) -> PMF_W<4> {
        PMF_W::new(self)
    }
    #[doc = "Bit 5 - Broadcast Frames Disable"]
    #[inline(always)]
    #[must_use]
    pub fn dbf(&mut self) -> DBF_W<5> {
        DBF_W::new(self)
    }
    #[doc = "Bits 6:7 - Pass Control Frames"]
    #[inline(always)]
    #[must_use]
    pub fn pcf(&mut self) -> PCF_W<6> {
        PCF_W::new(self)
    }
    #[doc = "Bit 8 - Source Address Inverse Filtering"]
    #[inline(always)]
    #[must_use]
    pub fn saif(&mut self) -> SAIF_W<8> {
        SAIF_W::new(self)
    }
    #[doc = "Bit 9 - Source Address Filter"]
    #[inline(always)]
    #[must_use]
    pub fn saf(&mut self) -> SAF_W<9> {
        SAF_W::new(self)
    }
    #[doc = "Bit 10 - Hash or Perfect Filter"]
    #[inline(always)]
    #[must_use]
    pub fn hpf(&mut self) -> HPF_W<10> {
        HPF_W::new(self)
    }
    #[doc = "Bit 16 - VLAN Filter Enable"]
    #[inline(always)]
    #[must_use]
    pub fn vtfe(&mut self) -> VTFE_W<16> {
        VTFE_W::new(self)
    }
    #[doc = "Bit 21 - The MAC receiver discards all IP packets without TCP or UDP fields"]
    #[inline(always)]
    #[must_use]
    pub fn dntu(&mut self) -> DNTU_W<21> {
        DNTU_W::new(self)
    }
    #[doc = "Bit 31 - Receive All"]
    #[inline(always)]
    #[must_use]
    pub fn rall(&mut self) -> RALL_W<31> {
        RALL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ethernet MAC frame filter register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ffr](index.html) module"]
pub struct FFR_SPEC;
impl crate::RegisterSpec for FFR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ffr::R](R) reader structure"]
impl crate::Readable for FFR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ffr::W](W) writer structure"]
impl crate::Writable for FFR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FFR to value 0"]
impl crate::Resettable for FFR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
