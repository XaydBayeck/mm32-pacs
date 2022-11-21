#[doc = "Register `AHB1RSTR` reader"]
pub struct R(crate::R<AHB1RSTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AHB1RSTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AHB1RSTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AHB1RSTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AHB1RSTR` writer"]
pub struct W(crate::W<AHB1RSTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AHB1RSTR_SPEC>;
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
impl From<crate::W<AHB1RSTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AHB1RSTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GPIOA` reader - FSMC reset"]
pub type GPIOA_R = crate::BitReader<bool>;
#[doc = "Field `GPIOA` writer - FSMC reset"]
pub type GPIOA_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB1RSTR_SPEC, bool, O>;
#[doc = "Field `GPIOB` reader - GPIOb reset"]
pub type GPIOB_R = crate::BitReader<bool>;
#[doc = "Field `GPIOB` writer - GPIOb reset"]
pub type GPIOB_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB1RSTR_SPEC, bool, O>;
#[doc = "Field `GPIOC` reader - GPIOC reset"]
pub type GPIOC_R = crate::BitReader<bool>;
#[doc = "Field `GPIOC` writer - GPIOC reset"]
pub type GPIOC_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB1RSTR_SPEC, bool, O>;
#[doc = "Field `GPIOD` reader - GPIOD reset"]
pub type GPIOD_R = crate::BitReader<bool>;
#[doc = "Field `GPIOD` writer - GPIOD reset"]
pub type GPIOD_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB1RSTR_SPEC, bool, O>;
#[doc = "Field `GPIOE` reader - GPIOE reset"]
pub type GPIOE_R = crate::BitReader<bool>;
#[doc = "Field `GPIOE` writer - GPIOE reset"]
pub type GPIOE_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB1RSTR_SPEC, bool, O>;
#[doc = "Field `GPIOF` reader - GPIOF reset"]
pub type GPIOF_R = crate::BitReader<bool>;
#[doc = "Field `GPIOF` writer - GPIOF reset"]
pub type GPIOF_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB1RSTR_SPEC, bool, O>;
#[doc = "Field `GPIOG` reader - GPIOG reset"]
pub type GPIOG_R = crate::BitReader<bool>;
#[doc = "Field `GPIOG` writer - GPIOG reset"]
pub type GPIOG_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB1RSTR_SPEC, bool, O>;
#[doc = "Field `GPIOH` reader - USBOTGFS reset"]
pub type GPIOH_R = crate::BitReader<bool>;
#[doc = "Field `GPIOH` writer - USBOTGFS reset"]
pub type GPIOH_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB1RSTR_SPEC, bool, O>;
#[doc = "Field `SDIO` reader - SDIO reset"]
pub type SDIO_R = crate::BitReader<bool>;
#[doc = "Field `SDIO` writer - SDIO reset"]
pub type SDIO_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB1RSTR_SPEC, bool, O>;
#[doc = "Field `CRC` reader - *12"]
pub type CRC_R = crate::BitReader<bool>;
#[doc = "Field `CRC` writer - *12"]
pub type CRC_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB1RSTR_SPEC, bool, O>;
#[doc = "Field `DMA1` reader - DMA1 reset"]
pub type DMA1_R = crate::BitReader<bool>;
#[doc = "Field `DMA1` writer - DMA1 reset"]
pub type DMA1_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB1RSTR_SPEC, bool, O>;
#[doc = "Field `DMA2` reader - DMA2 reset"]
pub type DMA2_R = crate::BitReader<bool>;
#[doc = "Field `DMA2` writer - DMA2 reset"]
pub type DMA2_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB1RSTR_SPEC, bool, O>;
#[doc = "Field `ETHMAC` reader - ETHMAC reset"]
pub type ETHMAC_R = crate::BitReader<bool>;
#[doc = "Field `ETHMAC` writer - ETHMAC reset"]
pub type ETHMAC_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB1RSTR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - FSMC reset"]
    #[inline(always)]
    pub fn gpioa(&self) -> GPIOA_R {
        GPIOA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - GPIOb reset"]
    #[inline(always)]
    pub fn gpiob(&self) -> GPIOB_R {
        GPIOB_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - GPIOC reset"]
    #[inline(always)]
    pub fn gpioc(&self) -> GPIOC_R {
        GPIOC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - GPIOD reset"]
    #[inline(always)]
    pub fn gpiod(&self) -> GPIOD_R {
        GPIOD_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - GPIOE reset"]
    #[inline(always)]
    pub fn gpioe(&self) -> GPIOE_R {
        GPIOE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - GPIOF reset"]
    #[inline(always)]
    pub fn gpiof(&self) -> GPIOF_R {
        GPIOF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - GPIOG reset"]
    #[inline(always)]
    pub fn gpiog(&self) -> GPIOG_R {
        GPIOG_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - USBOTGFS reset"]
    #[inline(always)]
    pub fn gpioh(&self) -> GPIOH_R {
        GPIOH_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 10 - SDIO reset"]
    #[inline(always)]
    pub fn sdio(&self) -> SDIO_R {
        SDIO_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - *12"]
    #[inline(always)]
    pub fn crc(&self) -> CRC_R {
        CRC_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 21 - DMA1 reset"]
    #[inline(always)]
    pub fn dma1(&self) -> DMA1_R {
        DMA1_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - DMA2 reset"]
    #[inline(always)]
    pub fn dma2(&self) -> DMA2_R {
        DMA2_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 25 - ETHMAC reset"]
    #[inline(always)]
    pub fn ethmac(&self) -> ETHMAC_R {
        ETHMAC_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - FSMC reset"]
    #[inline(always)]
    #[must_use]
    pub fn gpioa(&mut self) -> GPIOA_W<0> {
        GPIOA_W::new(self)
    }
    #[doc = "Bit 1 - GPIOb reset"]
    #[inline(always)]
    #[must_use]
    pub fn gpiob(&mut self) -> GPIOB_W<1> {
        GPIOB_W::new(self)
    }
    #[doc = "Bit 2 - GPIOC reset"]
    #[inline(always)]
    #[must_use]
    pub fn gpioc(&mut self) -> GPIOC_W<2> {
        GPIOC_W::new(self)
    }
    #[doc = "Bit 3 - GPIOD reset"]
    #[inline(always)]
    #[must_use]
    pub fn gpiod(&mut self) -> GPIOD_W<3> {
        GPIOD_W::new(self)
    }
    #[doc = "Bit 4 - GPIOE reset"]
    #[inline(always)]
    #[must_use]
    pub fn gpioe(&mut self) -> GPIOE_W<4> {
        GPIOE_W::new(self)
    }
    #[doc = "Bit 5 - GPIOF reset"]
    #[inline(always)]
    #[must_use]
    pub fn gpiof(&mut self) -> GPIOF_W<5> {
        GPIOF_W::new(self)
    }
    #[doc = "Bit 6 - GPIOG reset"]
    #[inline(always)]
    #[must_use]
    pub fn gpiog(&mut self) -> GPIOG_W<6> {
        GPIOG_W::new(self)
    }
    #[doc = "Bit 7 - USBOTGFS reset"]
    #[inline(always)]
    #[must_use]
    pub fn gpioh(&mut self) -> GPIOH_W<7> {
        GPIOH_W::new(self)
    }
    #[doc = "Bit 10 - SDIO reset"]
    #[inline(always)]
    #[must_use]
    pub fn sdio(&mut self) -> SDIO_W<10> {
        SDIO_W::new(self)
    }
    #[doc = "Bit 12 - *12"]
    #[inline(always)]
    #[must_use]
    pub fn crc(&mut self) -> CRC_W<12> {
        CRC_W::new(self)
    }
    #[doc = "Bit 21 - DMA1 reset"]
    #[inline(always)]
    #[must_use]
    pub fn dma1(&mut self) -> DMA1_W<21> {
        DMA1_W::new(self)
    }
    #[doc = "Bit 22 - DMA2 reset"]
    #[inline(always)]
    #[must_use]
    pub fn dma2(&mut self) -> DMA2_W<22> {
        DMA2_W::new(self)
    }
    #[doc = "Bit 25 - ETHMAC reset"]
    #[inline(always)]
    #[must_use]
    pub fn ethmac(&mut self) -> ETHMAC_W<25> {
        ETHMAC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Advanced High Performance Bus 1 Reset Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahb1rstr](index.html) module"]
pub struct AHB1RSTR_SPEC;
impl crate::RegisterSpec for AHB1RSTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ahb1rstr::R](R) reader structure"]
impl crate::Readable for AHB1RSTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ahb1rstr::W](W) writer structure"]
impl crate::Writable for AHB1RSTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AHB1RSTR to value 0"]
impl crate::Resettable for AHB1RSTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
