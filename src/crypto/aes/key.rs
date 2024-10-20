#[doc = "Register `key[%s]` reader"]
pub type R = crate::R<KeySpec>;
#[doc = "Register `key[%s]` writer"]
pub type W = crate::W<KeySpec>;
#[doc = "Field `key` reader - "]
pub type KeyR = crate::FieldReader<u32>;
#[doc = "Field `key` writer - "]
pub type KeyW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn key(&self) -> KeyR {
        KeyR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn key(&mut self) -> KeyW<KeySpec> {
        KeyW::new(self, 0)
    }
}
#[doc = "JH7110 Crypto AES Key\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`key::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`key::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct KeySpec;
impl crate::RegisterSpec for KeySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`key::R`](R) reader structure"]
impl crate::Readable for KeySpec {}
#[doc = "`write(|w| ..)` method takes [`key::W`](W) writer structure"]
impl crate::Writable for KeySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets key[%s]
to value 0"]
impl crate::Resettable for KeySpec {
    const RESET_VALUE: u32 = 0;
}
