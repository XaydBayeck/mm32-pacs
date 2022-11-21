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
#[doc = "Field `IMR0` reader - Interrupt Mask on line 0"]
pub type IMR0_R = crate::BitReader<bool>;
#[doc = "Field `IMR0` writer - Interrupt Mask on line 0"]
pub type IMR0_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR_SPEC, bool, O>;
#[doc = "Field `IMR1` reader - Interrupt Mask on line 1"]
pub type IMR1_R = crate::BitReader<bool>;
#[doc = "Field `IMR1` writer - Interrupt Mask on line 1"]
pub type IMR1_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR_SPEC, bool, O>;
#[doc = "Field `IMR2` reader - Interrupt Mask on line 2"]
pub type IMR2_R = crate::BitReader<bool>;
#[doc = "Field `IMR2` writer - Interrupt Mask on line 2"]
pub type IMR2_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR_SPEC, bool, O>;
#[doc = "Field `IMR3` reader - Interrupt Mask on line 3"]
pub type IMR3_R = crate::BitReader<bool>;
#[doc = "Field `IMR3` writer - Interrupt Mask on line 3"]
pub type IMR3_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR_SPEC, bool, O>;
#[doc = "Field `IMR4` reader - Interrupt Mask on line 4"]
pub type IMR4_R = crate::BitReader<bool>;
#[doc = "Field `IMR4` writer - Interrupt Mask on line 4"]
pub type IMR4_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR_SPEC, bool, O>;
#[doc = "Field `IMR5` reader - Interrupt Mask on line 5"]
pub type IMR5_R = crate::BitReader<bool>;
#[doc = "Field `IMR5` writer - Interrupt Mask on line 5"]
pub type IMR5_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR_SPEC, bool, O>;
#[doc = "Field `IMR6` reader - Interrupt Mask on line 6"]
pub type IMR6_R = crate::BitReader<bool>;
#[doc = "Field `IMR6` writer - Interrupt Mask on line 6"]
pub type IMR6_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR_SPEC, bool, O>;
#[doc = "Field `IMR7` reader - Interrupt Mask on line 7"]
pub type IMR7_R = crate::BitReader<bool>;
#[doc = "Field `IMR7` writer - Interrupt Mask on line 7"]
pub type IMR7_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR_SPEC, bool, O>;
#[doc = "Field `IMR8` reader - Interrupt Mask on line 8"]
pub type IMR8_R = crate::BitReader<bool>;
#[doc = "Field `IMR8` writer - Interrupt Mask on line 8"]
pub type IMR8_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR_SPEC, bool, O>;
#[doc = "Field `IMR9` reader - Interrupt Mask on line 9"]
pub type IMR9_R = crate::BitReader<bool>;
#[doc = "Field `IMR9` writer - Interrupt Mask on line 9"]
pub type IMR9_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR_SPEC, bool, O>;
#[doc = "Field `IMR10` reader - Interrupt Mask on line 10"]
pub type IMR10_R = crate::BitReader<bool>;
#[doc = "Field `IMR10` writer - Interrupt Mask on line 10"]
pub type IMR10_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR_SPEC, bool, O>;
#[doc = "Field `IMR11` reader - Interrupt Mask on line 11"]
pub type IMR11_R = crate::BitReader<bool>;
#[doc = "Field `IMR11` writer - Interrupt Mask on line 11"]
pub type IMR11_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR_SPEC, bool, O>;
#[doc = "Field `IMR12` reader - Interrupt Mask on line 12"]
pub type IMR12_R = crate::BitReader<bool>;
#[doc = "Field `IMR12` writer - Interrupt Mask on line 12"]
pub type IMR12_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR_SPEC, bool, O>;
#[doc = "Field `IMR13` reader - Interrupt Mask on line 13"]
pub type IMR13_R = crate::BitReader<bool>;
#[doc = "Field `IMR13` writer - Interrupt Mask on line 13"]
pub type IMR13_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR_SPEC, bool, O>;
#[doc = "Field `IMR14` reader - Interrupt Mask on line 14"]
pub type IMR14_R = crate::BitReader<bool>;
#[doc = "Field `IMR14` writer - Interrupt Mask on line 14"]
pub type IMR14_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR_SPEC, bool, O>;
#[doc = "Field `IMR15` reader - Interrupt Mask on line 15"]
pub type IMR15_R = crate::BitReader<bool>;
#[doc = "Field `IMR15` writer - Interrupt Mask on line 15"]
pub type IMR15_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR_SPEC, bool, O>;
#[doc = "Field `IMR16` reader - Interrupt Mask on line 16"]
pub type IMR16_R = crate::BitReader<bool>;
#[doc = "Field `IMR16` writer - Interrupt Mask on line 16"]
pub type IMR16_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR_SPEC, bool, O>;
#[doc = "Field `IMR17` reader - Interrupt Mask on line 17"]
pub type IMR17_R = crate::BitReader<bool>;
#[doc = "Field `IMR17` writer - Interrupt Mask on line 17"]
pub type IMR17_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR_SPEC, bool, O>;
#[doc = "Field `IMR18` reader - Interrupt Mask on line 18"]
pub type IMR18_R = crate::BitReader<bool>;
#[doc = "Field `IMR18` writer - Interrupt Mask on line 18"]
pub type IMR18_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR_SPEC, bool, O>;
#[doc = "Field `IMR19` reader - Interrupt Mask on line 19"]
pub type IMR19_R = crate::BitReader<bool>;
#[doc = "Field `IMR19` writer - Interrupt Mask on line 19"]
pub type IMR19_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR_SPEC, bool, O>;
#[doc = "Field `IMR20` reader - Interrupt Mask on line 20"]
pub type IMR20_R = crate::BitReader<bool>;
#[doc = "Field `IMR20` writer - Interrupt Mask on line 20"]
pub type IMR20_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR_SPEC, bool, O>;
#[doc = "Field `IMR24` reader - Interrupt Mask on line 24"]
pub type IMR24_R = crate::BitReader<bool>;
#[doc = "Field `IMR24` writer - Interrupt Mask on line 24"]
pub type IMR24_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Interrupt Mask on line 0"]
    #[inline(always)]
    pub fn imr0(&self) -> IMR0_R {
        IMR0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Interrupt Mask on line 1"]
    #[inline(always)]
    pub fn imr1(&self) -> IMR1_R {
        IMR1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt Mask on line 2"]
    #[inline(always)]
    pub fn imr2(&self) -> IMR2_R {
        IMR2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt Mask on line 3"]
    #[inline(always)]
    pub fn imr3(&self) -> IMR3_R {
        IMR3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Interrupt Mask on line 4"]
    #[inline(always)]
    pub fn imr4(&self) -> IMR4_R {
        IMR4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Interrupt Mask on line 5"]
    #[inline(always)]
    pub fn imr5(&self) -> IMR5_R {
        IMR5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Interrupt Mask on line 6"]
    #[inline(always)]
    pub fn imr6(&self) -> IMR6_R {
        IMR6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Interrupt Mask on line 7"]
    #[inline(always)]
    pub fn imr7(&self) -> IMR7_R {
        IMR7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Interrupt Mask on line 8"]
    #[inline(always)]
    pub fn imr8(&self) -> IMR8_R {
        IMR8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Interrupt Mask on line 9"]
    #[inline(always)]
    pub fn imr9(&self) -> IMR9_R {
        IMR9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Interrupt Mask on line 10"]
    #[inline(always)]
    pub fn imr10(&self) -> IMR10_R {
        IMR10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Interrupt Mask on line 11"]
    #[inline(always)]
    pub fn imr11(&self) -> IMR11_R {
        IMR11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Interrupt Mask on line 12"]
    #[inline(always)]
    pub fn imr12(&self) -> IMR12_R {
        IMR12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Interrupt Mask on line 13"]
    #[inline(always)]
    pub fn imr13(&self) -> IMR13_R {
        IMR13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Interrupt Mask on line 14"]
    #[inline(always)]
    pub fn imr14(&self) -> IMR14_R {
        IMR14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Interrupt Mask on line 15"]
    #[inline(always)]
    pub fn imr15(&self) -> IMR15_R {
        IMR15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Interrupt Mask on line 16"]
    #[inline(always)]
    pub fn imr16(&self) -> IMR16_R {
        IMR16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Interrupt Mask on line 17"]
    #[inline(always)]
    pub fn imr17(&self) -> IMR17_R {
        IMR17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Interrupt Mask on line 18"]
    #[inline(always)]
    pub fn imr18(&self) -> IMR18_R {
        IMR18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Interrupt Mask on line 19"]
    #[inline(always)]
    pub fn imr19(&self) -> IMR19_R {
        IMR19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Interrupt Mask on line 20"]
    #[inline(always)]
    pub fn imr20(&self) -> IMR20_R {
        IMR20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 24 - Interrupt Mask on line 24"]
    #[inline(always)]
    pub fn imr24(&self) -> IMR24_R {
        IMR24_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt Mask on line 0"]
    #[inline(always)]
    #[must_use]
    pub fn imr0(&mut self) -> IMR0_W<0> {
        IMR0_W::new(self)
    }
    #[doc = "Bit 1 - Interrupt Mask on line 1"]
    #[inline(always)]
    #[must_use]
    pub fn imr1(&mut self) -> IMR1_W<1> {
        IMR1_W::new(self)
    }
    #[doc = "Bit 2 - Interrupt Mask on line 2"]
    #[inline(always)]
    #[must_use]
    pub fn imr2(&mut self) -> IMR2_W<2> {
        IMR2_W::new(self)
    }
    #[doc = "Bit 3 - Interrupt Mask on line 3"]
    #[inline(always)]
    #[must_use]
    pub fn imr3(&mut self) -> IMR3_W<3> {
        IMR3_W::new(self)
    }
    #[doc = "Bit 4 - Interrupt Mask on line 4"]
    #[inline(always)]
    #[must_use]
    pub fn imr4(&mut self) -> IMR4_W<4> {
        IMR4_W::new(self)
    }
    #[doc = "Bit 5 - Interrupt Mask on line 5"]
    #[inline(always)]
    #[must_use]
    pub fn imr5(&mut self) -> IMR5_W<5> {
        IMR5_W::new(self)
    }
    #[doc = "Bit 6 - Interrupt Mask on line 6"]
    #[inline(always)]
    #[must_use]
    pub fn imr6(&mut self) -> IMR6_W<6> {
        IMR6_W::new(self)
    }
    #[doc = "Bit 7 - Interrupt Mask on line 7"]
    #[inline(always)]
    #[must_use]
    pub fn imr7(&mut self) -> IMR7_W<7> {
        IMR7_W::new(self)
    }
    #[doc = "Bit 8 - Interrupt Mask on line 8"]
    #[inline(always)]
    #[must_use]
    pub fn imr8(&mut self) -> IMR8_W<8> {
        IMR8_W::new(self)
    }
    #[doc = "Bit 9 - Interrupt Mask on line 9"]
    #[inline(always)]
    #[must_use]
    pub fn imr9(&mut self) -> IMR9_W<9> {
        IMR9_W::new(self)
    }
    #[doc = "Bit 10 - Interrupt Mask on line 10"]
    #[inline(always)]
    #[must_use]
    pub fn imr10(&mut self) -> IMR10_W<10> {
        IMR10_W::new(self)
    }
    #[doc = "Bit 11 - Interrupt Mask on line 11"]
    #[inline(always)]
    #[must_use]
    pub fn imr11(&mut self) -> IMR11_W<11> {
        IMR11_W::new(self)
    }
    #[doc = "Bit 12 - Interrupt Mask on line 12"]
    #[inline(always)]
    #[must_use]
    pub fn imr12(&mut self) -> IMR12_W<12> {
        IMR12_W::new(self)
    }
    #[doc = "Bit 13 - Interrupt Mask on line 13"]
    #[inline(always)]
    #[must_use]
    pub fn imr13(&mut self) -> IMR13_W<13> {
        IMR13_W::new(self)
    }
    #[doc = "Bit 14 - Interrupt Mask on line 14"]
    #[inline(always)]
    #[must_use]
    pub fn imr14(&mut self) -> IMR14_W<14> {
        IMR14_W::new(self)
    }
    #[doc = "Bit 15 - Interrupt Mask on line 15"]
    #[inline(always)]
    #[must_use]
    pub fn imr15(&mut self) -> IMR15_W<15> {
        IMR15_W::new(self)
    }
    #[doc = "Bit 16 - Interrupt Mask on line 16"]
    #[inline(always)]
    #[must_use]
    pub fn imr16(&mut self) -> IMR16_W<16> {
        IMR16_W::new(self)
    }
    #[doc = "Bit 17 - Interrupt Mask on line 17"]
    #[inline(always)]
    #[must_use]
    pub fn imr17(&mut self) -> IMR17_W<17> {
        IMR17_W::new(self)
    }
    #[doc = "Bit 18 - Interrupt Mask on line 18"]
    #[inline(always)]
    #[must_use]
    pub fn imr18(&mut self) -> IMR18_W<18> {
        IMR18_W::new(self)
    }
    #[doc = "Bit 19 - Interrupt Mask on line 19"]
    #[inline(always)]
    #[must_use]
    pub fn imr19(&mut self) -> IMR19_W<19> {
        IMR19_W::new(self)
    }
    #[doc = "Bit 20 - Interrupt Mask on line 20"]
    #[inline(always)]
    #[must_use]
    pub fn imr20(&mut self) -> IMR20_W<20> {
        IMR20_W::new(self)
    }
    #[doc = "Bit 24 - Interrupt Mask on line 24"]
    #[inline(always)]
    #[must_use]
    pub fn imr24(&mut self) -> IMR24_W<24> {
        IMR24_W::new(self)
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
