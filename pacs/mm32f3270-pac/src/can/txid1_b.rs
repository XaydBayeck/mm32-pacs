#[doc = "Register `TXID1_B` reader"]
pub struct R(crate::R<TXID1_B_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TXID1_B_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TXID1_B_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TXID1_B_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TXID1_B` writer"]
pub struct W(crate::W<TXID1_B_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TXID1_B_SPEC>;
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
impl From<crate::W<TXID1_B_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TXID1_B_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DLC` reader - Data length code"]
pub type DLC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DLC` writer - Data length code"]
pub type DLC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TXID1_B_SPEC, u8, u8, 4, O>;
#[doc = "Field `RTR` reader - Remote transmission request"]
pub type RTR_R = crate::BitReader<bool>;
#[doc = "Field `RTR` writer - Remote transmission request"]
pub type RTR_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXID1_B_SPEC, bool, O>;
#[doc = "Field `ID0` reader - Identifier bit 0"]
pub type ID0_R = crate::BitReader<bool>;
#[doc = "Field `ID0` writer - Identifier bit 0"]
pub type ID0_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXID1_B_SPEC, bool, O>;
#[doc = "Field `ID1` reader - Identifier bit 1"]
pub type ID1_R = crate::BitReader<bool>;
#[doc = "Field `ID1` writer - Identifier bit 1"]
pub type ID1_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXID1_B_SPEC, bool, O>;
#[doc = "Field `ID2` reader - Identifier bit 2"]
pub type ID2_R = crate::BitReader<bool>;
#[doc = "Field `ID2` writer - Identifier bit 2"]
pub type ID2_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXID1_B_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:3 - Data length code"]
    #[inline(always)]
    pub fn dlc(&self) -> DLC_R {
        DLC_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Remote transmission request"]
    #[inline(always)]
    pub fn rtr(&self) -> RTR_R {
        RTR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Identifier bit 0"]
    #[inline(always)]
    pub fn id0(&self) -> ID0_R {
        ID0_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Identifier bit 1"]
    #[inline(always)]
    pub fn id1(&self) -> ID1_R {
        ID1_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Identifier bit 2"]
    #[inline(always)]
    pub fn id2(&self) -> ID2_R {
        ID2_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Data length code"]
    #[inline(always)]
    #[must_use]
    pub fn dlc(&mut self) -> DLC_W<0> {
        DLC_W::new(self)
    }
    #[doc = "Bit 4 - Remote transmission request"]
    #[inline(always)]
    #[must_use]
    pub fn rtr(&mut self) -> RTR_W<4> {
        RTR_W::new(self)
    }
    #[doc = "Bit 5 - Identifier bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn id0(&mut self) -> ID0_W<5> {
        ID0_W::new(self)
    }
    #[doc = "Bit 6 - Identifier bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn id1(&mut self) -> ID1_W<6> {
        ID1_W::new(self)
    }
    #[doc = "Bit 7 - Identifier bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn id2(&mut self) -> ID2_W<7> {
        ID2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Basic TX ID register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txid1_b](index.html) module"]
pub struct TXID1_B_SPEC;
impl crate::RegisterSpec for TXID1_B_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [txid1_b::R](R) reader structure"]
impl crate::Readable for TXID1_B_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [txid1_b::W](W) writer structure"]
impl crate::Writable for TXID1_B_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TXID1_B to value 0"]
impl crate::Resettable for TXID1_B_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
