#[doc = "Register `ABRCR` reader"]
pub struct R(crate::R<ABRCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ABRCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ABRCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ABRCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ABRCR` writer"]
pub struct W(crate::W<ABRCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ABRCR_SPEC>;
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
impl From<crate::W<ABRCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ABRCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `Abren` reader - Automatic baud rate enable"]
pub type ABREN_R = crate::BitReader<bool>;
#[doc = "Field `Abren` writer - Automatic baud rate enable"]
pub type ABREN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ABRCR_SPEC, bool, O>;
#[doc = "Field `Abr_bitcnt` reader - Automatic baud rate detection length"]
pub type ABR_BITCNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `Abr_bitcnt` writer - Automatic baud rate detection length"]
pub type ABR_BITCNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ABRCR_SPEC, u8, u8, 2, O>;
#[doc = "Field `Former_edge` reader - Auto baud rate previous edge selection"]
pub type FORMER_EDGE_R = crate::BitReader<bool>;
#[doc = "Field `Former_edge` writer - Auto baud rate previous edge selection"]
pub type FORMER_EDGE_W<'a, const O: u8> = crate::BitWriter<'a, u32, ABRCR_SPEC, bool, O>;
#[doc = "Field `Later_edge` reader - Automatic baud rate after edge selection"]
pub type LATER_EDGE_R = crate::BitReader<bool>;
#[doc = "Field `Later_edge` writer - Automatic baud rate after edge selection"]
pub type LATER_EDGE_W<'a, const O: u8> = crate::BitWriter<'a, u32, ABRCR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Automatic baud rate enable"]
    #[inline(always)]
    pub fn abren(&self) -> ABREN_R {
        ABREN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Automatic baud rate detection length"]
    #[inline(always)]
    pub fn abr_bitcnt(&self) -> ABR_BITCNT_R {
        ABR_BITCNT_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3 - Auto baud rate previous edge selection"]
    #[inline(always)]
    pub fn former_edge(&self) -> FORMER_EDGE_R {
        FORMER_EDGE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Automatic baud rate after edge selection"]
    #[inline(always)]
    pub fn later_edge(&self) -> LATER_EDGE_R {
        LATER_EDGE_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Automatic baud rate enable"]
    #[inline(always)]
    #[must_use]
    pub fn abren(&mut self) -> ABREN_W<0> {
        ABREN_W::new(self)
    }
    #[doc = "Bits 1:2 - Automatic baud rate detection length"]
    #[inline(always)]
    #[must_use]
    pub fn abr_bitcnt(&mut self) -> ABR_BITCNT_W<1> {
        ABR_BITCNT_W::new(self)
    }
    #[doc = "Bit 3 - Auto baud rate previous edge selection"]
    #[inline(always)]
    #[must_use]
    pub fn former_edge(&mut self) -> FORMER_EDGE_W<3> {
        FORMER_EDGE_W::new(self)
    }
    #[doc = "Bit 4 - Automatic baud rate after edge selection"]
    #[inline(always)]
    #[must_use]
    pub fn later_edge(&mut self) -> LATER_EDGE_W<4> {
        LATER_EDGE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Automatic Baud Rate Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [abrcr](index.html) module"]
pub struct ABRCR_SPEC;
impl crate::RegisterSpec for ABRCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [abrcr::R](R) reader structure"]
impl crate::Readable for ABRCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [abrcr::W](W) writer structure"]
impl crate::Writable for ABRCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ABRCR to value 0"]
impl crate::Resettable for ABRCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
