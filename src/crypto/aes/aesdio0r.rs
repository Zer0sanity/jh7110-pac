#[doc = "Register `aesdio0r` reader"]
pub type R = crate::R<Aesdio0rSpec>;
#[doc = "Register `aesdio0r` writer"]
pub type W = crate::W<Aesdio0rSpec>;
#[doc = "Field `aesdio0r` reader - "]
pub type Aesdio0rR = crate::FieldReader<u32>;
#[doc = "Field `aesdio0r` writer - "]
pub type Aesdio0rW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn aesdio0r(&self) -> Aesdio0rR {
        Aesdio0rR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn aesdio0r(&mut self) -> Aesdio0rW<Aesdio0rSpec> {
        Aesdio0rW::new(self, 0)
    }
}
#[doc = "JH7110 Crypto AES AESDIO0R\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aesdio0r::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aesdio0r::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Aesdio0rSpec;
impl crate::RegisterSpec for Aesdio0rSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aesdio0r::R`](R) reader structure"]
impl crate::Readable for Aesdio0rSpec {}
#[doc = "`write(|w| ..)` method takes [`aesdio0r::W`](W) writer structure"]
impl crate::Writable for Aesdio0rSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets aesdio0r to value 0"]
impl crate::Resettable for Aesdio0rSpec {
    const RESET_VALUE: u32 = 0;
}
