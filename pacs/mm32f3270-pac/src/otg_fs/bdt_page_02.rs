#[doc = "Register `BDT_PAGE_02` reader"]
pub struct R(crate::R<BDT_PAGE_02_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BDT_PAGE_02_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BDT_PAGE_02_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BDT_PAGE_02_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BDT_PAGE_02` writer"]
pub struct W(crate::W<BDT_PAGE_02_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BDT_PAGE_02_SPEC>;
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
impl From<crate::W<BDT_PAGE_02_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BDT_PAGE_02_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BDT_BA_23_16` reader - The 8_bit value provides address bits 23 to 16 of the BDT base address, which defines the location of the buffer descriptor table in the system memory"]
pub type BDT_BA_23_16_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BDT_BA_23_16` writer - The 8_bit value provides address bits 23 to 16 of the BDT base address, which defines the location of the buffer descriptor table in the system memory"]
pub type BDT_BA_23_16_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, BDT_PAGE_02_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - The 8_bit value provides address bits 23 to 16 of the BDT base address, which defines the location of the buffer descriptor table in the system memory"]
    #[inline(always)]
    pub fn bdt_ba_23_16(&self) -> BDT_BA_23_16_R {
        BDT_BA_23_16_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - The 8_bit value provides address bits 23 to 16 of the BDT base address, which defines the location of the buffer descriptor table in the system memory"]
    #[inline(always)]
    #[must_use]
    pub fn bdt_ba_23_16(&mut self) -> BDT_BA_23_16_W<0> {
        BDT_BA_23_16_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "BDT page register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bdt_page_02](index.html) module"]
pub struct BDT_PAGE_02_SPEC;
impl crate::RegisterSpec for BDT_PAGE_02_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bdt_page_02::R](R) reader structure"]
impl crate::Readable for BDT_PAGE_02_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bdt_page_02::W](W) writer structure"]
impl crate::Writable for BDT_PAGE_02_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BDT_PAGE_02 to value 0"]
impl crate::Resettable for BDT_PAGE_02_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
