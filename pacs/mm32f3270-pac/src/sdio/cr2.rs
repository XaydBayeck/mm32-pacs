#[doc = "Register `CR2` reader"]
pub struct R(crate::R<CR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR2` writer"]
pub struct W(crate::W<CR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR2_SPEC>;
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
impl From<crate::W<CR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AUTODATTR` reader - Set auto data transfer"]
pub type AUTODATTR_R = crate::BitReader<bool>;
#[doc = "Field `AUTODATTR` writer - Set auto data transfer"]
pub type AUTODATTR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
#[doc = "Field `TRDIR` reader - Set data transfer direction"]
pub type TRDIR_R = crate::BitReader<bool>;
#[doc = "Field `TRDIR` writer - Set data transfer direction"]
pub type TRDIR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
#[doc = "Field `AUTOTREN` reader - Set auto 8 null/command/response transfer"]
pub type AUTOTREN_R = crate::BitReader<bool>;
#[doc = "Field `AUTOTREN` writer - Set auto 8 null/command/response transfer"]
pub type AUTOTREN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
#[doc = "Field `OPT` reader - Response/Command selection when bit\\[5\\]
is 0"]
pub type OPT_R = crate::BitReader<bool>;
#[doc = "Field `OPT` writer - Response/Command selection when bit\\[5\\]
is 0"]
pub type OPT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
#[doc = "Field `CID_CSDRD` reader - CID/CSD read control bit"]
pub type CID_CSDRD_R = crate::BitReader<bool>;
#[doc = "Field `CID_CSDRD` writer - CID/CSD read control bit"]
pub type CID_CSDRD_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
#[doc = "Field `PCLKG` reader - SD/MMC/SDIO port CLK line 8 null clocks generation"]
pub type PCLKG_R = crate::BitReader<bool>;
#[doc = "Field `PCLKG` writer - SD/MMC/SDIO port CLK line 8 null clocks generation"]
pub type PCLKG_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
#[doc = "Field `RRESPEN` reader - Enable auto receive response after command"]
pub type RRESPEN_R = crate::BitReader<bool>;
#[doc = "Field `RRESPEN` writer - Enable auto receive response after command"]
pub type RRESPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
#[doc = "Field `AUTOCLKGEN` reader - Enable auto gnerate 8 null clock after response/command or single block data"]
pub type AUTOCLKGEN_R = crate::BitReader<bool>;
#[doc = "Field `AUTOCLKGEN` writer - Enable auto gnerate 8 null clock after response/command or single block data"]
pub type AUTOCLKGEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
#[doc = "Field `CMDCH` reader - SDIO command character"]
pub type CMDCH_R = crate::BitReader<bool>;
#[doc = "Field `CMDCH` writer - SDIO command character"]
pub type CMDCH_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
#[doc = "Field `CMDAF` reader - SDIO cmd12/IO Abort flag"]
pub type CMDAF_R = crate::BitReader<bool>;
#[doc = "Field `CMDAF` writer - SDIO cmd12/IO Abort flag"]
pub type CMDAF_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Set auto data transfer"]
    #[inline(always)]
    pub fn autodattr(&self) -> AUTODATTR_R {
        AUTODATTR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set data transfer direction"]
    #[inline(always)]
    pub fn trdir(&self) -> TRDIR_R {
        TRDIR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Set auto 8 null/command/response transfer"]
    #[inline(always)]
    pub fn autotren(&self) -> AUTOTREN_R {
        AUTOTREN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Response/Command selection when bit\\[5\\]
is 0"]
    #[inline(always)]
    pub fn opt(&self) -> OPT_R {
        OPT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CID/CSD read control bit"]
    #[inline(always)]
    pub fn cid_csdrd(&self) -> CID_CSDRD_R {
        CID_CSDRD_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SD/MMC/SDIO port CLK line 8 null clocks generation"]
    #[inline(always)]
    pub fn pclkg(&self) -> PCLKG_R {
        PCLKG_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable auto receive response after command"]
    #[inline(always)]
    pub fn rrespen(&self) -> RRESPEN_R {
        RRESPEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable auto gnerate 8 null clock after response/command or single block data"]
    #[inline(always)]
    pub fn autoclkgen(&self) -> AUTOCLKGEN_R {
        AUTOCLKGEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - SDIO command character"]
    #[inline(always)]
    pub fn cmdch(&self) -> CMDCH_R {
        CMDCH_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SDIO cmd12/IO Abort flag"]
    #[inline(always)]
    pub fn cmdaf(&self) -> CMDAF_R {
        CMDAF_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Set auto data transfer"]
    #[inline(always)]
    #[must_use]
    pub fn autodattr(&mut self) -> AUTODATTR_W<0> {
        AUTODATTR_W::new(self)
    }
    #[doc = "Bit 1 - Set data transfer direction"]
    #[inline(always)]
    #[must_use]
    pub fn trdir(&mut self) -> TRDIR_W<1> {
        TRDIR_W::new(self)
    }
    #[doc = "Bit 2 - Set auto 8 null/command/response transfer"]
    #[inline(always)]
    #[must_use]
    pub fn autotren(&mut self) -> AUTOTREN_W<2> {
        AUTOTREN_W::new(self)
    }
    #[doc = "Bit 3 - Response/Command selection when bit\\[5\\]
is 0"]
    #[inline(always)]
    #[must_use]
    pub fn opt(&mut self) -> OPT_W<3> {
        OPT_W::new(self)
    }
    #[doc = "Bit 4 - CID/CSD read control bit"]
    #[inline(always)]
    #[must_use]
    pub fn cid_csdrd(&mut self) -> CID_CSDRD_W<4> {
        CID_CSDRD_W::new(self)
    }
    #[doc = "Bit 5 - SD/MMC/SDIO port CLK line 8 null clocks generation"]
    #[inline(always)]
    #[must_use]
    pub fn pclkg(&mut self) -> PCLKG_W<5> {
        PCLKG_W::new(self)
    }
    #[doc = "Bit 6 - Enable auto receive response after command"]
    #[inline(always)]
    #[must_use]
    pub fn rrespen(&mut self) -> RRESPEN_W<6> {
        RRESPEN_W::new(self)
    }
    #[doc = "Bit 7 - Enable auto gnerate 8 null clock after response/command or single block data"]
    #[inline(always)]
    #[must_use]
    pub fn autoclkgen(&mut self) -> AUTOCLKGEN_W<7> {
        AUTOCLKGEN_W::new(self)
    }
    #[doc = "Bit 8 - SDIO command character"]
    #[inline(always)]
    #[must_use]
    pub fn cmdch(&mut self) -> CMDCH_W<8> {
        CMDCH_W::new(self)
    }
    #[doc = "Bit 9 - SDIO cmd12/IO Abort flag"]
    #[inline(always)]
    #[must_use]
    pub fn cmdaf(&mut self) -> CMDAF_W<9> {
        CMDAF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CR2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr2](index.html) module"]
pub struct CR2_SPEC;
impl crate::RegisterSpec for CR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr2::R](R) reader structure"]
impl crate::Readable for CR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr2::W](W) writer structure"]
impl crate::Writable for CR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CR2 to value 0"]
impl crate::Resettable for CR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
