#[doc = "Register `TXID0_B` reader"]
pub struct R(crate::R<TXID0_B_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TXID0_B_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TXID0_B_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TXID0_B_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TXID0_B` writer"]
pub struct W(crate::W<TXID0_B_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TXID0_B_SPEC>;
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
impl From<crate::W<TXID0_B_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TXID0_B_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ID3` reader - Identifier bit 3"]
pub type ID3_R = crate::BitReader<bool>;
#[doc = "Field `ID3` writer - Identifier bit 3"]
pub type ID3_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXID0_B_SPEC, bool, O>;
#[doc = "Field `ID4` reader - Identifier bit 4"]
pub type ID4_R = crate::BitReader<bool>;
#[doc = "Field `ID4` writer - Identifier bit 4"]
pub type ID4_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXID0_B_SPEC, bool, O>;
#[doc = "Field `ID5` reader - Identifier bit 5"]
pub type ID5_R = crate::BitReader<bool>;
#[doc = "Field `ID5` writer - Identifier bit 5"]
pub type ID5_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXID0_B_SPEC, bool, O>;
#[doc = "Field `ID6` reader - Identifier bit 6"]
pub type ID6_R = crate::BitReader<bool>;
#[doc = "Field `ID6` writer - Identifier bit 6"]
pub type ID6_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXID0_B_SPEC, bool, O>;
#[doc = "Field `ID7` reader - Identifier bit 7"]
pub type ID7_R = crate::BitReader<bool>;
#[doc = "Field `ID7` writer - Identifier bit 7"]
pub type ID7_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXID0_B_SPEC, bool, O>;
#[doc = "Field `ID8` reader - Identifier bit 8"]
pub type ID8_R = crate::BitReader<bool>;
#[doc = "Field `ID8` writer - Identifier bit 8"]
pub type ID8_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXID0_B_SPEC, bool, O>;
#[doc = "Field `ID9` reader - Identifier bit 9"]
pub type ID9_R = crate::BitReader<bool>;
#[doc = "Field `ID9` writer - Identifier bit 9"]
pub type ID9_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXID0_B_SPEC, bool, O>;
#[doc = "Field `ID10` reader - Identifier bit 10"]
pub type ID10_R = crate::BitReader<bool>;
#[doc = "Field `ID10` writer - Identifier bit 10"]
pub type ID10_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXID0_B_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Identifier bit 3"]
    #[inline(always)]
    pub fn id3(&self) -> ID3_R {
        ID3_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Identifier bit 4"]
    #[inline(always)]
    pub fn id4(&self) -> ID4_R {
        ID4_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Identifier bit 5"]
    #[inline(always)]
    pub fn id5(&self) -> ID5_R {
        ID5_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Identifier bit 6"]
    #[inline(always)]
    pub fn id6(&self) -> ID6_R {
        ID6_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Identifier bit 7"]
    #[inline(always)]
    pub fn id7(&self) -> ID7_R {
        ID7_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Identifier bit 8"]
    #[inline(always)]
    pub fn id8(&self) -> ID8_R {
        ID8_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Identifier bit 9"]
    #[inline(always)]
    pub fn id9(&self) -> ID9_R {
        ID9_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Identifier bit 10"]
    #[inline(always)]
    pub fn id10(&self) -> ID10_R {
        ID10_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Identifier bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn id3(&mut self) -> ID3_W<0> {
        ID3_W::new(self)
    }
    #[doc = "Bit 1 - Identifier bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn id4(&mut self) -> ID4_W<1> {
        ID4_W::new(self)
    }
    #[doc = "Bit 2 - Identifier bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn id5(&mut self) -> ID5_W<2> {
        ID5_W::new(self)
    }
    #[doc = "Bit 3 - Identifier bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn id6(&mut self) -> ID6_W<3> {
        ID6_W::new(self)
    }
    #[doc = "Bit 4 - Identifier bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn id7(&mut self) -> ID7_W<4> {
        ID7_W::new(self)
    }
    #[doc = "Bit 5 - Identifier bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn id8(&mut self) -> ID8_W<5> {
        ID8_W::new(self)
    }
    #[doc = "Bit 6 - Identifier bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn id9(&mut self) -> ID9_W<6> {
        ID9_W::new(self)
    }
    #[doc = "Bit 7 - Identifier bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn id10(&mut self) -> ID10_W<7> {
        ID10_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Basic TX ID register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txid0_b](index.html) module"]
pub struct TXID0_B_SPEC;
impl crate::RegisterSpec for TXID0_B_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [txid0_b::R](R) reader structure"]
impl crate::Readable for TXID0_B_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [txid0_b::W](W) writer structure"]
impl crate::Writable for TXID0_B_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TXID0_B to value 0"]
impl crate::Resettable for TXID0_B_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
