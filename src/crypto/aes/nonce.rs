#[doc = "Register `nonce[%s]` reader"]
pub type R = crate::R<NonceSpec>;
#[doc = "Register `nonce[%s]` writer"]
pub type W = crate::W<NonceSpec>;
#[doc = "Field `nonce` reader - "]
pub type NonceR = crate::FieldReader<u32>;
#[doc = "Field `nonce` writer - "]
pub type NonceW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn nonce(&self) -> NonceR {
        NonceR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn nonce(&mut self) -> NonceW<NonceSpec> {
        NonceW::new(self, 0)
    }
}
#[doc = "JH7110 Crypto AES Nonce\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nonce::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nonce::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NonceSpec;
impl crate::RegisterSpec for NonceSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`nonce::R`](R) reader structure"]
impl crate::Readable for NonceSpec {}
#[doc = "`write(|w| ..)` method takes [`nonce::W`](W) writer structure"]
impl crate::Writable for NonceSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets nonce[%s]
to value 0"]
impl crate::Resettable for NonceSpec {
    const RESET_VALUE: u32 = 0;
}
