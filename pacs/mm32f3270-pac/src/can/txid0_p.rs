#[doc = "Register `TXID0_P` reader"]
pub struct R(crate::R<TXID0_P_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TXID0_P_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TXID0_P_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TXID0_P_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TXID0_P` writer"]
pub struct W(crate::W<TXID0_P_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TXID0_P_SPEC>;
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
impl From<crate::W<TXID0_P_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TXID0_P_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ID21` reader - Identifier bit 21"]
pub type ID21_R = crate::BitReader<bool>;
#[doc = "Field `ID21` writer - Identifier bit 21"]
pub type ID21_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXID0_P_SPEC, bool, O>;
#[doc = "Field `ID22` reader - Identifier bit 22"]
pub type ID22_R = crate::BitReader<bool>;
#[doc = "Field `ID22` writer - Identifier bit 22"]
pub type ID22_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXID0_P_SPEC, bool, O>;
#[doc = "Field `ID23` reader - Identifier bit 23"]
pub type ID23_R = crate::BitReader<bool>;
#[doc = "Field `ID23` writer - Identifier bit 23"]
pub type ID23_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXID0_P_SPEC, bool, O>;
#[doc = "Field `ID24` reader - Identifier bit 24"]
pub type ID24_R = crate::BitReader<bool>;
#[doc = "Field `ID24` writer - Identifier bit 24"]
pub type ID24_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXID0_P_SPEC, bool, O>;
#[doc = "Field `ID25` reader - Identifier bit 25"]
pub type ID25_R = crate::BitReader<bool>;
#[doc = "Field `ID25` writer - Identifier bit 25"]
pub type ID25_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXID0_P_SPEC, bool, O>;
#[doc = "Field `ID26` reader - Identifier bit 26"]
pub type ID26_R = crate::BitReader<bool>;
#[doc = "Field `ID26` writer - Identifier bit 26"]
pub type ID26_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXID0_P_SPEC, bool, O>;
#[doc = "Field `ID27` reader - Identifier bit 27"]
pub type ID27_R = crate::BitReader<bool>;
#[doc = "Field `ID27` writer - Identifier bit 27"]
pub type ID27_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXID0_P_SPEC, bool, O>;
#[doc = "Field `ID28` reader - Identifier bit 28"]
pub type ID28_R = crate::BitReader<bool>;
#[doc = "Field `ID28` writer - Identifier bit 28"]
pub type ID28_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXID0_P_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Identifier bit 21"]
    #[inline(always)]
    pub fn id21(&self) -> ID21_R {
        ID21_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Identifier bit 22"]
    #[inline(always)]
    pub fn id22(&self) -> ID22_R {
        ID22_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Identifier bit 23"]
    #[inline(always)]
    pub fn id23(&self) -> ID23_R {
        ID23_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Identifier bit 24"]
    #[inline(always)]
    pub fn id24(&self) -> ID24_R {
        ID24_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Identifier bit 25"]
    #[inline(always)]
    pub fn id25(&self) -> ID25_R {
        ID25_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Identifier bit 26"]
    #[inline(always)]
    pub fn id26(&self) -> ID26_R {
        ID26_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Identifier bit 27"]
    #[inline(always)]
    pub fn id27(&self) -> ID27_R {
        ID27_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Identifier bit 28"]
    #[inline(always)]
    pub fn id28(&self) -> ID28_R {
        ID28_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Identifier bit 21"]
    #[inline(always)]
    #[must_use]
    pub fn id21(&mut self) -> ID21_W<0> {
        ID21_W::new(self)
    }
    #[doc = "Bit 1 - Identifier bit 22"]
    #[inline(always)]
    #[must_use]
    pub fn id22(&mut self) -> ID22_W<1> {
        ID22_W::new(self)
    }
    #[doc = "Bit 2 - Identifier bit 23"]
    #[inline(always)]
    #[must_use]
    pub fn id23(&mut self) -> ID23_W<2> {
        ID23_W::new(self)
    }
    #[doc = "Bit 3 - Identifier bit 24"]
    #[inline(always)]
    #[must_use]
    pub fn id24(&mut self) -> ID24_W<3> {
        ID24_W::new(self)
    }
    #[doc = "Bit 4 - Identifier bit 25"]
    #[inline(always)]
    #[must_use]
    pub fn id25(&mut self) -> ID25_W<4> {
        ID25_W::new(self)
    }
    #[doc = "Bit 5 - Identifier bit 26"]
    #[inline(always)]
    #[must_use]
    pub fn id26(&mut self) -> ID26_W<5> {
        ID26_W::new(self)
    }
    #[doc = "Bit 6 - Identifier bit 27"]
    #[inline(always)]
    #[must_use]
    pub fn id27(&mut self) -> ID27_W<6> {
        ID27_W::new(self)
    }
    #[doc = "Bit 7 - Identifier bit 28"]
    #[inline(always)]
    #[must_use]
    pub fn id28(&mut self) -> ID28_W<7> {
        ID28_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Peli TX ID register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txid0_p](index.html) module"]
pub struct TXID0_P_SPEC;
impl crate::RegisterSpec for TXID0_P_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [txid0_p::R](R) reader structure"]
impl crate::Readable for TXID0_P_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [txid0_p::W](W) writer structure"]
impl crate::Writable for TXID0_P_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TXID0_P to value 0"]
impl crate::Resettable for TXID0_P_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
