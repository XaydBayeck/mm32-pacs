#[doc = "Register `SR` reader"]
pub struct R(crate::R<SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TXEPT` reader - Transmitter empty bit"]
pub type TXEPT_R = crate::BitReader<bool>;
#[doc = "Field `RXAVL` reader - Receive available byte data message"]
pub type RXAVL_R = crate::BitReader<bool>;
#[doc = "Field `TXFULL` reader - Transmitter FIFO full status bit"]
pub type TXFULL_R = crate::BitReader<bool>;
#[doc = "Field `RXAVL_4BYTE` reader - Receive available 4 byte data message"]
pub type RXAVL_4BYTE_R = crate::BitReader<bool>;
#[doc = "Field `TXFADDR` reader - Receive FIFO address"]
pub type TXFADDR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RXFADDR` reader - Transmit FIFO address"]
pub type RXFADDR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BUSY` reader - Is there any data currently being transferred"]
pub type BUSY_R = crate::BitReader<bool>;
#[doc = "Field `CHSIDE` reader - Is the channel currently being transmitted is the left channel or the right channel"]
pub type CHSIDE_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Transmitter empty bit"]
    #[inline(always)]
    pub fn txept(&self) -> TXEPT_R {
        TXEPT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Receive available byte data message"]
    #[inline(always)]
    pub fn rxavl(&self) -> RXAVL_R {
        RXAVL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmitter FIFO full status bit"]
    #[inline(always)]
    pub fn txfull(&self) -> TXFULL_R {
        TXFULL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Receive available 4 byte data message"]
    #[inline(always)]
    pub fn rxavl_4byte(&self) -> RXAVL_4BYTE_R {
        RXAVL_4BYTE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - Receive FIFO address"]
    #[inline(always)]
    pub fn txfaddr(&self) -> TXFADDR_R {
        TXFADDR_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Transmit FIFO address"]
    #[inline(always)]
    pub fn rxfaddr(&self) -> RXFADDR_R {
        RXFADDR_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - Is there any data currently being transferred"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Is the channel currently being transmitted is the left channel or the right channel"]
    #[inline(always)]
    pub fn chside(&self) -> CHSIDE_R {
        CHSIDE_R::new(((self.bits >> 13) & 1) != 0)
    }
}
#[doc = "Current status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](index.html) module"]
pub struct SR_SPEC;
impl crate::RegisterSpec for SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sr::R](R) reader structure"]
impl crate::Readable for SR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SR to value 0x01"]
impl crate::Resettable for SR_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
