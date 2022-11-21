#[doc = "Register `COMP1_CSR` reader"]
pub struct R(crate::R<COMP1_CSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COMP1_CSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COMP1_CSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COMP1_CSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `COMP1_CSR` writer"]
pub struct W(crate::W<COMP1_CSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<COMP1_CSR_SPEC>;
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
impl From<crate::W<COMP1_CSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<COMP1_CSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EN` reader - Comparator 1 enable"]
pub type EN_R = crate::BitReader<bool>;
#[doc = "Field `EN` writer - Comparator 1 enable"]
pub type EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, COMP1_CSR_SPEC, bool, O>;
#[doc = "Field `MODE` reader - Comparator 1 mode"]
pub type MODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MODE` writer - Comparator 1 mode"]
pub type MODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, COMP1_CSR_SPEC, u8, u8, 2, O>;
#[doc = "Field `INM_SEL` reader - Comparator 1 inverting input selection"]
pub type INM_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INM_SEL` writer - Comparator 1 inverting input selection"]
pub type INM_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, COMP1_CSR_SPEC, u8, u8, 2, O>;
#[doc = "Field `INP_SEL` reader - Comparator 1 normal phase input selection"]
pub type INP_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INP_SEL` writer - Comparator 1 normal phase input selection"]
pub type INP_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, COMP1_CSR_SPEC, u8, u8, 2, O>;
#[doc = "Field `OUT_SEL` reader - Comparator 1 output selection"]
pub type OUT_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OUT_SEL` writer - Comparator 1 output selection"]
pub type OUT_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, COMP1_CSR_SPEC, u8, u8, 4, O>;
#[doc = "Field `POL` reader - Comparator 1 output polarity"]
pub type POL_R = crate::BitReader<bool>;
#[doc = "Field `POL` writer - Comparator 1 output polarity"]
pub type POL_W<'a, const O: u8> = crate::BitWriter<'a, u32, COMP1_CSR_SPEC, bool, O>;
#[doc = "Field `HYST` reader - Comparator 1 hysteresis"]
pub type HYST_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HYST` writer - Comparator 1 hysteresis"]
pub type HYST_W<'a, const O: u8> = crate::FieldWriter<'a, u32, COMP1_CSR_SPEC, u8, u8, 2, O>;
#[doc = "Field `OFLT` reader - Comparator 1 output filter"]
pub type OFLT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OFLT` writer - Comparator 1 output filter"]
pub type OFLT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, COMP1_CSR_SPEC, u8, u8, 3, O>;
#[doc = "Field `OUT` reader - Comparator 1 output status"]
pub type OUT_R = crate::BitReader<bool>;
#[doc = "Field `LOCK` reader - Comparator 1 lock"]
pub type LOCK_R = crate::BitReader<bool>;
#[doc = "Field `LOCK` writer - Comparator 1 lock"]
pub type LOCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, COMP1_CSR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Comparator 1 enable"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 2:3 - Comparator 1 mode"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Comparator 1 inverting input selection"]
    #[inline(always)]
    pub fn inm_sel(&self) -> INM_SEL_R {
        INM_SEL_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 7:8 - Comparator 1 normal phase input selection"]
    #[inline(always)]
    pub fn inp_sel(&self) -> INP_SEL_R {
        INP_SEL_R::new(((self.bits >> 7) & 3) as u8)
    }
    #[doc = "Bits 10:13 - Comparator 1 output selection"]
    #[inline(always)]
    pub fn out_sel(&self) -> OUT_SEL_R {
        OUT_SEL_R::new(((self.bits >> 10) & 0x0f) as u8)
    }
    #[doc = "Bit 15 - Comparator 1 output polarity"]
    #[inline(always)]
    pub fn pol(&self) -> POL_R {
        POL_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:17 - Comparator 1 hysteresis"]
    #[inline(always)]
    pub fn hyst(&self) -> HYST_R {
        HYST_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:20 - Comparator 1 output filter"]
    #[inline(always)]
    pub fn oflt(&self) -> OFLT_R {
        OFLT_R::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bit 30 - Comparator 1 output status"]
    #[inline(always)]
    pub fn out(&self) -> OUT_R {
        OUT_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Comparator 1 lock"]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Comparator 1 enable"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<0> {
        EN_W::new(self)
    }
    #[doc = "Bits 2:3 - Comparator 1 mode"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<2> {
        MODE_W::new(self)
    }
    #[doc = "Bits 4:5 - Comparator 1 inverting input selection"]
    #[inline(always)]
    #[must_use]
    pub fn inm_sel(&mut self) -> INM_SEL_W<4> {
        INM_SEL_W::new(self)
    }
    #[doc = "Bits 7:8 - Comparator 1 normal phase input selection"]
    #[inline(always)]
    #[must_use]
    pub fn inp_sel(&mut self) -> INP_SEL_W<7> {
        INP_SEL_W::new(self)
    }
    #[doc = "Bits 10:13 - Comparator 1 output selection"]
    #[inline(always)]
    #[must_use]
    pub fn out_sel(&mut self) -> OUT_SEL_W<10> {
        OUT_SEL_W::new(self)
    }
    #[doc = "Bit 15 - Comparator 1 output polarity"]
    #[inline(always)]
    #[must_use]
    pub fn pol(&mut self) -> POL_W<15> {
        POL_W::new(self)
    }
    #[doc = "Bits 16:17 - Comparator 1 hysteresis"]
    #[inline(always)]
    #[must_use]
    pub fn hyst(&mut self) -> HYST_W<16> {
        HYST_W::new(self)
    }
    #[doc = "Bits 18:20 - Comparator 1 output filter"]
    #[inline(always)]
    #[must_use]
    pub fn oflt(&mut self) -> OFLT_W<18> {
        OFLT_W::new(self)
    }
    #[doc = "Bit 31 - Comparator 1 lock"]
    #[inline(always)]
    #[must_use]
    pub fn lock(&mut self) -> LOCK_W<31> {
        LOCK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "COMP1 Control State Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [comp1_csr](index.html) module"]
pub struct COMP1_CSR_SPEC;
impl crate::RegisterSpec for COMP1_CSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [comp1_csr::R](R) reader structure"]
impl crate::Readable for COMP1_CSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [comp1_csr::W](W) writer structure"]
impl crate::Writable for COMP1_CSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets COMP1_CSR to value 0"]
impl crate::Resettable for COMP1_CSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
