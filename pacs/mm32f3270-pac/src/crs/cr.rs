#[doc = "Register `CR` reader"]
pub struct R(crate::R<CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR` writer"]
pub struct W(crate::W<CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR_SPEC>;
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
impl From<crate::W<CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OKIE` reader - SYNC event OK interrupt enable"]
pub type OKIE_R = crate::BitReader<bool>;
#[doc = "Field `OKIE` writer - SYNC event OK interrupt enable"]
pub type OKIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `WARNIE` reader - SYNC warning interrupt enable"]
pub type WARNIE_R = crate::BitReader<bool>;
#[doc = "Field `WARNIE` writer - SYNC warning interrupt enable"]
pub type WARNIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `ERRIE` reader - Synchronization or trimming error interrupt enable"]
pub type ERRIE_R = crate::BitReader<bool>;
#[doc = "Field `ERRIE` writer - Synchronization or trimming error interrupt enable"]
pub type ERRIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `EXPTIE` reader - Expected SYNC interrupt enable"]
pub type EXPTIE_R = crate::BitReader<bool>;
#[doc = "Field `EXPTIE` writer - Expected SYNC interrupt enable"]
pub type EXPTIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `CNTEN` reader - Frequency error counter enable"]
pub type CNTEN_R = crate::BitReader<bool>;
#[doc = "Field `CNTEN` writer - Frequency error counter enable"]
pub type CNTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `AUTO_TRIMEN` reader - Automatic trimming enable"]
pub type AUTO_TRIMEN_R = crate::BitReader<bool>;
#[doc = "Field `AUTO_TRIMEN` writer - Automatic trimming enable"]
pub type AUTO_TRIMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `SWSYNC` reader - Generate software SYNC event"]
pub type SWSYNC_R = crate::BitReader<bool>;
#[doc = "Field `SWSYNC` writer - Generate software SYNC event"]
pub type SWSYNC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `TRIM` reader - HSI48 oscillator smooth trimming"]
pub type TRIM_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TRIM` writer - HSI48 oscillator smooth trimming"]
pub type TRIM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u16, u16, 10, O>;
impl R {
    #[doc = "Bit 0 - SYNC event OK interrupt enable"]
    #[inline(always)]
    pub fn okie(&self) -> OKIE_R {
        OKIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SYNC warning interrupt enable"]
    #[inline(always)]
    pub fn warnie(&self) -> WARNIE_R {
        WARNIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Synchronization or trimming error interrupt enable"]
    #[inline(always)]
    pub fn errie(&self) -> ERRIE_R {
        ERRIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Expected SYNC interrupt enable"]
    #[inline(always)]
    pub fn exptie(&self) -> EXPTIE_R {
        EXPTIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - Frequency error counter enable"]
    #[inline(always)]
    pub fn cnten(&self) -> CNTEN_R {
        CNTEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Automatic trimming enable"]
    #[inline(always)]
    pub fn auto_trimen(&self) -> AUTO_TRIMEN_R {
        AUTO_TRIMEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Generate software SYNC event"]
    #[inline(always)]
    pub fn swsync(&self) -> SWSYNC_R {
        SWSYNC_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:17 - HSI48 oscillator smooth trimming"]
    #[inline(always)]
    pub fn trim(&self) -> TRIM_R {
        TRIM_R::new(((self.bits >> 8) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - SYNC event OK interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn okie(&mut self) -> OKIE_W<0> {
        OKIE_W::new(self)
    }
    #[doc = "Bit 1 - SYNC warning interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn warnie(&mut self) -> WARNIE_W<1> {
        WARNIE_W::new(self)
    }
    #[doc = "Bit 2 - Synchronization or trimming error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn errie(&mut self) -> ERRIE_W<2> {
        ERRIE_W::new(self)
    }
    #[doc = "Bit 3 - Expected SYNC interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn exptie(&mut self) -> EXPTIE_W<3> {
        EXPTIE_W::new(self)
    }
    #[doc = "Bit 5 - Frequency error counter enable"]
    #[inline(always)]
    #[must_use]
    pub fn cnten(&mut self) -> CNTEN_W<5> {
        CNTEN_W::new(self)
    }
    #[doc = "Bit 6 - Automatic trimming enable"]
    #[inline(always)]
    #[must_use]
    pub fn auto_trimen(&mut self) -> AUTO_TRIMEN_W<6> {
        AUTO_TRIMEN_W::new(self)
    }
    #[doc = "Bit 7 - Generate software SYNC event"]
    #[inline(always)]
    #[must_use]
    pub fn swsync(&mut self) -> SWSYNC_W<7> {
        SWSYNC_W::new(self)
    }
    #[doc = "Bits 8:17 - HSI48 oscillator smooth trimming"]
    #[inline(always)]
    #[must_use]
    pub fn trim(&mut self) -> TRIM_W<8> {
        TRIM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](index.html) module"]
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr::R](R) reader structure"]
impl crate::Readable for CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr::W](W) writer structure"]
impl crate::Writable for CR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CR to value 0x2000"]
impl crate::Resettable for CR_SPEC {
    const RESET_VALUE: Self::Ux = 0x2000;
}
