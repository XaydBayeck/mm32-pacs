#[doc = "Register `CSR` reader"]
pub struct R(crate::R<CSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TXC` reader - Update interrupt flag"]
pub type TXC_R = crate::BitReader<bool>;
#[doc = "Field `RXAVL` reader - Receive valid data flag bit"]
pub type RXAVL_R = crate::BitReader<bool>;
#[doc = "Field `TXFULL` reader - Transmit buffer full flag bit"]
pub type TXFULL_R = crate::BitReader<bool>;
#[doc = "Field `TXBUF_EMPTY` reader - Transmit buffer empty flag bit"]
pub type TXBUF_EMPTY_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Update interrupt flag"]
    #[inline(always)]
    pub fn txc(&self) -> TXC_R {
        TXC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Receive valid data flag bit"]
    #[inline(always)]
    pub fn rxavl(&self) -> RXAVL_R {
        RXAVL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmit buffer full flag bit"]
    #[inline(always)]
    pub fn txfull(&self) -> TXFULL_R {
        TXFULL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmit buffer empty flag bit"]
    #[inline(always)]
    pub fn txbuf_empty(&self) -> TXBUF_EMPTY_R {
        TXBUF_EMPTY_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "Current status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csr](index.html) module"]
pub struct CSR_SPEC;
impl crate::RegisterSpec for CSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [csr::R](R) reader structure"]
impl crate::Readable for CSR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CSR to value 0x09"]
impl crate::Resettable for CSR_SPEC {
    const RESET_VALUE: Self::Ux = 0x09;
}
