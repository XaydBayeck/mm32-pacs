#[doc = "Register `HOLD` reader"]
pub struct R(crate::R<HOLD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HOLD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HOLD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HOLD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HOLD` writer"]
pub struct W(crate::W<HOLD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HOLD_SPEC>;
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
impl From<crate::W<HOLD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HOLD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TX_HOLD` reader - Sets the required SDA hold time in units of ic_clk period"]
pub type TX_HOLD_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TX_HOLD` writer - Sets the required SDA hold time in units of ic_clk period"]
pub type TX_HOLD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HOLD_SPEC, u16, u16, 16, O>;
#[doc = "Field `RX_HOLD` reader - Sets the required SDA hold time in units of ic_clk period"]
pub type RX_HOLD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RX_HOLD` writer - Sets the required SDA hold time in units of ic_clk period"]
pub type RX_HOLD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HOLD_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:15 - Sets the required SDA hold time in units of ic_clk period"]
    #[inline(always)]
    pub fn tx_hold(&self) -> TX_HOLD_R {
        TX_HOLD_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:23 - Sets the required SDA hold time in units of ic_clk period"]
    #[inline(always)]
    pub fn rx_hold(&self) -> RX_HOLD_R {
        RX_HOLD_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - Sets the required SDA hold time in units of ic_clk period"]
    #[inline(always)]
    #[must_use]
    pub fn tx_hold(&mut self) -> TX_HOLD_W<0> {
        TX_HOLD_W::new(self)
    }
    #[doc = "Bits 16:23 - Sets the required SDA hold time in units of ic_clk period"]
    #[inline(always)]
    #[must_use]
    pub fn rx_hold(&mut self) -> RX_HOLD_W<16> {
        RX_HOLD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SDA Hold Time Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hold](index.html) module"]
pub struct HOLD_SPEC;
impl crate::RegisterSpec for HOLD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hold::R](R) reader structure"]
impl crate::Readable for HOLD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hold::W](W) writer structure"]
impl crate::Writable for HOLD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HOLD to value 0x01"]
impl crate::Resettable for HOLD_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
