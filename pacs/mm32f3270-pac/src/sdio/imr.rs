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
#[doc = "Field `CMDD` reader - CMD done interrupt mask"]
pub type CMDD_R = crate::BitReader<bool>;
#[doc = "Field `CMDD` writer - CMD done interrupt mask"]
pub type CMDD_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR_SPEC, bool, O>;
#[doc = "Field `DATD` reader - CMD done interrupt mask"]
pub type DATD_R = crate::BitReader<bool>;
#[doc = "Field `DATD` writer - CMD done interrupt mask"]
pub type DATD_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR_SPEC, bool, O>;
#[doc = "Field `DATE` reader - DAT CRC error interrupt mask"]
pub type DATE_R = crate::BitReader<bool>;
#[doc = "Field `DATE` writer - DAT CRC error interrupt mask"]
pub type DATE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR_SPEC, bool, O>;
#[doc = "Field `CMDE` reader - CMD CRC error interrupt mask"]
pub type CMDE_R = crate::BitReader<bool>;
#[doc = "Field `CMDE` writer - CMD CRC error interrupt mask"]
pub type CMDE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR_SPEC, bool, O>;
#[doc = "Field `MBD` reader - Multi Block done interrupt mask"]
pub type MBD_R = crate::BitReader<bool>;
#[doc = "Field `MBD` writer - Multi Block done interrupt mask"]
pub type MBD_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR_SPEC, bool, O>;
#[doc = "Field `MBT` reader - Multi Block Timeout interrupt mask"]
pub type MBT_R = crate::BitReader<bool>;
#[doc = "Field `MBT` writer - Multi Block Timeout interrupt mask"]
pub type MBT_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR_SPEC, bool, O>;
#[doc = "Field `CRNT` reader - Cmd and Resp Ncr Timeout interrupt mask"]
pub type CRNT_R = crate::BitReader<bool>;
#[doc = "Field `CRNT` writer - Cmd and Resp Ncr Timeout interrupt mask"]
pub type CRNT_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR_SPEC, bool, O>;
#[doc = "Field `CRCE` reader - CRC status token err interrupt mask"]
pub type CRCE_R = crate::BitReader<bool>;
#[doc = "Field `CRCE` writer - CRC status token err interrupt mask"]
pub type CRCE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR_SPEC, bool, O>;
#[doc = "Field `D1L` reader - SDIO data1 line interrupt mask"]
pub type D1L_R = crate::BitReader<bool>;
#[doc = "Field `D1L` writer - SDIO data1 line interrupt mask"]
pub type D1L_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - CMD done interrupt mask"]
    #[inline(always)]
    pub fn cmdd(&self) -> CMDD_R {
        CMDD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CMD done interrupt mask"]
    #[inline(always)]
    pub fn datd(&self) -> DATD_R {
        DATD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DAT CRC error interrupt mask"]
    #[inline(always)]
    pub fn date(&self) -> DATE_R {
        DATE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CMD CRC error interrupt mask"]
    #[inline(always)]
    pub fn cmde(&self) -> CMDE_R {
        CMDE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Multi Block done interrupt mask"]
    #[inline(always)]
    pub fn mbd(&self) -> MBD_R {
        MBD_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Multi Block Timeout interrupt mask"]
    #[inline(always)]
    pub fn mbt(&self) -> MBT_R {
        MBT_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Cmd and Resp Ncr Timeout interrupt mask"]
    #[inline(always)]
    pub fn crnt(&self) -> CRNT_R {
        CRNT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - CRC status token err interrupt mask"]
    #[inline(always)]
    pub fn crce(&self) -> CRCE_R {
        CRCE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - SDIO data1 line interrupt mask"]
    #[inline(always)]
    pub fn d1l(&self) -> D1L_R {
        D1L_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CMD done interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn cmdd(&mut self) -> CMDD_W<0> {
        CMDD_W::new(self)
    }
    #[doc = "Bit 1 - CMD done interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn datd(&mut self) -> DATD_W<1> {
        DATD_W::new(self)
    }
    #[doc = "Bit 2 - DAT CRC error interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn date(&mut self) -> DATE_W<2> {
        DATE_W::new(self)
    }
    #[doc = "Bit 3 - CMD CRC error interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn cmde(&mut self) -> CMDE_W<3> {
        CMDE_W::new(self)
    }
    #[doc = "Bit 4 - Multi Block done interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn mbd(&mut self) -> MBD_W<4> {
        MBD_W::new(self)
    }
    #[doc = "Bit 5 - Multi Block Timeout interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn mbt(&mut self) -> MBT_W<5> {
        MBT_W::new(self)
    }
    #[doc = "Bit 6 - Cmd and Resp Ncr Timeout interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn crnt(&mut self) -> CRNT_W<6> {
        CRNT_W::new(self)
    }
    #[doc = "Bit 7 - CRC status token err interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn crce(&mut self) -> CRCE_W<7> {
        CRCE_W::new(self)
    }
    #[doc = "Bit 8 - SDIO data1 line interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn d1l(&mut self) -> D1L_W<8> {
        D1L_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [imr](index.html) module"]
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
#[doc = "`reset()` method sets IMR to value 0"]
impl crate::Resettable for IMR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
