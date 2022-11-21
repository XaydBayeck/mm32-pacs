#[doc = "Register `SR` reader"]
pub struct R(crate::R<SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SR` writer"]
pub struct W(crate::W<SR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SR_SPEC>;
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
impl From<crate::W<SR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADIF` reader - ADC interrupt flag"]
pub type ADIF_R = crate::BitReader<bool>;
#[doc = "Field `ADIF` writer - ADC interrupt flag"]
pub type ADIF_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, SR_SPEC, bool, O>;
#[doc = "Field `ADWIF` reader - ADC window comparator interrupt flag"]
pub type ADWIF_R = crate::BitReader<bool>;
#[doc = "Field `ADWIF` writer - ADC window comparator interrupt flag"]
pub type ADWIF_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, SR_SPEC, bool, O>;
#[doc = "Field `BUSY` reader - Busy"]
pub type BUSY_R = crate::BitReader<bool>;
#[doc = "Field `CHANNEL` reader - Current conversion channel"]
pub type CHANNEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VALID` reader - Valid flag"]
pub type VALID_R = crate::FieldReader<u16, u16>;
#[doc = "Field `OVERRUN` reader - Overrun flag"]
pub type OVERRUN_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bit 0 - ADC interrupt flag"]
    #[inline(always)]
    pub fn adif(&self) -> ADIF_R {
        ADIF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ADC window comparator interrupt flag"]
    #[inline(always)]
    pub fn adwif(&self) -> ADWIF_R {
        ADWIF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Busy"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:7 - Current conversion channel"]
    #[inline(always)]
    pub fn channel(&self) -> CHANNEL_R {
        CHANNEL_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:19 - Valid flag"]
    #[inline(always)]
    pub fn valid(&self) -> VALID_R {
        VALID_R::new(((self.bits >> 8) & 0x0fff) as u16)
    }
    #[doc = "Bits 20:31 - Overrun flag"]
    #[inline(always)]
    pub fn overrun(&self) -> OVERRUN_R {
        OVERRUN_R::new(((self.bits >> 20) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - ADC interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn adif(&mut self) -> ADIF_W<0> {
        ADIF_W::new(self)
    }
    #[doc = "Bit 1 - ADC window comparator interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn adwif(&mut self) -> ADWIF_W<1> {
        ADWIF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](index.html) module"]
pub struct SR_SPEC;
impl crate::RegisterSpec for SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sr::R](R) reader structure"]
impl crate::Readable for SR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sr::W](W) writer structure"]
impl crate::Writable for SR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x03;
}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
