#[doc = "Register `ivlen` reader"]
pub type R = crate::R<IvlenSpec>;
#[doc = "Register `ivlen` writer"]
pub type W = crate::W<IvlenSpec>;
#[doc = "Field `ivlen` reader - "]
pub type IvlenR = crate::FieldReader<u32>;
#[doc = "Field `ivlen` writer - "]
pub type IvlenW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn ivlen(&self) -> IvlenR {
        IvlenR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn ivlen(&mut self) -> IvlenW<IvlenSpec> {
        IvlenW::new(self, 0)
    }
}
#[doc = "JH7110 Crypto AES IVLEN\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ivlen::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ivlen::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IvlenSpec;
impl crate::RegisterSpec for IvlenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ivlen::R`](R) reader structure"]
impl crate::Readable for IvlenSpec {}
#[doc = "`write(|w| ..)` method takes [`ivlen::W`](W) writer structure"]
impl crate::Writable for IvlenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ivlen to value 0"]
impl crate::Resettable for IvlenSpec {
    const RESET_VALUE: u32 = 0;
}
