#[doc = "Register `klen` reader"]
pub type R = crate::R<KlenSpec>;
#[doc = "Register `klen` writer"]
pub type W = crate::W<KlenSpec>;
#[doc = "Field `klen` reader - SHA KLEN"]
pub type KlenR = crate::FieldReader<u32>;
#[doc = "Field `klen` writer - SHA KLEN"]
pub type KlenW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - SHA KLEN"]
    #[inline(always)]
    pub fn klen(&self) -> KlenR {
        KlenR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - SHA KLEN"]
    #[inline(always)]
    #[must_use]
    pub fn klen(&mut self) -> KlenW<KlenSpec> {
        KlenW::new(self, 0)
    }
}
#[doc = "JH7110 Crypto SHA KLEN\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`klen::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`klen::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct KlenSpec;
impl crate::RegisterSpec for KlenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`klen::R`](R) reader structure"]
impl crate::Readable for KlenSpec {}
#[doc = "`write(|w| ..)` method takes [`klen::W`](W) writer structure"]
impl crate::Writable for KlenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets klen to value 0"]
impl crate::Resettable for KlenSpec {
    const RESET_VALUE: u32 = 0;
}
