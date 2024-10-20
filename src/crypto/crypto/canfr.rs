#[doc = "Register `canfr` reader"]
pub type R = crate::R<CanfrSpec>;
#[doc = "Register `canfr` writer"]
pub type W = crate::W<CanfrSpec>;
#[doc = "Field `canfr` reader - Crypto CANFR"]
pub type CanfrR = crate::FieldReader<u32>;
#[doc = "Field `canfr` writer - Crypto CANFR"]
pub type CanfrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Crypto CANFR"]
    #[inline(always)]
    pub fn canfr(&self) -> CanfrR {
        CanfrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Crypto CANFR"]
    #[inline(always)]
    #[must_use]
    pub fn canfr(&mut self) -> CanfrW<CanfrSpec> {
        CanfrW::new(self, 0)
    }
}
#[doc = "JH7110 Crypto CANFR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`canfr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`canfr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CanfrSpec;
impl crate::RegisterSpec for CanfrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`canfr::R`](R) reader structure"]
impl crate::Readable for CanfrSpec {}
#[doc = "`write(|w| ..)` method takes [`canfr::W`](W) writer structure"]
impl crate::Writable for CanfrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets canfr to value 0"]
impl crate::Resettable for CanfrSpec {
    const RESET_VALUE: u32 = 0;
}
