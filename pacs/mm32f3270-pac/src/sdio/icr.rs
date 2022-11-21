#[doc = "Register `ICR` reader"]
pub struct R(crate::R<ICR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ICR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ICR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ICR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ICR` writer"]
pub struct W(crate::W<ICR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ICR_SPEC>;
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
impl From<crate::W<ICR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ICR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMDD` reader - CMD complete interrupt mask bit"]
pub type CMDD_R = crate::BitReader<bool>;
#[doc = "Field `CMDD` writer - CMD complete interrupt mask bit"]
pub type CMDD_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
#[doc = "Field `DATD` reader - DAT complete interrupt mask bit"]
pub type DATD_R = crate::BitReader<bool>;
#[doc = "Field `DATD` writer - DAT complete interrupt mask bit"]
pub type DATD_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
#[doc = "Field `DATE` reader - DAT CRC error interrupt mask bit"]
pub type DATE_R = crate::BitReader<bool>;
#[doc = "Field `DATE` writer - DAT CRC error interrupt mask bit"]
pub type DATE_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
#[doc = "Field `CMDE` reader - CMD CRC error interrupt mask bit"]
pub type CMDE_R = crate::BitReader<bool>;
#[doc = "Field `CMDE` writer - CMD CRC error interrupt mask bit"]
pub type CMDE_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
#[doc = "Field `MBD` reader - Multi-block transfer complete interrupt mask bit"]
pub type MBD_R = crate::BitReader<bool>;
#[doc = "Field `MBD` writer - Multi-block transfer complete interrupt mask bit"]
pub type MBD_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
#[doc = "Field `MBT` reader - Multi-block transmission timeout interrupt mask bit"]
pub type MBT_R = crate::BitReader<bool>;
#[doc = "Field `MBT` writer - Multi-block transmission timeout interrupt mask bit"]
pub type MBT_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
#[doc = "Field `CRNT` reader - Command and response Ncr timeout interrupt mask bit"]
pub type CRNT_R = crate::BitReader<bool>;
#[doc = "Field `CRNT` writer - Command and response Ncr timeout interrupt mask bit"]
pub type CRNT_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
#[doc = "Field `CRCE` reader - CRC status error flag interrupt mask bit"]
pub type CRCE_R = crate::BitReader<bool>;
#[doc = "Field `CRCE` writer - CRC status error flag interrupt mask bit"]
pub type CRCE_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
#[doc = "Field `D1` reader - SDIO data1 line interrupt flag/clear bit"]
pub type D1_R = crate::BitReader<bool>;
#[doc = "Field `D1` writer - SDIO data1 line interrupt flag/clear bit"]
pub type D1_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - CMD complete interrupt mask bit"]
    #[inline(always)]
    pub fn cmdd(&self) -> CMDD_R {
        CMDD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DAT complete interrupt mask bit"]
    #[inline(always)]
    pub fn datd(&self) -> DATD_R {
        DATD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DAT CRC error interrupt mask bit"]
    #[inline(always)]
    pub fn date(&self) -> DATE_R {
        DATE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CMD CRC error interrupt mask bit"]
    #[inline(always)]
    pub fn cmde(&self) -> CMDE_R {
        CMDE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Multi-block transfer complete interrupt mask bit"]
    #[inline(always)]
    pub fn mbd(&self) -> MBD_R {
        MBD_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Multi-block transmission timeout interrupt mask bit"]
    #[inline(always)]
    pub fn mbt(&self) -> MBT_R {
        MBT_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Command and response Ncr timeout interrupt mask bit"]
    #[inline(always)]
    pub fn crnt(&self) -> CRNT_R {
        CRNT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - CRC status error flag interrupt mask bit"]
    #[inline(always)]
    pub fn crce(&self) -> CRCE_R {
        CRCE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - SDIO data1 line interrupt flag/clear bit"]
    #[inline(always)]
    pub fn d1(&self) -> D1_R {
        D1_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CMD complete interrupt mask bit"]
    #[inline(always)]
    #[must_use]
    pub fn cmdd(&mut self) -> CMDD_W<0> {
        CMDD_W::new(self)
    }
    #[doc = "Bit 1 - DAT complete interrupt mask bit"]
    #[inline(always)]
    #[must_use]
    pub fn datd(&mut self) -> DATD_W<1> {
        DATD_W::new(self)
    }
    #[doc = "Bit 2 - DAT CRC error interrupt mask bit"]
    #[inline(always)]
    #[must_use]
    pub fn date(&mut self) -> DATE_W<2> {
        DATE_W::new(self)
    }
    #[doc = "Bit 3 - CMD CRC error interrupt mask bit"]
    #[inline(always)]
    #[must_use]
    pub fn cmde(&mut self) -> CMDE_W<3> {
        CMDE_W::new(self)
    }
    #[doc = "Bit 4 - Multi-block transfer complete interrupt mask bit"]
    #[inline(always)]
    #[must_use]
    pub fn mbd(&mut self) -> MBD_W<4> {
        MBD_W::new(self)
    }
    #[doc = "Bit 5 - Multi-block transmission timeout interrupt mask bit"]
    #[inline(always)]
    #[must_use]
    pub fn mbt(&mut self) -> MBT_W<5> {
        MBT_W::new(self)
    }
    #[doc = "Bit 6 - Command and response Ncr timeout interrupt mask bit"]
    #[inline(always)]
    #[must_use]
    pub fn crnt(&mut self) -> CRNT_W<6> {
        CRNT_W::new(self)
    }
    #[doc = "Bit 7 - CRC status error flag interrupt mask bit"]
    #[inline(always)]
    #[must_use]
    pub fn crce(&mut self) -> CRCE_W<7> {
        CRCE_W::new(self)
    }
    #[doc = "Bit 8 - SDIO data1 line interrupt flag/clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn d1(&mut self) -> D1_W<8> {
        D1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt clear mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icr](index.html) module"]
pub struct ICR_SPEC;
impl crate::RegisterSpec for ICR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [icr::R](R) reader structure"]
impl crate::Readable for ICR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [icr::W](W) writer structure"]
impl crate::Writable for ICR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ICR to value 0"]
impl crate::Resettable for ICR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
