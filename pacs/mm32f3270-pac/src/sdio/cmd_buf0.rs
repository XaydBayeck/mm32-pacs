#[doc = "Register `CMD_BUF0` reader"]
pub struct R(crate::R<CMD_BUF0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMD_BUF0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMD_BUF0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMD_BUF0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CMD_BUF0` writer"]
pub struct W(crate::W<CMD_BUF0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMD_BUF0_SPEC>;
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
impl From<crate::W<CMD_BUF0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMD_BUF0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DAT` reader - cmd_buf byte 0, mapped to command bit 15 to bit 8 bits"]
pub type DAT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DAT` writer - cmd_buf byte 0, mapped to command bit 15 to bit 8 bits"]
pub type DAT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CMD_BUF0_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - cmd_buf byte 0, mapped to command bit 15 to bit 8 bits"]
    #[inline(always)]
    pub fn dat(&self) -> DAT_R {
        DAT_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - cmd_buf byte 0, mapped to command bit 15 to bit 8 bits"]
    #[inline(always)]
    #[must_use]
    pub fn dat(&mut self) -> DAT_W<0> {
        DAT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CMD buffer register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmd_buf0](index.html) module"]
pub struct CMD_BUF0_SPEC;
impl crate::RegisterSpec for CMD_BUF0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmd_buf0::R](R) reader structure"]
impl crate::Readable for CMD_BUF0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cmd_buf0::W](W) writer structure"]
impl crate::Writable for CMD_BUF0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CMD_BUF0 to value 0"]
impl crate::Resettable for CMD_BUF0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
