# [doc = "Register `DR7` reader"] pub struct R (crate :: R < DR7_SPEC >) ; impl core :: ops :: Deref for R { type Target = crate :: R < DR7_SPEC > ; # [inline (always)] fn deref (& self) -> & Self :: Target { & self . 0 } } impl From < crate :: R < DR7_SPEC > > for R { # [inline (always)] fn from (reader : crate :: R < DR7_SPEC >) -> Self { R (reader) } } # [doc = "Register `DR7` writer"] pub struct W (crate :: W < DR7_SPEC >) ; impl core :: ops :: Deref for W { type Target = crate :: W < DR7_SPEC > ; # [inline (always)] fn deref (& self) -> & Self :: Target { & self . 0 } } impl core :: ops :: DerefMut for W { # [inline (always)] fn deref_mut (& mut self) -> & mut Self :: Target { & mut self . 0 } } impl From < crate :: W < DR7_SPEC > > for W { # [inline (always)] fn from (writer : crate :: W < DR7_SPEC >) -> Self { W (writer) } } # [doc = "Field `BKP` reader - Backup data"] pub type BKP_R = crate :: FieldReader < u16 , u16 > ; # [doc = "Field `BKP` writer - Backup data"] pub type BKP_W < 'a , const O : u8 > = crate :: FieldWriter < 'a , u32 , DR7_SPEC , u16 , u16 , 16 , O > ; impl R { # [doc = "Bits 0:15 - Backup data"] # [inline (always)] pub fn bkp (& self) -> BKP_R { BKP_R :: new ((self . bits & 0xffff) as u16) } } impl W { # [doc = "Bits 0:15 - Backup data"] # [inline (always)] # [must_use] pub fn bkp (& mut self) -> BKP_W < 0 > { BKP_W :: new (self) } # [doc = "Writes raw bits to the register."] # [inline (always)] pub unsafe fn bits (& mut self , bits : u32) -> & mut Self { self . 0 . bits (bits) ; self } } # [doc = "Backup data register(BKP_DR)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dr7](index.html) module"] pub struct DR7_SPEC ; impl crate :: RegisterSpec for DR7_SPEC { type Ux = u32 ; } # [doc = "`read()` method returns [dr7::R](R) reader structure"] impl crate :: Readable for DR7_SPEC { type Reader = R ; } # [doc = "`write(|w| ..)` method takes [dr7::W](W) writer structure"] impl crate :: Writable for DR7_SPEC { type Writer = W ; const ZERO_TO_MODIFY_FIELDS_BITMAP : Self :: Ux = 0 ; const ONE_TO_MODIFY_FIELDS_BITMAP : Self :: Ux = 0 ; } # [doc = "`reset()` method sets DR7 to value 0"] impl crate :: Resettable for DR7_SPEC { const RESET_VALUE : Self :: Ux = 0 ; }