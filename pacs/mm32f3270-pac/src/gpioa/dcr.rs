#[doc = "Register `DCR` reader"]
pub struct R(crate::R<DCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DCR` writer"]
pub struct W(crate::W<DCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DCR_SPEC>;
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
impl From<crate::W<DCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PX0` reader - PX0"]
pub type PX0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PX0` writer - PX0"]
pub type PX0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DCR_SPEC, u8, u8, 2, O>;
#[doc = "Field `PX1` reader - PX1"]
pub type PX1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PX1` writer - PX1"]
pub type PX1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DCR_SPEC, u8, u8, 2, O>;
#[doc = "Field `PX2` reader - PX2"]
pub type PX2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PX2` writer - PX2"]
pub type PX2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DCR_SPEC, u8, u8, 2, O>;
#[doc = "Field `PX3` reader - PX3"]
pub type PX3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PX3` writer - PX3"]
pub type PX3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DCR_SPEC, u8, u8, 2, O>;
#[doc = "Field `PX4` reader - PX4"]
pub type PX4_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PX4` writer - PX4"]
pub type PX4_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DCR_SPEC, u8, u8, 2, O>;
#[doc = "Field `PX5` reader - PX5"]
pub type PX5_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PX5` writer - PX5"]
pub type PX5_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DCR_SPEC, u8, u8, 2, O>;
#[doc = "Field `PX6` reader - PX6"]
pub type PX6_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PX6` writer - PX6"]
pub type PX6_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DCR_SPEC, u8, u8, 2, O>;
#[doc = "Field `PX7` reader - PX7"]
pub type PX7_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PX7` writer - PX7"]
pub type PX7_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DCR_SPEC, u8, u8, 2, O>;
#[doc = "Field `PX8` reader - PX8"]
pub type PX8_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PX8` writer - PX8"]
pub type PX8_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DCR_SPEC, u8, u8, 2, O>;
#[doc = "Field `PX9` reader - PX9"]
pub type PX9_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PX9` writer - PX9"]
pub type PX9_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DCR_SPEC, u8, u8, 2, O>;
#[doc = "Field `PX10` reader - PX10"]
pub type PX10_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PX10` writer - PX10"]
pub type PX10_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DCR_SPEC, u8, u8, 2, O>;
#[doc = "Field `PX11` reader - PX11"]
pub type PX11_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PX11` writer - PX11"]
pub type PX11_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DCR_SPEC, u8, u8, 2, O>;
#[doc = "Field `PX12` reader - PX12"]
pub type PX12_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PX12` writer - PX12"]
pub type PX12_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DCR_SPEC, u8, u8, 2, O>;
#[doc = "Field `PX13` reader - PX13"]
pub type PX13_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PX13` writer - PX13"]
pub type PX13_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DCR_SPEC, u8, u8, 2, O>;
#[doc = "Field `PX14` reader - PX14"]
pub type PX14_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PX14` writer - PX14"]
pub type PX14_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DCR_SPEC, u8, u8, 2, O>;
#[doc = "Field `PX15` reader - PX15"]
pub type PX15_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PX15` writer - PX15"]
pub type PX15_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DCR_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:1 - PX0"]
    #[inline(always)]
    pub fn px0(&self) -> PX0_R {
        PX0_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - PX1"]
    #[inline(always)]
    pub fn px1(&self) -> PX1_R {
        PX1_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - PX2"]
    #[inline(always)]
    pub fn px2(&self) -> PX2_R {
        PX2_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - PX3"]
    #[inline(always)]
    pub fn px3(&self) -> PX3_R {
        PX3_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - PX4"]
    #[inline(always)]
    pub fn px4(&self) -> PX4_R {
        PX4_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - PX5"]
    #[inline(always)]
    pub fn px5(&self) -> PX5_R {
        PX5_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - PX6"]
    #[inline(always)]
    pub fn px6(&self) -> PX6_R {
        PX6_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - PX7"]
    #[inline(always)]
    pub fn px7(&self) -> PX7_R {
        PX7_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - PX8"]
    #[inline(always)]
    pub fn px8(&self) -> PX8_R {
        PX8_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - PX9"]
    #[inline(always)]
    pub fn px9(&self) -> PX9_R {
        PX9_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - PX10"]
    #[inline(always)]
    pub fn px10(&self) -> PX10_R {
        PX10_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - PX11"]
    #[inline(always)]
    pub fn px11(&self) -> PX11_R {
        PX11_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - PX12"]
    #[inline(always)]
    pub fn px12(&self) -> PX12_R {
        PX12_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - PX13"]
    #[inline(always)]
    pub fn px13(&self) -> PX13_R {
        PX13_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - PX14"]
    #[inline(always)]
    pub fn px14(&self) -> PX14_R {
        PX14_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - PX15"]
    #[inline(always)]
    pub fn px15(&self) -> PX15_R {
        PX15_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - PX0"]
    #[inline(always)]
    #[must_use]
    pub fn px0(&mut self) -> PX0_W<0> {
        PX0_W::new(self)
    }
    #[doc = "Bits 2:3 - PX1"]
    #[inline(always)]
    #[must_use]
    pub fn px1(&mut self) -> PX1_W<2> {
        PX1_W::new(self)
    }
    #[doc = "Bits 4:5 - PX2"]
    #[inline(always)]
    #[must_use]
    pub fn px2(&mut self) -> PX2_W<4> {
        PX2_W::new(self)
    }
    #[doc = "Bits 6:7 - PX3"]
    #[inline(always)]
    #[must_use]
    pub fn px3(&mut self) -> PX3_W<6> {
        PX3_W::new(self)
    }
    #[doc = "Bits 8:9 - PX4"]
    #[inline(always)]
    #[must_use]
    pub fn px4(&mut self) -> PX4_W<8> {
        PX4_W::new(self)
    }
    #[doc = "Bits 10:11 - PX5"]
    #[inline(always)]
    #[must_use]
    pub fn px5(&mut self) -> PX5_W<10> {
        PX5_W::new(self)
    }
    #[doc = "Bits 12:13 - PX6"]
    #[inline(always)]
    #[must_use]
    pub fn px6(&mut self) -> PX6_W<12> {
        PX6_W::new(self)
    }
    #[doc = "Bits 14:15 - PX7"]
    #[inline(always)]
    #[must_use]
    pub fn px7(&mut self) -> PX7_W<14> {
        PX7_W::new(self)
    }
    #[doc = "Bits 16:17 - PX8"]
    #[inline(always)]
    #[must_use]
    pub fn px8(&mut self) -> PX8_W<16> {
        PX8_W::new(self)
    }
    #[doc = "Bits 18:19 - PX9"]
    #[inline(always)]
    #[must_use]
    pub fn px9(&mut self) -> PX9_W<18> {
        PX9_W::new(self)
    }
    #[doc = "Bits 20:21 - PX10"]
    #[inline(always)]
    #[must_use]
    pub fn px10(&mut self) -> PX10_W<20> {
        PX10_W::new(self)
    }
    #[doc = "Bits 22:23 - PX11"]
    #[inline(always)]
    #[must_use]
    pub fn px11(&mut self) -> PX11_W<22> {
        PX11_W::new(self)
    }
    #[doc = "Bits 24:25 - PX12"]
    #[inline(always)]
    #[must_use]
    pub fn px12(&mut self) -> PX12_W<24> {
        PX12_W::new(self)
    }
    #[doc = "Bits 26:27 - PX13"]
    #[inline(always)]
    #[must_use]
    pub fn px13(&mut self) -> PX13_W<26> {
        PX13_W::new(self)
    }
    #[doc = "Bits 28:29 - PX14"]
    #[inline(always)]
    #[must_use]
    pub fn px14(&mut self) -> PX14_W<28> {
        PX14_W::new(self)
    }
    #[doc = "Bits 30:31 - PX15"]
    #[inline(always)]
    #[must_use]
    pub fn px15(&mut self) -> PX15_W<30> {
        PX15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port output open drain control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcr](index.html) module"]
pub struct DCR_SPEC;
impl crate::RegisterSpec for DCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dcr::R](R) reader structure"]
impl crate::Readable for DCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dcr::W](W) writer structure"]
impl crate::Writable for DCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DCR to value 0"]
impl crate::Resettable for DCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
