#[doc = "Register `alen[%s]` reader"]
pub type R = crate::R<AlenSpec>;
#[doc = "Register `alen[%s]` writer"]
pub type W = crate::W<AlenSpec>;
#[doc = "Field `alen` reader - "]
pub type AlenR = crate::FieldReader<u32>;
#[doc = "Field `alen` writer - "]
pub type AlenW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn alen(&self) -> AlenR {
        AlenR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn alen(&mut self) -> AlenW<AlenSpec> {
        AlenW::new(self, 0)
    }
}
#[doc = "JH7110 Crypto AES ALEN\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`alen::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`alen::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AlenSpec;
impl crate::RegisterSpec for AlenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`alen::R`](R) reader structure"]
impl crate::Readable for AlenSpec {}
#[doc = "`write(|w| ..)` method takes [`alen::W`](W) writer structure"]
impl crate::Writable for AlenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets alen[%s]
to value 0"]
impl crate::Resettable for AlenSpec {
    const RESET_VALUE: u32 = 0;
}
