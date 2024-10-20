#[doc = "Register `canr` reader"]
pub type R = crate::R<CanrSpec>;
#[doc = "Register `canr` writer"]
pub type W = crate::W<CanrSpec>;
#[doc = "Field `canr` reader - Crypto CANR"]
pub type CanrR = crate::FieldReader<u32>;
#[doc = "Field `canr` writer - Crypto CANR"]
pub type CanrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Crypto CANR"]
    #[inline(always)]
    pub fn canr(&self) -> CanrR {
        CanrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Crypto CANR"]
    #[inline(always)]
    #[must_use]
    pub fn canr(&mut self) -> CanrW<CanrSpec> {
        CanrW::new(self, 0)
    }
}
#[doc = "JH7110 Crypto CANR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`canr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`canr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CanrSpec;
impl crate::RegisterSpec for CanrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`canr::R`](R) reader structure"]
impl crate::Readable for CanrSpec {}
#[doc = "`write(|w| ..)` method takes [`canr::W`](W) writer structure"]
impl crate::Writable for CanrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets canr to value 0"]
impl crate::Resettable for CanrSpec {
    const RESET_VALUE: u32 = 0;
}
