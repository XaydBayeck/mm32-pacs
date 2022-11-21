#[doc = "Register `APB2ENR` reader"]
pub struct R(crate::R<APB2ENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APB2ENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APB2ENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APB2ENR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APB2ENR` writer"]
pub struct W(crate::W<APB2ENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APB2ENR_SPEC>;
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
impl From<crate::W<APB2ENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APB2ENR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIM1` reader - TIM1 timer clock enable"]
pub type TIM1_R = crate::BitReader<bool>;
#[doc = "Field `TIM1` writer - TIM1 timer clock enable"]
pub type TIM1_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2ENR_SPEC, bool, O>;
#[doc = "Field `TIM8` reader - TIM8 timer clock enable"]
pub type TIM8_R = crate::BitReader<bool>;
#[doc = "Field `TIM8` writer - TIM8 timer clock enable"]
pub type TIM8_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2ENR_SPEC, bool, O>;
#[doc = "Field `UART1` reader - UART1 clock enable"]
pub type UART1_R = crate::BitReader<bool>;
#[doc = "Field `UART1` writer - UART1 clock enable"]
pub type UART1_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2ENR_SPEC, bool, O>;
#[doc = "Field `UART6` reader - UART6 clock enable"]
pub type UART6_R = crate::BitReader<bool>;
#[doc = "Field `UART6` writer - UART6 clock enable"]
pub type UART6_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2ENR_SPEC, bool, O>;
#[doc = "Field `ADC1` reader - ADC1 clock enable"]
pub type ADC1_R = crate::BitReader<bool>;
#[doc = "Field `ADC1` writer - ADC1 clock enable"]
pub type ADC1_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2ENR_SPEC, bool, O>;
#[doc = "Field `ADC2` reader - ADC2 clock enable"]
pub type ADC2_R = crate::BitReader<bool>;
#[doc = "Field `ADC2` writer - ADC2 clock enable"]
pub type ADC2_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2ENR_SPEC, bool, O>;
#[doc = "Field `ADC3` reader - ADC3 clock enable"]
pub type ADC3_R = crate::BitReader<bool>;
#[doc = "Field `ADC3` writer - ADC3 clock enable"]
pub type ADC3_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2ENR_SPEC, bool, O>;
#[doc = "Field `SPI1` reader - SPI1 clock enable"]
pub type SPI1_R = crate::BitReader<bool>;
#[doc = "Field `SPI1` writer - SPI1 clock enable"]
pub type SPI1_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2ENR_SPEC, bool, O>;
#[doc = "Field `SYSCFG` reader - SYSCFG clock enable"]
pub type SYSCFG_R = crate::BitReader<bool>;
#[doc = "Field `SYSCFG` writer - SYSCFG clock enable"]
pub type SYSCFG_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2ENR_SPEC, bool, O>;
#[doc = "Field `COMP` reader - COMP clock enable"]
pub type COMP_R = crate::BitReader<bool>;
#[doc = "Field `COMP` writer - COMP clock enable"]
pub type COMP_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2ENR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - TIM1 timer clock enable"]
    #[inline(always)]
    pub fn tim1(&self) -> TIM1_R {
        TIM1_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TIM8 timer clock enable"]
    #[inline(always)]
    pub fn tim8(&self) -> TIM8_R {
        TIM8_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - UART1 clock enable"]
    #[inline(always)]
    pub fn uart1(&self) -> UART1_R {
        UART1_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - UART6 clock enable"]
    #[inline(always)]
    pub fn uart6(&self) -> UART6_R {
        UART6_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - ADC1 clock enable"]
    #[inline(always)]
    pub fn adc1(&self) -> ADC1_R {
        ADC1_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - ADC2 clock enable"]
    #[inline(always)]
    pub fn adc2(&self) -> ADC2_R {
        ADC2_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - ADC3 clock enable"]
    #[inline(always)]
    pub fn adc3(&self) -> ADC3_R {
        ADC3_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - SPI1 clock enable"]
    #[inline(always)]
    pub fn spi1(&self) -> SPI1_R {
        SPI1_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - SYSCFG clock enable"]
    #[inline(always)]
    pub fn syscfg(&self) -> SYSCFG_R {
        SYSCFG_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - COMP clock enable"]
    #[inline(always)]
    pub fn comp(&self) -> COMP_R {
        COMP_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TIM1 timer clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tim1(&mut self) -> TIM1_W<0> {
        TIM1_W::new(self)
    }
    #[doc = "Bit 1 - TIM8 timer clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tim8(&mut self) -> TIM8_W<1> {
        TIM8_W::new(self)
    }
    #[doc = "Bit 4 - UART1 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn uart1(&mut self) -> UART1_W<4> {
        UART1_W::new(self)
    }
    #[doc = "Bit 5 - UART6 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn uart6(&mut self) -> UART6_W<5> {
        UART6_W::new(self)
    }
    #[doc = "Bit 8 - ADC1 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn adc1(&mut self) -> ADC1_W<8> {
        ADC1_W::new(self)
    }
    #[doc = "Bit 9 - ADC2 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn adc2(&mut self) -> ADC2_W<9> {
        ADC2_W::new(self)
    }
    #[doc = "Bit 10 - ADC3 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn adc3(&mut self) -> ADC3_W<10> {
        ADC3_W::new(self)
    }
    #[doc = "Bit 12 - SPI1 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn spi1(&mut self) -> SPI1_W<12> {
        SPI1_W::new(self)
    }
    #[doc = "Bit 14 - SYSCFG clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn syscfg(&mut self) -> SYSCFG_W<14> {
        SYSCFG_W::new(self)
    }
    #[doc = "Bit 15 - COMP clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn comp(&mut self) -> COMP_W<15> {
        COMP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Advanced Peripheral Bus 2 Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb2enr](index.html) module"]
pub struct APB2ENR_SPEC;
impl crate::RegisterSpec for APB2ENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apb2enr::R](R) reader structure"]
impl crate::Readable for APB2ENR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [apb2enr::W](W) writer structure"]
impl crate::Writable for APB2ENR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets APB2ENR to value 0"]
impl crate::Resettable for APB2ENR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
