#[doc = "Register `MACA31HR` reader"]
pub struct R(crate::R<MACA31HR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MACA31HR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MACA31HR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MACA31HR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MACA31HR` writer"]
pub struct W(crate::W<MACA31HR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MACA31HR_SPEC>;
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
impl From<crate::W<MACA31HR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MACA31HR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADDH` reader - MAC address15 high \\[47:32\\]"]
pub type ADDH_R = crate::FieldReader<u16, u16>;
#[doc = "Field `ADDH` writer - MAC address15 high \\[47:32\\]"]
pub type ADDH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MACA31HR_SPEC, u16, u16, 16, O>;
#[doc = "Field `MBYTEC` reader - Mask Byte Control"]
pub type MBYTEC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MBYTEC` writer - Mask Byte Control"]
pub type MBYTEC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MACA31HR_SPEC, u8, u8, 6, O>;
#[doc = "Field `SELE` reader - Source address"]
pub type SELE_R = crate::BitReader<bool>;
#[doc = "Field `SELE` writer - Source address"]
pub type SELE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACA31HR_SPEC, bool, O>;
#[doc = "Field `ADDE` reader - Address enable"]
pub type ADDE_R = crate::BitReader<bool>;
#[doc = "Field `ADDE` writer - Address enable"]
pub type ADDE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACA31HR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:15 - MAC address15 high \\[47:32\\]"]
    #[inline(always)]
    pub fn addh(&self) -> ADDH_R {
        ADDH_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 24:29 - Mask Byte Control"]
    #[inline(always)]
    pub fn mbytec(&self) -> MBYTEC_R {
        MBYTEC_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
    #[doc = "Bit 30 - Source address"]
    #[inline(always)]
    pub fn sele(&self) -> SELE_R {
        SELE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Address enable"]
    #[inline(always)]
    pub fn adde(&self) -> ADDE_R {
        ADDE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - MAC address15 high \\[47:32\\]"]
    #[inline(always)]
    #[must_use]
    pub fn addh(&mut self) -> ADDH_W<0> {
        ADDH_W::new(self)
    }
    #[doc = "Bits 24:29 - Mask Byte Control"]
    #[inline(always)]
    #[must_use]
    pub fn mbytec(&mut self) -> MBYTEC_W<24> {
        MBYTEC_W::new(self)
    }
    #[doc = "Bit 30 - Source address"]
    #[inline(always)]
    #[must_use]
    pub fn sele(&mut self) -> SELE_W<30> {
        SELE_W::new(self)
    }
    #[doc = "Bit 31 - Address enable"]
    #[inline(always)]
    #[must_use]
    pub fn adde(&mut self) -> ADDE_W<31> {
        ADDE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ethernet MAC address 15 high register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [maca31hr](index.html) module"]
pub struct MACA31HR_SPEC;
impl crate::RegisterSpec for MACA31HR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [maca31hr::R](R) reader structure"]
impl crate::Readable for MACA31HR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [maca31hr::W](W) writer structure"]
impl crate::Writable for MACA31HR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MACA31HR to value 0"]
impl crate::Resettable for MACA31HR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
