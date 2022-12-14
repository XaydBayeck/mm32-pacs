#[doc = "Register `BDT_PAGE_01` reader"]
pub struct R(crate::R<BDT_PAGE_01_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BDT_PAGE_01_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BDT_PAGE_01_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BDT_PAGE_01_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BDT_PAGE_01` writer"]
pub struct W(crate::W<BDT_PAGE_01_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BDT_PAGE_01_SPEC>;
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
impl From<crate::W<BDT_PAGE_01_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BDT_PAGE_01_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BDT_BA` reader - The 7_bit value provides address bits 15 to 9 of the BDT base address,which defines the location of the buffer descriptor table in the system memory"]
pub type BDT_BA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BDT_BA` writer - The 7_bit value provides address bits 15 to 9 of the BDT base address,which defines the location of the buffer descriptor table in the system memory"]
pub type BDT_BA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BDT_PAGE_01_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bits 1:7 - The 7_bit value provides address bits 15 to 9 of the BDT base address,which defines the location of the buffer descriptor table in the system memory"]
    #[inline(always)]
    pub fn bdt_ba(&self) -> BDT_BA_R {
        BDT_BA_R::new(((self.bits >> 1) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 1:7 - The 7_bit value provides address bits 15 to 9 of the BDT base address,which defines the location of the buffer descriptor table in the system memory"]
    #[inline(always)]
    #[must_use]
    pub fn bdt_ba(&mut self) -> BDT_BA_W<1> {
        BDT_BA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "BDT page register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bdt_page_01](index.html) module"]
pub struct BDT_PAGE_01_SPEC;
impl crate::RegisterSpec for BDT_PAGE_01_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bdt_page_01::R](R) reader structure"]
impl crate::Readable for BDT_PAGE_01_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bdt_page_01::W](W) writer structure"]
impl crate::Writable for BDT_PAGE_01_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BDT_PAGE_01 to value 0"]
impl crate::Resettable for BDT_PAGE_01_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
