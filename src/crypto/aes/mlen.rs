#[doc = "Register `mlen[%s]` reader"]
pub type R = crate::R<MlenSpec>;
#[doc = "Register `mlen[%s]` writer"]
pub type W = crate::W<MlenSpec>;
#[doc = "Field `mlen` reader - "]
pub type MlenR = crate::FieldReader<u32>;
#[doc = "Field `mlen` writer - "]
pub type MlenW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn mlen(&self) -> MlenR {
        MlenR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn mlen(&mut self) -> MlenW<MlenSpec> {
        MlenW::new(self, 0)
    }
}
#[doc = "JH7110 Crypto AES MLEN\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mlen::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mlen::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MlenSpec;
impl crate::RegisterSpec for MlenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mlen::R`](R) reader structure"]
impl crate::Readable for MlenSpec {}
#[doc = "`write(|w| ..)` method takes [`mlen::W`](W) writer structure"]
impl crate::Writable for MlenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets mlen[%s]
to value 0"]
impl crate::Resettable for MlenSpec {
    const RESET_VALUE: u32 = 0;
}
