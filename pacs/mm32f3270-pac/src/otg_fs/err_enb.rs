#[doc = "Register `ERR_ENB` reader"]
pub struct R(crate::R<ERR_ENB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ERR_ENB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ERR_ENB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ERR_ENB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ERR_ENB` writer"]
pub struct W(crate::W<ERR_ENB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ERR_ENB_SPEC>;
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
impl From<crate::W<ERR_ENB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ERR_ENB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PID_ERR_EN` reader - Setting this bit enables PID_ Err interrupt"]
pub type PID_ERR_EN_R = crate::BitReader<bool>;
#[doc = "Field `PID_ERR_EN` writer - Setting this bit enables PID_ Err interrupt"]
pub type PID_ERR_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ERR_ENB_SPEC, bool, O>;
#[doc = "Field `CRC5_EOF_EN` reader - Setting this bit enables CRC5orEOF interrupt."]
pub type CRC5_EOF_EN_R = crate::BitReader<bool>;
#[doc = "Field `CRC5_EOF_EN` writer - Setting this bit enables CRC5orEOF interrupt."]
pub type CRC5_EOF_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ERR_ENB_SPEC, bool, O>;
#[doc = "Field `CRC16_EN` reader - Setting this bit enables CRC16 interrupt."]
pub type CRC16_EN_R = crate::BitReader<bool>;
#[doc = "Field `CRC16_EN` writer - Setting this bit enables CRC16 interrupt."]
pub type CRC16_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ERR_ENB_SPEC, bool, O>;
#[doc = "Field `DFN8_EN` reader - Setting this bit enables DFN8 interrupt."]
pub type DFN8_EN_R = crate::BitReader<bool>;
#[doc = "Field `DFN8_EN` writer - Setting this bit enables DFN8 interrupt."]
pub type DFN8_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ERR_ENB_SPEC, bool, O>;
#[doc = "Field `BTO_ERR_EN` reader - Setting this bit enables BTO_ERR interrupt."]
pub type BTO_ERR_EN_R = crate::BitReader<bool>;
#[doc = "Field `BTO_ERR_EN` writer - Setting this bit enables BTO_ERR interrupt."]
pub type BTO_ERR_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ERR_ENB_SPEC, bool, O>;
#[doc = "Field `DMA_ERR_EN` reader - Setting this bit enables DMA_ERR interrupt."]
pub type DMA_ERR_EN_R = crate::BitReader<bool>;
#[doc = "Field `DMA_ERR_EN` writer - Setting this bit enables DMA_ERR interrupt."]
pub type DMA_ERR_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ERR_ENB_SPEC, bool, O>;
#[doc = "Field `BTS_ERR_EN` reader - Setting this bit enables BTS_ERR interrupt."]
pub type BTS_ERR_EN_R = crate::BitReader<bool>;
#[doc = "Field `BTS_ERR_EN` writer - Setting this bit enables BTS_ERR interrupt."]
pub type BTS_ERR_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ERR_ENB_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Setting this bit enables PID_ Err interrupt"]
    #[inline(always)]
    pub fn pid_err_en(&self) -> PID_ERR_EN_R {
        PID_ERR_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Setting this bit enables CRC5orEOF interrupt."]
    #[inline(always)]
    pub fn crc5_eof_en(&self) -> CRC5_EOF_EN_R {
        CRC5_EOF_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Setting this bit enables CRC16 interrupt."]
    #[inline(always)]
    pub fn crc16_en(&self) -> CRC16_EN_R {
        CRC16_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Setting this bit enables DFN8 interrupt."]
    #[inline(always)]
    pub fn dfn8_en(&self) -> DFN8_EN_R {
        DFN8_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Setting this bit enables BTO_ERR interrupt."]
    #[inline(always)]
    pub fn bto_err_en(&self) -> BTO_ERR_EN_R {
        BTO_ERR_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Setting this bit enables DMA_ERR interrupt."]
    #[inline(always)]
    pub fn dma_err_en(&self) -> DMA_ERR_EN_R {
        DMA_ERR_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - Setting this bit enables BTS_ERR interrupt."]
    #[inline(always)]
    pub fn bts_err_en(&self) -> BTS_ERR_EN_R {
        BTS_ERR_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Setting this bit enables PID_ Err interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn pid_err_en(&mut self) -> PID_ERR_EN_W<0> {
        PID_ERR_EN_W::new(self)
    }
    #[doc = "Bit 1 - Setting this bit enables CRC5orEOF interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn crc5_eof_en(&mut self) -> CRC5_EOF_EN_W<1> {
        CRC5_EOF_EN_W::new(self)
    }
    #[doc = "Bit 2 - Setting this bit enables CRC16 interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn crc16_en(&mut self) -> CRC16_EN_W<2> {
        CRC16_EN_W::new(self)
    }
    #[doc = "Bit 3 - Setting this bit enables DFN8 interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn dfn8_en(&mut self) -> DFN8_EN_W<3> {
        DFN8_EN_W::new(self)
    }
    #[doc = "Bit 4 - Setting this bit enables BTO_ERR interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn bto_err_en(&mut self) -> BTO_ERR_EN_W<4> {
        BTO_ERR_EN_W::new(self)
    }
    #[doc = "Bit 5 - Setting this bit enables DMA_ERR interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn dma_err_en(&mut self) -> DMA_ERR_EN_W<5> {
        DMA_ERR_EN_W::new(self)
    }
    #[doc = "Bit 7 - Setting this bit enables BTS_ERR interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn bts_err_en(&mut self) -> BTS_ERR_EN_W<7> {
        BTS_ERR_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Error interrupt enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [err_enb](index.html) module"]
pub struct ERR_ENB_SPEC;
impl crate::RegisterSpec for ERR_ENB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [err_enb::R](R) reader structure"]
impl crate::Readable for ERR_ENB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [err_enb::W](W) writer structure"]
impl crate::Writable for ERR_ENB_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ERR_ENB to value 0"]
impl crate::Resettable for ERR_ENB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
