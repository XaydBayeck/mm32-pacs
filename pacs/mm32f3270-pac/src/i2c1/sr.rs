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
#[doc = "Field `ACTIV` reader - I2C activity status"]
pub type ACTIV_R = crate::BitReader<bool>;
#[doc = "Field `TFNF` reader - Transmit FIFO not full"]
pub type TFNF_R = crate::BitReader<bool>;
#[doc = "Field `TFE` reader - Transmit FIFO completely empty"]
pub type TFE_R = crate::BitReader<bool>;
#[doc = "Field `RFNE` reader - Receive FIFO not empty"]
pub type RFNE_R = crate::BitReader<bool>;
#[doc = "Field `RFF` reader - Receive FIFO completely full"]
pub type RFF_R = crate::BitReader<bool>;
#[doc = "Field `MST_ACTIV` reader - Master FSM activity status"]
pub type MST_ACTIV_R = crate::BitReader<bool>;
#[doc = "Field `SLV_ACTIV` reader - Slave FSM activity status"]
pub type SLV_ACTIV_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - I2C activity status"]
    #[inline(always)]
    pub fn activ(&self) -> ACTIV_R {
        ACTIV_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit FIFO not full"]
    #[inline(always)]
    pub fn tfnf(&self) -> TFNF_R {
        TFNF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmit FIFO completely empty"]
    #[inline(always)]
    pub fn tfe(&self) -> TFE_R {
        TFE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Receive FIFO not empty"]
    #[inline(always)]
    pub fn rfne(&self) -> RFNE_R {
        RFNE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Receive FIFO completely full"]
    #[inline(always)]
    pub fn rff(&self) -> RFF_R {
        RFF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Master FSM activity status"]
    #[inline(always)]
    pub fn mst_activ(&self) -> MST_ACTIV_R {
        MST_ACTIV_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Slave FSM activity status"]
    #[inline(always)]
    pub fn slv_activ(&self) -> SLV_ACTIV_R {
        SLV_ACTIV_R::new(((self.bits >> 6) & 1) != 0)
    }
}
#[doc = "Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](index.html) module"]
pub struct SR_SPEC;
impl crate::RegisterSpec for SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sr::R](R) reader structure"]
impl crate::Readable for SR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SR to value 0x06"]
impl crate::Resettable for SR_SPEC {
    const RESET_VALUE: Self::Ux = 0x06;
}
