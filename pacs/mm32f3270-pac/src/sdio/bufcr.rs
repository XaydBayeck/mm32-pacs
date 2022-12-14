#[doc = "Register `BUFCR` reader"]
pub struct R(crate::R<BUFCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BUFCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BUFCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BUFCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BUFCR` writer"]
pub struct W(crate::W<BUFCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BUFCR_SPEC>;
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
impl From<crate::W<BUFCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BUFCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DBF` reader - Data buff full"]
pub type DBF_R = crate::BitReader<bool>;
#[doc = "Field `DBF` writer - Data buff full"]
pub type DBF_W<'a, const O: u8> = crate::BitWriter<'a, u32, BUFCR_SPEC, bool, O>;
#[doc = "Field `DBE` reader - Data buff empty. read-only"]
pub type DBE_R = crate::BitReader<bool>;
#[doc = "Field `DBE` writer - Data buff empty. read-only"]
pub type DBE_W<'a, const O: u8> = crate::BitWriter<'a, u32, BUFCR_SPEC, bool, O>;
#[doc = "Field `DBML` reader - Data buff data water mark level"]
pub type DBML_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DBML` writer - Data buff data water mark level"]
pub type DBML_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BUFCR_SPEC, u8, u8, 8, O>;
#[doc = "Field `DMAHEN` reader - DMA hardware interface enable"]
pub type DMAHEN_R = crate::BitReader<bool>;
#[doc = "Field `DMAHEN` writer - DMA hardware interface enable"]
pub type DMAHEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, BUFCR_SPEC, bool, O>;
#[doc = "Field `BADIR` reader - Set buff access direction"]
pub type BADIR_R = crate::BitReader<bool>;
#[doc = "Field `BADIR` writer - Set buff access direction"]
pub type BADIR_W<'a, const O: u8> = crate::BitWriter<'a, u32, BUFCR_SPEC, bool, O>;
#[doc = "Field `DFIFOSM` reader - Data FIFO status signal mask bit"]
pub type DFIFOSM_R = crate::BitReader<bool>;
#[doc = "Field `DFIFOSM` writer - Data FIFO status signal mask bit"]
pub type DFIFOSM_W<'a, const O: u8> = crate::BitWriter<'a, u32, BUFCR_SPEC, bool, O>;
#[doc = "Field `DMARM` reader - DMA Requst mask"]
pub type DMARM_R = crate::BitReader<bool>;
#[doc = "Field `DMARM` writer - DMA Requst mask"]
pub type DMARM_W<'a, const O: u8> = crate::BitWriter<'a, u32, BUFCR_SPEC, bool, O>;
#[doc = "Field `DBFEN` reader - Data Buf flush enable"]
pub type DBFEN_R = crate::BitReader<bool>;
#[doc = "Field `DBFEN` writer - Data Buf flush enable"]
pub type DBFEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, BUFCR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Data buff full"]
    #[inline(always)]
    pub fn dbf(&self) -> DBF_R {
        DBF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Data buff empty. read-only"]
    #[inline(always)]
    pub fn dbe(&self) -> DBE_R {
        DBE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:9 - Data buff data water mark level"]
    #[inline(always)]
    pub fn dbml(&self) -> DBML_R {
        DBML_R::new(((self.bits >> 2) & 0xff) as u8)
    }
    #[doc = "Bit 10 - DMA hardware interface enable"]
    #[inline(always)]
    pub fn dmahen(&self) -> DMAHEN_R {
        DMAHEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Set buff access direction"]
    #[inline(always)]
    pub fn badir(&self) -> BADIR_R {
        BADIR_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Data FIFO status signal mask bit"]
    #[inline(always)]
    pub fn dfifosm(&self) -> DFIFOSM_R {
        DFIFOSM_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - DMA Requst mask"]
    #[inline(always)]
    pub fn dmarm(&self) -> DMARM_R {
        DMARM_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Data Buf flush enable"]
    #[inline(always)]
    pub fn dbfen(&self) -> DBFEN_R {
        DBFEN_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Data buff full"]
    #[inline(always)]
    #[must_use]
    pub fn dbf(&mut self) -> DBF_W<0> {
        DBF_W::new(self)
    }
    #[doc = "Bit 1 - Data buff empty. read-only"]
    #[inline(always)]
    #[must_use]
    pub fn dbe(&mut self) -> DBE_W<1> {
        DBE_W::new(self)
    }
    #[doc = "Bits 2:9 - Data buff data water mark level"]
    #[inline(always)]
    #[must_use]
    pub fn dbml(&mut self) -> DBML_W<2> {
        DBML_W::new(self)
    }
    #[doc = "Bit 10 - DMA hardware interface enable"]
    #[inline(always)]
    #[must_use]
    pub fn dmahen(&mut self) -> DMAHEN_W<10> {
        DMAHEN_W::new(self)
    }
    #[doc = "Bit 11 - Set buff access direction"]
    #[inline(always)]
    #[must_use]
    pub fn badir(&mut self) -> BADIR_W<11> {
        BADIR_W::new(self)
    }
    #[doc = "Bit 12 - Data FIFO status signal mask bit"]
    #[inline(always)]
    #[must_use]
    pub fn dfifosm(&mut self) -> DFIFOSM_W<12> {
        DFIFOSM_W::new(self)
    }
    #[doc = "Bit 14 - DMA Requst mask"]
    #[inline(always)]
    #[must_use]
    pub fn dmarm(&mut self) -> DMARM_W<14> {
        DMARM_W::new(self)
    }
    #[doc = "Bit 15 - Data Buf flush enable"]
    #[inline(always)]
    #[must_use]
    pub fn dbfen(&mut self) -> DBFEN_W<15> {
        DBFEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Buffer control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bufcr](index.html) module"]
pub struct BUFCR_SPEC;
impl crate::RegisterSpec for BUFCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bufcr::R](R) reader structure"]
impl crate::Readable for BUFCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bufcr::W](W) writer structure"]
impl crate::Writable for BUFCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BUFCR to value 0x02"]
impl crate::Resettable for BUFCR_SPEC {
    const RESET_VALUE: Self::Ux = 0x02;
}
