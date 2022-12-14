#[doc = "Register `CMR_P` writer"]
pub struct W(crate::W<CMR_P_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMR_P_SPEC>;
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
impl From<crate::W<CMR_P_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMR_P_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TR` writer - Transmission request"]
pub type TR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMR_P_SPEC, bool, O>;
#[doc = "Field `AT` writer - Abort transmission"]
pub type AT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMR_P_SPEC, bool, O>;
#[doc = "Field `RRB` writer - Release receive buffer"]
pub type RRB_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMR_P_SPEC, bool, O>;
#[doc = "Field `CDO` writer - Clear data overrun"]
pub type CDO_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMR_P_SPEC, bool, O>;
#[doc = "Field `SRR` writer - Self reset request"]
pub type SRR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMR_P_SPEC, bool, O>;
#[doc = "Field `ERB` writer - Empty Rxbuffer"]
pub type ERB_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMR_P_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - Transmission request"]
    #[inline(always)]
    #[must_use]
    pub fn tr(&mut self) -> TR_W<0> {
        TR_W::new(self)
    }
    #[doc = "Bit 1 - Abort transmission"]
    #[inline(always)]
    #[must_use]
    pub fn at(&mut self) -> AT_W<1> {
        AT_W::new(self)
    }
    #[doc = "Bit 2 - Release receive buffer"]
    #[inline(always)]
    #[must_use]
    pub fn rrb(&mut self) -> RRB_W<2> {
        RRB_W::new(self)
    }
    #[doc = "Bit 3 - Clear data overrun"]
    #[inline(always)]
    #[must_use]
    pub fn cdo(&mut self) -> CDO_W<3> {
        CDO_W::new(self)
    }
    #[doc = "Bit 4 - Self reset request"]
    #[inline(always)]
    #[must_use]
    pub fn srr(&mut self) -> SRR_W<4> {
        SRR_W::new(self)
    }
    #[doc = "Bit 5 - Empty Rxbuffer"]
    #[inline(always)]
    #[must_use]
    pub fn erb(&mut self) -> ERB_W<5> {
        ERB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Peli Command register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmr_p](index.html) module"]
pub struct CMR_P_SPEC;
impl crate::RegisterSpec for CMR_P_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [cmr_p::W](W) writer structure"]
impl crate::Writable for CMR_P_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CMR_P to value 0"]
impl crate::Resettable for CMR_P_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
