#[doc = "Register `CR1` reader"]
pub struct R(crate::R<CR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR1` writer"]
pub struct W(crate::W<CR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR1_SPEC>;
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
impl From<crate::W<CR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OPMSEL` reader - SD/MMC/SDIO port operation mode selection"]
pub type OPMSEL_R = crate::BitReader<bool>;
#[doc = "Field `OPMSEL` writer - SD/MMC/SDIO port operation mode selection"]
pub type OPMSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
#[doc = "Field `SMSEL` reader - Signal mode selection"]
pub type SMSEL_R = crate::BitReader<bool>;
#[doc = "Field `SMSEL` writer - Signal mode selection"]
pub type SMSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
#[doc = "Field `OUTM` reader - SD/MMC/SDIO port CMD line output drive mode selection"]
pub type OUTM_R = crate::BitReader<bool>;
#[doc = "Field `OUTM` writer - SD/MMC/SDIO port CMD line output drive mode selection"]
pub type OUTM_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
#[doc = "Field `CLKSEL` reader - SD/MMC/SDIO port CLK line speed selection"]
pub type CLKSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CLKSEL` writer - SD/MMC/SDIO port CLK line speed selection"]
pub type CLKSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR1_SPEC, u8, u8, 3, O>;
#[doc = "Field `PTSMSEL` reader - SD/MMC/SDIO port transfer speed mode selection"]
pub type PTSMSEL_R = crate::BitReader<bool>;
#[doc = "Field `PTSMSEL` writer - SD/MMC/SDIO port transfer speed mode selection"]
pub type PTSMSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
#[doc = "Field `DATWT` reader - Define the bus width of SD/MMC/SDIO port DAT line"]
pub type DATWT_R = crate::BitReader<bool>;
#[doc = "Field `DATWT` writer - Define the bus width of SD/MMC/SDIO port DAT line"]
pub type DATWT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
#[doc = "Field `MODE` reader - SDIO mode enable"]
pub type MODE_R = crate::BitReader<bool>;
#[doc = "Field `MODE` writer - SDIO mode enable"]
pub type MODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
#[doc = "Field `INTEN` reader - SDIO interrupt enable signal"]
pub type INTEN_R = crate::BitReader<bool>;
#[doc = "Field `INTEN` writer - SDIO interrupt enable signal"]
pub type INTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
#[doc = "Field `RDWTEN` reader - SDIO read wait enable signal"]
pub type RDWTEN_R = crate::BitReader<bool>;
#[doc = "Field `RDWTEN` writer - SDIO read wait enable signal"]
pub type RDWTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - SD/MMC/SDIO port operation mode selection"]
    #[inline(always)]
    pub fn opmsel(&self) -> OPMSEL_R {
        OPMSEL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Signal mode selection"]
    #[inline(always)]
    pub fn smsel(&self) -> SMSEL_R {
        SMSEL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SD/MMC/SDIO port CMD line output drive mode selection"]
    #[inline(always)]
    pub fn outm(&self) -> OUTM_R {
        OUTM_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:5 - SD/MMC/SDIO port CLK line speed selection"]
    #[inline(always)]
    pub fn clksel(&self) -> CLKSEL_R {
        CLKSEL_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bit 6 - SD/MMC/SDIO port transfer speed mode selection"]
    #[inline(always)]
    pub fn ptsmsel(&self) -> PTSMSEL_R {
        PTSMSEL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Define the bus width of SD/MMC/SDIO port DAT line"]
    #[inline(always)]
    pub fn datwt(&self) -> DATWT_R {
        DATWT_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - SDIO mode enable"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SDIO interrupt enable signal"]
    #[inline(always)]
    pub fn inten(&self) -> INTEN_R {
        INTEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - SDIO read wait enable signal"]
    #[inline(always)]
    pub fn rdwten(&self) -> RDWTEN_R {
        RDWTEN_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SD/MMC/SDIO port operation mode selection"]
    #[inline(always)]
    #[must_use]
    pub fn opmsel(&mut self) -> OPMSEL_W<0> {
        OPMSEL_W::new(self)
    }
    #[doc = "Bit 1 - Signal mode selection"]
    #[inline(always)]
    #[must_use]
    pub fn smsel(&mut self) -> SMSEL_W<1> {
        SMSEL_W::new(self)
    }
    #[doc = "Bit 2 - SD/MMC/SDIO port CMD line output drive mode selection"]
    #[inline(always)]
    #[must_use]
    pub fn outm(&mut self) -> OUTM_W<2> {
        OUTM_W::new(self)
    }
    #[doc = "Bits 3:5 - SD/MMC/SDIO port CLK line speed selection"]
    #[inline(always)]
    #[must_use]
    pub fn clksel(&mut self) -> CLKSEL_W<3> {
        CLKSEL_W::new(self)
    }
    #[doc = "Bit 6 - SD/MMC/SDIO port transfer speed mode selection"]
    #[inline(always)]
    #[must_use]
    pub fn ptsmsel(&mut self) -> PTSMSEL_W<6> {
        PTSMSEL_W::new(self)
    }
    #[doc = "Bit 7 - Define the bus width of SD/MMC/SDIO port DAT line"]
    #[inline(always)]
    #[must_use]
    pub fn datwt(&mut self) -> DATWT_W<7> {
        DATWT_W::new(self)
    }
    #[doc = "Bit 8 - SDIO mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<8> {
        MODE_W::new(self)
    }
    #[doc = "Bit 9 - SDIO interrupt enable signal"]
    #[inline(always)]
    #[must_use]
    pub fn inten(&mut self) -> INTEN_W<9> {
        INTEN_W::new(self)
    }
    #[doc = "Bit 10 - SDIO read wait enable signal"]
    #[inline(always)]
    #[must_use]
    pub fn rdwten(&mut self) -> RDWTEN_W<10> {
        RDWTEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CR1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr1](index.html) module"]
pub struct CR1_SPEC;
impl crate::RegisterSpec for CR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr1::R](R) reader structure"]
impl crate::Readable for CR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr1::W](W) writer structure"]
impl crate::Writable for CR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CR1 to value 0x45"]
impl crate::Resettable for CR1_SPEC {
    const RESET_VALUE: Self::Ux = 0x45;
}
