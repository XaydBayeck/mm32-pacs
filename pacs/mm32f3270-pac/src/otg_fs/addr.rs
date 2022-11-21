# [doc = "Register `ADDR` reader"] pub struct R (crate :: R < ADDR_SPEC >) ; impl core :: ops :: Deref for R { type Target = crate :: R < ADDR_SPEC > ; # [inline (always)] fn deref (& self) -> & Self :: Target { & self . 0 } } impl From < crate :: R < ADDR_SPEC > > for R { # [inline (always)] fn from (reader : crate :: R < ADDR_SPEC >) -> Self { R (reader) } } # [doc = "Register `ADDR` writer"] pub struct W (crate :: W < ADDR_SPEC >) ; impl core :: ops :: Deref for W { type Target = crate :: W < ADDR_SPEC > ; # [inline (always)] fn deref (& self) -> & Self :: Target { & self . 0 } } impl core :: ops :: DerefMut for W { # [inline (always)] fn deref_mut (& mut self) -> & mut Self :: Target { & mut self . 0 } } impl From < crate :: W < ADDR_SPEC > > for W { # [inline (always)] fn from (writer : crate :: W < ADDR_SPEC >) -> Self { W (writer) } } # [doc = "Field `ADDR` reader - The USB address of the host is defined in the usb_7 mode"] pub type ADDR_R = crate :: FieldReader < u8 , u8 > ; # [doc = "Field `ADDR` writer - The USB address of the host is defined in the usb_7 mode"] pub type ADDR_W < 'a , const O : u8 > = crate :: FieldWriter < 'a , u32 , ADDR_SPEC , u8 , u8 , 7 , O > ; # [doc = "Field `LS_EN` reader - Enabling this bit tells otg_fs that the next token command to write to the token register must be executed at a low speed."] pub type LS_EN_R = crate :: BitReader < bool > ; # [doc = "Field `LS_EN` writer - Enabling this bit tells otg_fs that the next token command to write to the token register must be executed at a low speed."] pub type LS_EN_W < 'a , const O : u8 > = crate :: BitWriter < 'a , u32 , ADDR_SPEC , bool , O > ; impl R { # [doc = "Bits 0:6 - The USB address of the host is defined in the usb_7 mode"] # [inline (always)] pub fn addr (& self) -> ADDR_R { ADDR_R :: new ((self . bits & 0x7f) as u8) } # [doc = "Bit 7 - Enabling this bit tells otg_fs that the next token command to write to the token register must be executed at a low speed."] # [inline (always)] pub fn ls_en (& self) -> LS_EN_R { LS_EN_R :: new (((self . bits >> 7) & 1) != 0) } } impl W { # [doc = "Bits 0:6 - The USB address of the host is defined in the usb_7 mode"] # [inline (always)] # [must_use] pub fn addr (& mut self) -> ADDR_W < 0 > { ADDR_W :: new (self) } # [doc = "Bit 7 - Enabling this bit tells otg_fs that the next token command to write to the token register must be executed at a low speed."] # [inline (always)] # [must_use] pub fn ls_en (& mut self) -> LS_EN_W < 7 > { LS_EN_W :: new (self) } # [doc = "Writes raw bits to the register."] # [inline (always)] pub unsafe fn bits (& mut self , bits : u32) -> & mut Self { self . 0 . bits (bits) ; self } } # [doc = "Address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [addr](index.html) module"] pub struct ADDR_SPEC ; impl crate :: RegisterSpec for ADDR_SPEC { type Ux = u32 ; } # [doc = "`read()` method returns [addr::R](R) reader structure"] impl crate :: Readable for ADDR_SPEC { type Reader = R ; } # [doc = "`write(|w| ..)` method takes [addr::W](W) writer structure"] impl crate :: Writable for ADDR_SPEC { type Writer = W ; const ZERO_TO_MODIFY_FIELDS_BITMAP : Self :: Ux = 0 ; const ONE_TO_MODIFY_FIELDS_BITMAP : Self :: Ux = 0 ; } # [doc = "`reset()` method sets ADDR to value 0"] impl crate :: Resettable for ADDR_SPEC { const RESET_VALUE : Self :: Ux = 0 ; }