#[doc = "Register `CCR` reader"]
pub struct R(crate::R<CCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CCR` writer"]
pub struct W(crate::W<CCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCR_SPEC>;
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
impl From<crate::W<CCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PEN` reader - Parity enable bit"]
pub type PEN_R = crate::BitReader<bool>;
#[doc = "Field `PEN` writer - Parity enable bit"]
pub type PEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR_SPEC, bool, O>;
#[doc = "Field `PSEL` reader - Parity selection bit"]
pub type PSEL_R = crate::BitReader<bool>;
#[doc = "Field `PSEL` writer - Parity selection bit"]
pub type PSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR_SPEC, bool, O>;
#[doc = "Field `SPB0` reader - Stop bit 0 selection"]
pub type SPB0_R = crate::BitReader<bool>;
#[doc = "Field `SPB0` writer - Stop bit 0 selection"]
pub type SPB0_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR_SPEC, bool, O>;
#[doc = "Field `BRK` reader - UART transmit frame break"]
pub type BRK_R = crate::BitReader<bool>;
#[doc = "Field `BRK` writer - UART transmit frame break"]
pub type BRK_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR_SPEC, bool, O>;
#[doc = "Field `CHAR` reader - UART width bit"]
pub type CHAR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CHAR` writer - UART width bit"]
pub type CHAR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCR_SPEC, u8, u8, 2, O>;
#[doc = "Field `SPB1` reader - Stop bit 1 selection bit"]
pub type SPB1_R = crate::BitReader<bool>;
#[doc = "Field `SPB1` writer - Stop bit 1 selection bit"]
pub type SPB1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR_SPEC, bool, O>;
#[doc = "Field `B8RXD` reader - Synchronous frame receive"]
pub type B8RXD_R = crate::BitReader<bool>;
#[doc = "Field `B8TXD` reader - Synchronous frame transmit"]
pub type B8TXD_R = crate::BitReader<bool>;
#[doc = "Field `B8TXD` writer - Synchronous frame transmit"]
pub type B8TXD_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR_SPEC, bool, O>;
#[doc = "Field `B8POL` reader - Synchronous frame polarity control bit"]
pub type B8POL_R = crate::BitReader<bool>;
#[doc = "Field `B8POL` writer - Synchronous frame polarity control bit"]
pub type B8POL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR_SPEC, bool, O>;
#[doc = "Field `B8TOG` reader - Synchronous frame auto toggle bit"]
pub type B8TOG_R = crate::BitReader<bool>;
#[doc = "Field `B8TOG` writer - Synchronous frame auto toggle bit"]
pub type B8TOG_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR_SPEC, bool, O>;
#[doc = "Field `B8EN` reader - Synchronous frame enable bit"]
pub type B8EN_R = crate::BitReader<bool>;
#[doc = "Field `B8EN` writer - Synchronous frame enable bit"]
pub type B8EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR_SPEC, bool, O>;
#[doc = "Field `RWU` reader - Receive wake up method"]
pub type RWU_R = crate::BitReader<bool>;
#[doc = "Field `RWU` writer - Receive wake up method"]
pub type RWU_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR_SPEC, bool, O>;
#[doc = "Field `WAKE` reader - Wake up method"]
pub type WAKE_R = crate::BitReader<bool>;
#[doc = "Field `WAKE` writer - Wake up method"]
pub type WAKE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR_SPEC, bool, O>;
#[doc = "Field `LIN` reader - UART LIN enable bit"]
pub type LIN_R = crate::BitReader<bool>;
#[doc = "Field `LIN` writer - UART LIN enable bit"]
pub type LIN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Parity enable bit"]
    #[inline(always)]
    pub fn pen(&self) -> PEN_R {
        PEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Parity selection bit"]
    #[inline(always)]
    pub fn psel(&self) -> PSEL_R {
        PSEL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Stop bit 0 selection"]
    #[inline(always)]
    pub fn spb0(&self) -> SPB0_R {
        SPB0_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - UART transmit frame break"]
    #[inline(always)]
    pub fn brk(&self) -> BRK_R {
        BRK_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - UART width bit"]
    #[inline(always)]
    pub fn char(&self) -> CHAR_R {
        CHAR_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - Stop bit 1 selection bit"]
    #[inline(always)]
    pub fn spb1(&self) -> SPB1_R {
        SPB1_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Synchronous frame receive"]
    #[inline(always)]
    pub fn b8rxd(&self) -> B8RXD_R {
        B8RXD_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Synchronous frame transmit"]
    #[inline(always)]
    pub fn b8txd(&self) -> B8TXD_R {
        B8TXD_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Synchronous frame polarity control bit"]
    #[inline(always)]
    pub fn b8pol(&self) -> B8POL_R {
        B8POL_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Synchronous frame auto toggle bit"]
    #[inline(always)]
    pub fn b8tog(&self) -> B8TOG_R {
        B8TOG_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Synchronous frame enable bit"]
    #[inline(always)]
    pub fn b8en(&self) -> B8EN_R {
        B8EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Receive wake up method"]
    #[inline(always)]
    pub fn rwu(&self) -> RWU_R {
        RWU_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Wake up method"]
    #[inline(always)]
    pub fn wake(&self) -> WAKE_R {
        WAKE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - UART LIN enable bit"]
    #[inline(always)]
    pub fn lin(&self) -> LIN_R {
        LIN_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Parity enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn pen(&mut self) -> PEN_W<0> {
        PEN_W::new(self)
    }
    #[doc = "Bit 1 - Parity selection bit"]
    #[inline(always)]
    #[must_use]
    pub fn psel(&mut self) -> PSEL_W<1> {
        PSEL_W::new(self)
    }
    #[doc = "Bit 2 - Stop bit 0 selection"]
    #[inline(always)]
    #[must_use]
    pub fn spb0(&mut self) -> SPB0_W<2> {
        SPB0_W::new(self)
    }
    #[doc = "Bit 3 - UART transmit frame break"]
    #[inline(always)]
    #[must_use]
    pub fn brk(&mut self) -> BRK_W<3> {
        BRK_W::new(self)
    }
    #[doc = "Bits 4:5 - UART width bit"]
    #[inline(always)]
    #[must_use]
    pub fn char(&mut self) -> CHAR_W<4> {
        CHAR_W::new(self)
    }
    #[doc = "Bit 6 - Stop bit 1 selection bit"]
    #[inline(always)]
    #[must_use]
    pub fn spb1(&mut self) -> SPB1_W<6> {
        SPB1_W::new(self)
    }
    #[doc = "Bit 8 - Synchronous frame transmit"]
    #[inline(always)]
    #[must_use]
    pub fn b8txd(&mut self) -> B8TXD_W<8> {
        B8TXD_W::new(self)
    }
    #[doc = "Bit 9 - Synchronous frame polarity control bit"]
    #[inline(always)]
    #[must_use]
    pub fn b8pol(&mut self) -> B8POL_W<9> {
        B8POL_W::new(self)
    }
    #[doc = "Bit 10 - Synchronous frame auto toggle bit"]
    #[inline(always)]
    #[must_use]
    pub fn b8tog(&mut self) -> B8TOG_W<10> {
        B8TOG_W::new(self)
    }
    #[doc = "Bit 11 - Synchronous frame enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn b8en(&mut self) -> B8EN_W<11> {
        B8EN_W::new(self)
    }
    #[doc = "Bit 12 - Receive wake up method"]
    #[inline(always)]
    #[must_use]
    pub fn rwu(&mut self) -> RWU_W<12> {
        RWU_W::new(self)
    }
    #[doc = "Bit 13 - Wake up method"]
    #[inline(always)]
    #[must_use]
    pub fn wake(&mut self) -> WAKE_W<13> {
        WAKE_W::new(self)
    }
    #[doc = "Bit 14 - UART LIN enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn lin(&mut self) -> LIN_W<14> {
        LIN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "common control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccr](index.html) module"]
pub struct CCR_SPEC;
impl crate::RegisterSpec for CCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ccr::R](R) reader structure"]
impl crate::Readable for CCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ccr::W](W) writer structure"]
impl crate::Writable for CCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CCR to value 0x30"]
impl crate::Resettable for CCR_SPEC {
    const RESET_VALUE: Self::Ux = 0x30;
}
