#[doc = "Register `APB1RSTR` reader"]
pub struct R(crate::R<APB1RSTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APB1RSTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APB1RSTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APB1RSTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APB1RSTR` writer"]
pub struct W(crate::W<APB1RSTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APB1RSTR_SPEC>;
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
impl From<crate::W<APB1RSTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APB1RSTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIM2` reader - TIM2 reset"]
pub type TIM2_R = crate::BitReader<bool>;
#[doc = "Field `TIM2` writer - TIM2 reset"]
pub type TIM2_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1RSTR_SPEC, bool, O>;
#[doc = "Field `TIM3` reader - TIM3 reset"]
pub type TIM3_R = crate::BitReader<bool>;
#[doc = "Field `TIM3` writer - TIM3 reset"]
pub type TIM3_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1RSTR_SPEC, bool, O>;
#[doc = "Field `TIM4` reader - TIM4 reset"]
pub type TIM4_R = crate::BitReader<bool>;
#[doc = "Field `TIM4` writer - TIM4 reset"]
pub type TIM4_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1RSTR_SPEC, bool, O>;
#[doc = "Field `TIM5` reader - TIM5 reset"]
pub type TIM5_R = crate::BitReader<bool>;
#[doc = "Field `TIM5` writer - TIM5 reset"]
pub type TIM5_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1RSTR_SPEC, bool, O>;
#[doc = "Field `TIM6` reader - TIM6 reset"]
pub type TIM6_R = crate::BitReader<bool>;
#[doc = "Field `TIM6` writer - TIM6 reset"]
pub type TIM6_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1RSTR_SPEC, bool, O>;
#[doc = "Field `TIM7` reader - TIM7 reset"]
pub type TIM7_R = crate::BitReader<bool>;
#[doc = "Field `TIM7` writer - TIM7 reset"]
pub type TIM7_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1RSTR_SPEC, bool, O>;
#[doc = "Field `WWDG` reader - Window watchdog reset"]
pub type WWDG_R = crate::BitReader<bool>;
#[doc = "Field `WWDG` writer - Window watchdog reset"]
pub type WWDG_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1RSTR_SPEC, bool, O>;
#[doc = "Field `SPI2` reader - SPI2 reset"]
pub type SPI2_R = crate::BitReader<bool>;
#[doc = "Field `SPI2` writer - SPI2 reset"]
pub type SPI2_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1RSTR_SPEC, bool, O>;
#[doc = "Field `SPI3` reader - SPI3 reset"]
pub type SPI3_R = crate::BitReader<bool>;
#[doc = "Field `SPI3` writer - SPI3 reset"]
pub type SPI3_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1RSTR_SPEC, bool, O>;
#[doc = "Field `UART2` reader - UART2 reset"]
pub type UART2_R = crate::BitReader<bool>;
#[doc = "Field `UART2` writer - UART2 reset"]
pub type UART2_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1RSTR_SPEC, bool, O>;
#[doc = "Field `UART3` reader - UART3 reset"]
pub type UART3_R = crate::BitReader<bool>;
#[doc = "Field `UART3` writer - UART3 reset"]
pub type UART3_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1RSTR_SPEC, bool, O>;
#[doc = "Field `UART4` reader - UART4 reset"]
pub type UART4_R = crate::BitReader<bool>;
#[doc = "Field `UART4` writer - UART4 reset"]
pub type UART4_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1RSTR_SPEC, bool, O>;
#[doc = "Field `UART5` reader - UART5 reset"]
pub type UART5_R = crate::BitReader<bool>;
#[doc = "Field `UART5` writer - UART5 reset"]
pub type UART5_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1RSTR_SPEC, bool, O>;
#[doc = "Field `I2C1` reader - I2C1 reset"]
pub type I2C1_R = crate::BitReader<bool>;
#[doc = "Field `I2C1` writer - I2C1 reset"]
pub type I2C1_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1RSTR_SPEC, bool, O>;
#[doc = "Field `I2C2` reader - I2C2 reset"]
pub type I2C2_R = crate::BitReader<bool>;
#[doc = "Field `I2C2` writer - I2C2 reset"]
pub type I2C2_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1RSTR_SPEC, bool, O>;
#[doc = "Field `CRS` reader - CRS reset"]
pub type CRS_R = crate::BitReader<bool>;
#[doc = "Field `CRS` writer - CRS reset"]
pub type CRS_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1RSTR_SPEC, bool, O>;
#[doc = "Field `CAN` reader - CAN reset"]
pub type CAN_R = crate::BitReader<bool>;
#[doc = "Field `CAN` writer - CAN reset"]
pub type CAN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1RSTR_SPEC, bool, O>;
#[doc = "Field `BKP` reader - Backup interface reset"]
pub type BKP_R = crate::BitReader<bool>;
#[doc = "Field `BKP` writer - Backup interface reset"]
pub type BKP_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1RSTR_SPEC, bool, O>;
#[doc = "Field `PWR` reader - Power interface reset"]
pub type PWR_R = crate::BitReader<bool>;
#[doc = "Field `PWR` writer - Power interface reset"]
pub type PWR_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1RSTR_SPEC, bool, O>;
#[doc = "Field `DAC` reader - DAC reset"]
pub type DAC_R = crate::BitReader<bool>;
#[doc = "Field `DAC` writer - DAC reset"]
pub type DAC_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1RSTR_SPEC, bool, O>;
#[doc = "Field `UART7` reader - UART7 reset"]
pub type UART7_R = crate::BitReader<bool>;
#[doc = "Field `UART7` writer - UART7 reset"]
pub type UART7_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1RSTR_SPEC, bool, O>;
#[doc = "Field `UART8` reader - UART8 reset"]
pub type UART8_R = crate::BitReader<bool>;
#[doc = "Field `UART8` writer - UART8 reset"]
pub type UART8_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1RSTR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - TIM2 reset"]
    #[inline(always)]
    pub fn tim2(&self) -> TIM2_R {
        TIM2_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TIM3 reset"]
    #[inline(always)]
    pub fn tim3(&self) -> TIM3_R {
        TIM3_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TIM4 reset"]
    #[inline(always)]
    pub fn tim4(&self) -> TIM4_R {
        TIM4_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TIM5 reset"]
    #[inline(always)]
    pub fn tim5(&self) -> TIM5_R {
        TIM5_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TIM6 reset"]
    #[inline(always)]
    pub fn tim6(&self) -> TIM6_R {
        TIM6_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TIM7 reset"]
    #[inline(always)]
    pub fn tim7(&self) -> TIM7_R {
        TIM7_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 11 - Window watchdog reset"]
    #[inline(always)]
    pub fn wwdg(&self) -> WWDG_R {
        WWDG_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 14 - SPI2 reset"]
    #[inline(always)]
    pub fn spi2(&self) -> SPI2_R {
        SPI2_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - SPI3 reset"]
    #[inline(always)]
    pub fn spi3(&self) -> SPI3_R {
        SPI3_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17 - UART2 reset"]
    #[inline(always)]
    pub fn uart2(&self) -> UART2_R {
        UART2_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - UART3 reset"]
    #[inline(always)]
    pub fn uart3(&self) -> UART3_R {
        UART3_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - UART4 reset"]
    #[inline(always)]
    pub fn uart4(&self) -> UART4_R {
        UART4_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - UART5 reset"]
    #[inline(always)]
    pub fn uart5(&self) -> UART5_R {
        UART5_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - I2C1 reset"]
    #[inline(always)]
    pub fn i2c1(&self) -> I2C1_R {
        I2C1_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - I2C2 reset"]
    #[inline(always)]
    pub fn i2c2(&self) -> I2C2_R {
        I2C2_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 24 - CRS reset"]
    #[inline(always)]
    pub fn crs(&self) -> CRS_R {
        CRS_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - CAN reset"]
    #[inline(always)]
    pub fn can(&self) -> CAN_R {
        CAN_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 27 - Backup interface reset"]
    #[inline(always)]
    pub fn bkp(&self) -> BKP_R {
        BKP_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Power interface reset"]
    #[inline(always)]
    pub fn pwr(&self) -> PWR_R {
        PWR_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - DAC reset"]
    #[inline(always)]
    pub fn dac(&self) -> DAC_R {
        DAC_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - UART7 reset"]
    #[inline(always)]
    pub fn uart7(&self) -> UART7_R {
        UART7_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - UART8 reset"]
    #[inline(always)]
    pub fn uart8(&self) -> UART8_R {
        UART8_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TIM2 reset"]
    #[inline(always)]
    #[must_use]
    pub fn tim2(&mut self) -> TIM2_W<0> {
        TIM2_W::new(self)
    }
    #[doc = "Bit 1 - TIM3 reset"]
    #[inline(always)]
    #[must_use]
    pub fn tim3(&mut self) -> TIM3_W<1> {
        TIM3_W::new(self)
    }
    #[doc = "Bit 2 - TIM4 reset"]
    #[inline(always)]
    #[must_use]
    pub fn tim4(&mut self) -> TIM4_W<2> {
        TIM4_W::new(self)
    }
    #[doc = "Bit 3 - TIM5 reset"]
    #[inline(always)]
    #[must_use]
    pub fn tim5(&mut self) -> TIM5_W<3> {
        TIM5_W::new(self)
    }
    #[doc = "Bit 4 - TIM6 reset"]
    #[inline(always)]
    #[must_use]
    pub fn tim6(&mut self) -> TIM6_W<4> {
        TIM6_W::new(self)
    }
    #[doc = "Bit 5 - TIM7 reset"]
    #[inline(always)]
    #[must_use]
    pub fn tim7(&mut self) -> TIM7_W<5> {
        TIM7_W::new(self)
    }
    #[doc = "Bit 11 - Window watchdog reset"]
    #[inline(always)]
    #[must_use]
    pub fn wwdg(&mut self) -> WWDG_W<11> {
        WWDG_W::new(self)
    }
    #[doc = "Bit 14 - SPI2 reset"]
    #[inline(always)]
    #[must_use]
    pub fn spi2(&mut self) -> SPI2_W<14> {
        SPI2_W::new(self)
    }
    #[doc = "Bit 15 - SPI3 reset"]
    #[inline(always)]
    #[must_use]
    pub fn spi3(&mut self) -> SPI3_W<15> {
        SPI3_W::new(self)
    }
    #[doc = "Bit 17 - UART2 reset"]
    #[inline(always)]
    #[must_use]
    pub fn uart2(&mut self) -> UART2_W<17> {
        UART2_W::new(self)
    }
    #[doc = "Bit 18 - UART3 reset"]
    #[inline(always)]
    #[must_use]
    pub fn uart3(&mut self) -> UART3_W<18> {
        UART3_W::new(self)
    }
    #[doc = "Bit 19 - UART4 reset"]
    #[inline(always)]
    #[must_use]
    pub fn uart4(&mut self) -> UART4_W<19> {
        UART4_W::new(self)
    }
    #[doc = "Bit 20 - UART5 reset"]
    #[inline(always)]
    #[must_use]
    pub fn uart5(&mut self) -> UART5_W<20> {
        UART5_W::new(self)
    }
    #[doc = "Bit 21 - I2C1 reset"]
    #[inline(always)]
    #[must_use]
    pub fn i2c1(&mut self) -> I2C1_W<21> {
        I2C1_W::new(self)
    }
    #[doc = "Bit 22 - I2C2 reset"]
    #[inline(always)]
    #[must_use]
    pub fn i2c2(&mut self) -> I2C2_W<22> {
        I2C2_W::new(self)
    }
    #[doc = "Bit 24 - CRS reset"]
    #[inline(always)]
    #[must_use]
    pub fn crs(&mut self) -> CRS_W<24> {
        CRS_W::new(self)
    }
    #[doc = "Bit 25 - CAN reset"]
    #[inline(always)]
    #[must_use]
    pub fn can(&mut self) -> CAN_W<25> {
        CAN_W::new(self)
    }
    #[doc = "Bit 27 - Backup interface reset"]
    #[inline(always)]
    #[must_use]
    pub fn bkp(&mut self) -> BKP_W<27> {
        BKP_W::new(self)
    }
    #[doc = "Bit 28 - Power interface reset"]
    #[inline(always)]
    #[must_use]
    pub fn pwr(&mut self) -> PWR_W<28> {
        PWR_W::new(self)
    }
    #[doc = "Bit 29 - DAC reset"]
    #[inline(always)]
    #[must_use]
    pub fn dac(&mut self) -> DAC_W<29> {
        DAC_W::new(self)
    }
    #[doc = "Bit 30 - UART7 reset"]
    #[inline(always)]
    #[must_use]
    pub fn uart7(&mut self) -> UART7_W<30> {
        UART7_W::new(self)
    }
    #[doc = "Bit 31 - UART8 reset"]
    #[inline(always)]
    #[must_use]
    pub fn uart8(&mut self) -> UART8_W<31> {
        UART8_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Advanced Peripheral Bus 1 Reset Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb1rstr](index.html) module"]
pub struct APB1RSTR_SPEC;
impl crate::RegisterSpec for APB1RSTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apb1rstr::R](R) reader structure"]
impl crate::Readable for APB1RSTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [apb1rstr::W](W) writer structure"]
impl crate::Writable for APB1RSTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets APB1RSTR to value 0"]
impl crate::Resettable for APB1RSTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
