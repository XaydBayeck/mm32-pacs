#[doc = "Register `CR` reader"]
pub struct R(crate::R<CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR` writer"]
pub struct W(crate::W<CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR_SPEC>;
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
impl From<crate::W<CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MASTER` reader - This bit controls whether the DW_apb_i2c master is enabled"]
pub type MASTER_R = crate::BitReader<bool>;
#[doc = "Field `MASTER` writer - This bit controls whether the DW_apb_i2c master is enabled"]
pub type MASTER_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `SPEED` reader - These bits control at which speed the DW_apb_i2c operates"]
pub type SPEED_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SPEED` writer - These bits control at which speed the DW_apb_i2c operates"]
pub type SPEED_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u8, u8, 2, O>;
#[doc = "Field `SLAVE10` reader - When acting as a alsve"]
pub type SLAVE10_R = crate::BitReader<bool>;
#[doc = "Field `SLAVE10` writer - When acting as a alsve"]
pub type SLAVE10_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `MASTER10` reader - Address mode when acting as a master"]
pub type MASTER10_R = crate::BitReader<bool>;
#[doc = "Field `MASTER10` writer - Address mode when acting as a master"]
pub type MASTER10_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `REPEN` reader - Determines whether RESTART comdtions may be sent when acting as a master"]
pub type REPEN_R = crate::BitReader<bool>;
#[doc = "Field `REPEN` writer - Determines whether RESTART comdtions may be sent when acting as a master"]
pub type REPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `DISSLAVE` reader - This bit controls whether I2C has its slave diabled"]
pub type DISSLAVE_R = crate::BitReader<bool>;
#[doc = "Field `DISSLAVE` writer - This bit controls whether I2C has its slave diabled"]
pub type DISSLAVE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `STOPINT` reader - STOP_DET_IFADDRESSED"]
pub type STOPINT_R = crate::BitReader<bool>;
#[doc = "Field `STOPINT` writer - STOP_DET_IFADDRESSED"]
pub type STOPINT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `EMPINT` reader - This bit controls the generation of the TX_EMPTY interrupt"]
pub type EMPINT_R = crate::BitReader<bool>;
#[doc = "Field `EMPINT` writer - This bit controls the generation of the TX_EMPTY interrupt"]
pub type EMPINT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `STOP` reader - Whether to generate a STOP signal after sending or receiving"]
pub type STOP_R = crate::BitReader<bool>;
#[doc = "Field `STOP` writer - Whether to generate a STOP signal after sending or receiving"]
pub type STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `RESTART` reader - Whether to generate a RESTART signal after sending or receiving"]
pub type RESTART_R = crate::BitReader<bool>;
#[doc = "Field `RESTART` writer - Whether to generate a RESTART signal after sending or receiving"]
pub type RESTART_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `SLV_TX_ABRT_DIS` reader - when acting as a slave"]
pub type SLV_TX_ABRT_DIS_R = crate::BitReader<bool>;
#[doc = "Field `SLV_TX_ABRT_DIS` writer - when acting as a slave"]
pub type SLV_TX_ABRT_DIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - This bit controls whether the DW_apb_i2c master is enabled"]
    #[inline(always)]
    pub fn master(&self) -> MASTER_R {
        MASTER_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - These bits control at which speed the DW_apb_i2c operates"]
    #[inline(always)]
    pub fn speed(&self) -> SPEED_R {
        SPEED_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3 - When acting as a alsve"]
    #[inline(always)]
    pub fn slave10(&self) -> SLAVE10_R {
        SLAVE10_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Address mode when acting as a master"]
    #[inline(always)]
    pub fn master10(&self) -> MASTER10_R {
        MASTER10_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Determines whether RESTART comdtions may be sent when acting as a master"]
    #[inline(always)]
    pub fn repen(&self) -> REPEN_R {
        REPEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - This bit controls whether I2C has its slave diabled"]
    #[inline(always)]
    pub fn disslave(&self) -> DISSLAVE_R {
        DISSLAVE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - STOP_DET_IFADDRESSED"]
    #[inline(always)]
    pub fn stopint(&self) -> STOPINT_R {
        STOPINT_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - This bit controls the generation of the TX_EMPTY interrupt"]
    #[inline(always)]
    pub fn empint(&self) -> EMPINT_R {
        EMPINT_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Whether to generate a STOP signal after sending or receiving"]
    #[inline(always)]
    pub fn stop(&self) -> STOP_R {
        STOP_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Whether to generate a RESTART signal after sending or receiving"]
    #[inline(always)]
    pub fn restart(&self) -> RESTART_R {
        RESTART_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - when acting as a slave"]
    #[inline(always)]
    pub fn slv_tx_abrt_dis(&self) -> SLV_TX_ABRT_DIS_R {
        SLV_TX_ABRT_DIS_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This bit controls whether the DW_apb_i2c master is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn master(&mut self) -> MASTER_W<0> {
        MASTER_W::new(self)
    }
    #[doc = "Bits 1:2 - These bits control at which speed the DW_apb_i2c operates"]
    #[inline(always)]
    #[must_use]
    pub fn speed(&mut self) -> SPEED_W<1> {
        SPEED_W::new(self)
    }
    #[doc = "Bit 3 - When acting as a alsve"]
    #[inline(always)]
    #[must_use]
    pub fn slave10(&mut self) -> SLAVE10_W<3> {
        SLAVE10_W::new(self)
    }
    #[doc = "Bit 4 - Address mode when acting as a master"]
    #[inline(always)]
    #[must_use]
    pub fn master10(&mut self) -> MASTER10_W<4> {
        MASTER10_W::new(self)
    }
    #[doc = "Bit 5 - Determines whether RESTART comdtions may be sent when acting as a master"]
    #[inline(always)]
    #[must_use]
    pub fn repen(&mut self) -> REPEN_W<5> {
        REPEN_W::new(self)
    }
    #[doc = "Bit 6 - This bit controls whether I2C has its slave diabled"]
    #[inline(always)]
    #[must_use]
    pub fn disslave(&mut self) -> DISSLAVE_W<6> {
        DISSLAVE_W::new(self)
    }
    #[doc = "Bit 7 - STOP_DET_IFADDRESSED"]
    #[inline(always)]
    #[must_use]
    pub fn stopint(&mut self) -> STOPINT_W<7> {
        STOPINT_W::new(self)
    }
    #[doc = "Bit 8 - This bit controls the generation of the TX_EMPTY interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn empint(&mut self) -> EMPINT_W<8> {
        EMPINT_W::new(self)
    }
    #[doc = "Bit 9 - Whether to generate a STOP signal after sending or receiving"]
    #[inline(always)]
    #[must_use]
    pub fn stop(&mut self) -> STOP_W<9> {
        STOP_W::new(self)
    }
    #[doc = "Bit 10 - Whether to generate a RESTART signal after sending or receiving"]
    #[inline(always)]
    #[must_use]
    pub fn restart(&mut self) -> RESTART_W<10> {
        RESTART_W::new(self)
    }
    #[doc = "Bit 11 - when acting as a slave"]
    #[inline(always)]
    #[must_use]
    pub fn slv_tx_abrt_dis(&mut self) -> SLV_TX_ABRT_DIS_W<11> {
        SLV_TX_ABRT_DIS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](index.html) module"]
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr::R](R) reader structure"]
impl crate::Readable for CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr::W](W) writer structure"]
impl crate::Writable for CR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CR to value 0x7f"]
impl crate::Resettable for CR_SPEC {
    const RESET_VALUE: Self::Ux = 0x7f;
}
