#[doc = "Register `TXID1_P` reader"]
pub struct R(crate::R<TXID1_P_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TXID1_P_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TXID1_P_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TXID1_P_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TXID1_P` writer"]
pub struct W(crate::W<TXID1_P_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TXID1_P_SPEC>;
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
impl From<crate::W<TXID1_P_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TXID1_P_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ID13` reader - Identifier bit 13"]
pub type ID13_R = crate::BitReader<bool>;
#[doc = "Field `ID13` writer - Identifier bit 13"]
pub type ID13_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXID1_P_SPEC, bool, O>;
#[doc = "Field `ID14` reader - Identifier bit 14"]
pub type ID14_R = crate::BitReader<bool>;
#[doc = "Field `ID14` writer - Identifier bit 14"]
pub type ID14_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXID1_P_SPEC, bool, O>;
#[doc = "Field `ID15` reader - Identifier bit 15"]
pub type ID15_R = crate::BitReader<bool>;
#[doc = "Field `ID15` writer - Identifier bit 15"]
pub type ID15_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXID1_P_SPEC, bool, O>;
#[doc = "Field `ID16` reader - Identifier bit 16"]
pub type ID16_R = crate::BitReader<bool>;
#[doc = "Field `ID16` writer - Identifier bit 16"]
pub type ID16_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXID1_P_SPEC, bool, O>;
#[doc = "Field `ID17` reader - Identifier bit 17"]
pub type ID17_R = crate::BitReader<bool>;
#[doc = "Field `ID17` writer - Identifier bit 17"]
pub type ID17_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXID1_P_SPEC, bool, O>;
#[doc = "Field `ID18` reader - Identifier bit 18"]
pub type ID18_R = crate::BitReader<bool>;
#[doc = "Field `ID18` writer - Identifier bit 18"]
pub type ID18_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXID1_P_SPEC, bool, O>;
#[doc = "Field `ID19` reader - Identifier bit 19"]
pub type ID19_R = crate::BitReader<bool>;
#[doc = "Field `ID19` writer - Identifier bit 19"]
pub type ID19_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXID1_P_SPEC, bool, O>;
#[doc = "Field `ID20` reader - Identifier bit 20"]
pub type ID20_R = crate::BitReader<bool>;
#[doc = "Field `ID20` writer - Identifier bit 20"]
pub type ID20_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXID1_P_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Identifier bit 13"]
    #[inline(always)]
    pub fn id13(&self) -> ID13_R {
        ID13_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Identifier bit 14"]
    #[inline(always)]
    pub fn id14(&self) -> ID14_R {
        ID14_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Identifier bit 15"]
    #[inline(always)]
    pub fn id15(&self) -> ID15_R {
        ID15_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Identifier bit 16"]
    #[inline(always)]
    pub fn id16(&self) -> ID16_R {
        ID16_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Identifier bit 17"]
    #[inline(always)]
    pub fn id17(&self) -> ID17_R {
        ID17_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Identifier bit 18"]
    #[inline(always)]
    pub fn id18(&self) -> ID18_R {
        ID18_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Identifier bit 19"]
    #[inline(always)]
    pub fn id19(&self) -> ID19_R {
        ID19_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Identifier bit 20"]
    #[inline(always)]
    pub fn id20(&self) -> ID20_R {
        ID20_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Identifier bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn id13(&mut self) -> ID13_W<0> {
        ID13_W::new(self)
    }
    #[doc = "Bit 1 - Identifier bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn id14(&mut self) -> ID14_W<1> {
        ID14_W::new(self)
    }
    #[doc = "Bit 2 - Identifier bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn id15(&mut self) -> ID15_W<2> {
        ID15_W::new(self)
    }
    #[doc = "Bit 3 - Identifier bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn id16(&mut self) -> ID16_W<3> {
        ID16_W::new(self)
    }
    #[doc = "Bit 4 - Identifier bit 17"]
    #[inline(always)]
    #[must_use]
    pub fn id17(&mut self) -> ID17_W<4> {
        ID17_W::new(self)
    }
    #[doc = "Bit 5 - Identifier bit 18"]
    #[inline(always)]
    #[must_use]
    pub fn id18(&mut self) -> ID18_W<5> {
        ID18_W::new(self)
    }
    #[doc = "Bit 6 - Identifier bit 19"]
    #[inline(always)]
    #[must_use]
    pub fn id19(&mut self) -> ID19_W<6> {
        ID19_W::new(self)
    }
    #[doc = "Bit 7 - Identifier bit 20"]
    #[inline(always)]
    #[must_use]
    pub fn id20(&mut self) -> ID20_W<7> {
        ID20_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Peli TX ID register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txid1_p](index.html) module"]
pub struct TXID1_P_SPEC;
impl crate::RegisterSpec for TXID1_P_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [txid1_p::R](R) reader structure"]
impl crate::Readable for TXID1_P_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [txid1_p::W](W) writer structure"]
impl crate::Writable for TXID1_P_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TXID1_P to value 0"]
impl crate::Resettable for TXID1_P_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
