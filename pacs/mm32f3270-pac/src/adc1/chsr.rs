#[doc = "Register `CHSR` reader"]
pub struct R(crate::R<CHSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CHSR` writer"]
pub struct W(crate::W<CHSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHSR_SPEC>;
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
impl From<crate::W<CHSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CHSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CHEN0` reader - Analog input channel 0 enable"]
pub type CHEN0_R = crate::BitReader<bool>;
#[doc = "Field `CHEN0` writer - Analog input channel 0 enable"]
pub type CHEN0_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHSR_SPEC, bool, O>;
#[doc = "Field `CHEN1` reader - Analog input channel 1 enable"]
pub type CHEN1_R = crate::BitReader<bool>;
#[doc = "Field `CHEN1` writer - Analog input channel 1 enable"]
pub type CHEN1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHSR_SPEC, bool, O>;
#[doc = "Field `CHEN2` reader - Analog input channel 2 enable"]
pub type CHEN2_R = crate::BitReader<bool>;
#[doc = "Field `CHEN2` writer - Analog input channel 2 enable"]
pub type CHEN2_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHSR_SPEC, bool, O>;
#[doc = "Field `CHEN3` reader - Analog input channel 3 enable"]
pub type CHEN3_R = crate::BitReader<bool>;
#[doc = "Field `CHEN3` writer - Analog input channel 3 enable"]
pub type CHEN3_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHSR_SPEC, bool, O>;
#[doc = "Field `CHEN4` reader - Analog input channel 4 enable"]
pub type CHEN4_R = crate::BitReader<bool>;
#[doc = "Field `CHEN4` writer - Analog input channel 4 enable"]
pub type CHEN4_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHSR_SPEC, bool, O>;
#[doc = "Field `CHEN5` reader - Analog input channel 5 enable"]
pub type CHEN5_R = crate::BitReader<bool>;
#[doc = "Field `CHEN5` writer - Analog input channel 5 enable"]
pub type CHEN5_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHSR_SPEC, bool, O>;
#[doc = "Field `CHEN6` reader - Analog input channel 6 enable"]
pub type CHEN6_R = crate::BitReader<bool>;
#[doc = "Field `CHEN6` writer - Analog input channel 6 enable"]
pub type CHEN6_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHSR_SPEC, bool, O>;
#[doc = "Field `CHEN7` reader - Analog input channel 7 enable"]
pub type CHEN7_R = crate::BitReader<bool>;
#[doc = "Field `CHEN7` writer - Analog input channel 7 enable"]
pub type CHEN7_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHSR_SPEC, bool, O>;
#[doc = "Field `CHEN8` reader - Analog input channel 8 enable"]
pub type CHEN8_R = crate::BitReader<bool>;
#[doc = "Field `CHEN8` writer - Analog input channel 8 enable"]
pub type CHEN8_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHSR_SPEC, bool, O>;
#[doc = "Field `CHEN9` reader - Analog input channel 9 enable"]
pub type CHEN9_R = crate::BitReader<bool>;
#[doc = "Field `CHEN9` writer - Analog input channel 9 enable"]
pub type CHEN9_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHSR_SPEC, bool, O>;
#[doc = "Field `CHENTS` reader - emperature Sensor enable"]
pub type CHENTS_R = crate::BitReader<bool>;
#[doc = "Field `CHENTS` writer - emperature Sensor enable"]
pub type CHENTS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHSR_SPEC, bool, O>;
#[doc = "Field `CHENVS` reader - Voltage Sensor enable"]
pub type CHENVS_R = crate::BitReader<bool>;
#[doc = "Field `CHENVS` writer - Voltage Sensor enable"]
pub type CHENVS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHSR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Analog input channel 0 enable"]
    #[inline(always)]
    pub fn chen0(&self) -> CHEN0_R {
        CHEN0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Analog input channel 1 enable"]
    #[inline(always)]
    pub fn chen1(&self) -> CHEN1_R {
        CHEN1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Analog input channel 2 enable"]
    #[inline(always)]
    pub fn chen2(&self) -> CHEN2_R {
        CHEN2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Analog input channel 3 enable"]
    #[inline(always)]
    pub fn chen3(&self) -> CHEN3_R {
        CHEN3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Analog input channel 4 enable"]
    #[inline(always)]
    pub fn chen4(&self) -> CHEN4_R {
        CHEN4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Analog input channel 5 enable"]
    #[inline(always)]
    pub fn chen5(&self) -> CHEN5_R {
        CHEN5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Analog input channel 6 enable"]
    #[inline(always)]
    pub fn chen6(&self) -> CHEN6_R {
        CHEN6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Analog input channel 7 enable"]
    #[inline(always)]
    pub fn chen7(&self) -> CHEN7_R {
        CHEN7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Analog input channel 8 enable"]
    #[inline(always)]
    pub fn chen8(&self) -> CHEN8_R {
        CHEN8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Analog input channel 9 enable"]
    #[inline(always)]
    pub fn chen9(&self) -> CHEN9_R {
        CHEN9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 14 - emperature Sensor enable"]
    #[inline(always)]
    pub fn chents(&self) -> CHENTS_R {
        CHENTS_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Voltage Sensor enable"]
    #[inline(always)]
    pub fn chenvs(&self) -> CHENVS_R {
        CHENVS_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Analog input channel 0 enable"]
    #[inline(always)]
    #[must_use]
    pub fn chen0(&mut self) -> CHEN0_W<0> {
        CHEN0_W::new(self)
    }
    #[doc = "Bit 1 - Analog input channel 1 enable"]
    #[inline(always)]
    #[must_use]
    pub fn chen1(&mut self) -> CHEN1_W<1> {
        CHEN1_W::new(self)
    }
    #[doc = "Bit 2 - Analog input channel 2 enable"]
    #[inline(always)]
    #[must_use]
    pub fn chen2(&mut self) -> CHEN2_W<2> {
        CHEN2_W::new(self)
    }
    #[doc = "Bit 3 - Analog input channel 3 enable"]
    #[inline(always)]
    #[must_use]
    pub fn chen3(&mut self) -> CHEN3_W<3> {
        CHEN3_W::new(self)
    }
    #[doc = "Bit 4 - Analog input channel 4 enable"]
    #[inline(always)]
    #[must_use]
    pub fn chen4(&mut self) -> CHEN4_W<4> {
        CHEN4_W::new(self)
    }
    #[doc = "Bit 5 - Analog input channel 5 enable"]
    #[inline(always)]
    #[must_use]
    pub fn chen5(&mut self) -> CHEN5_W<5> {
        CHEN5_W::new(self)
    }
    #[doc = "Bit 6 - Analog input channel 6 enable"]
    #[inline(always)]
    #[must_use]
    pub fn chen6(&mut self) -> CHEN6_W<6> {
        CHEN6_W::new(self)
    }
    #[doc = "Bit 7 - Analog input channel 7 enable"]
    #[inline(always)]
    #[must_use]
    pub fn chen7(&mut self) -> CHEN7_W<7> {
        CHEN7_W::new(self)
    }
    #[doc = "Bit 8 - Analog input channel 8 enable"]
    #[inline(always)]
    #[must_use]
    pub fn chen8(&mut self) -> CHEN8_W<8> {
        CHEN8_W::new(self)
    }
    #[doc = "Bit 9 - Analog input channel 9 enable"]
    #[inline(always)]
    #[must_use]
    pub fn chen9(&mut self) -> CHEN9_W<9> {
        CHEN9_W::new(self)
    }
    #[doc = "Bit 14 - emperature Sensor enable"]
    #[inline(always)]
    #[must_use]
    pub fn chents(&mut self) -> CHENTS_W<14> {
        CHENTS_W::new(self)
    }
    #[doc = "Bit 15 - Voltage Sensor enable"]
    #[inline(always)]
    #[must_use]
    pub fn chenvs(&mut self) -> CHENVS_W<15> {
        CHENVS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel select register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chsr](index.html) module"]
pub struct CHSR_SPEC;
impl crate::RegisterSpec for CHSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [chsr::R](R) reader structure"]
impl crate::Readable for CHSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [chsr::W](W) writer structure"]
impl crate::Writable for CHSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CHSR to value 0"]
impl crate::Resettable for CHSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
