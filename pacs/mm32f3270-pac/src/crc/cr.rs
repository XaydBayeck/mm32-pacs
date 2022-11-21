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
#[doc = "Field `RESET` writer - CRC reset"]
pub type RESET_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `POLY32_MGN` reader - CRC algorithm selection"]
pub type POLY32_MGN_R = crate::BitReader<bool>;
#[doc = "Field `POLY32_MGN` writer - CRC algorithm selection"]
pub type POLY32_MGN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `CRC_BITSEL` reader - Input bit selection"]
pub type CRC_BITSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CRC_BITSEL` writer - Input bit selection"]
pub type CRC_BITSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u8, u8, 2, O>;
#[doc = "Field `BIG_EI` reader - Input big or small selection"]
pub type BIG_EI_R = crate::BitReader<bool>;
#[doc = "Field `BIG_EI` writer - Input big or small selection"]
pub type BIG_EI_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `BIG_EO` reader - Output big or small selection"]
pub type BIG_EO_R = crate::BitReader<bool>;
#[doc = "Field `BIG_EO` writer - Output big or small selection"]
pub type BIG_EO_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 1 - CRC algorithm selection"]
    #[inline(always)]
    pub fn poly32_mgn(&self) -> POLY32_MGN_R {
        POLY32_MGN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Input bit selection"]
    #[inline(always)]
    pub fn crc_bitsel(&self) -> CRC_BITSEL_R {
        CRC_BITSEL_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - Input big or small selection"]
    #[inline(always)]
    pub fn big_ei(&self) -> BIG_EI_R {
        BIG_EI_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Output big or small selection"]
    #[inline(always)]
    pub fn big_eo(&self) -> BIG_EO_R {
        BIG_EO_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CRC reset"]
    #[inline(always)]
    #[must_use]
    pub fn reset(&mut self) -> RESET_W<0> {
        RESET_W::new(self)
    }
    #[doc = "Bit 1 - CRC algorithm selection"]
    #[inline(always)]
    #[must_use]
    pub fn poly32_mgn(&mut self) -> POLY32_MGN_W<1> {
        POLY32_MGN_W::new(self)
    }
    #[doc = "Bits 2:3 - Input bit selection"]
    #[inline(always)]
    #[must_use]
    pub fn crc_bitsel(&mut self) -> CRC_BITSEL_W<2> {
        CRC_BITSEL_W::new(self)
    }
    #[doc = "Bit 4 - Input big or small selection"]
    #[inline(always)]
    #[must_use]
    pub fn big_ei(&mut self) -> BIG_EI_W<4> {
        BIG_EI_W::new(self)
    }
    #[doc = "Bit 5 - Output big or small selection"]
    #[inline(always)]
    #[must_use]
    pub fn big_eo(&mut self) -> BIG_EO_W<5> {
        BIG_EO_W::new(self)
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
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
