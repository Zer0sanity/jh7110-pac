#[doc = "Register `caafr` reader"]
pub type R = crate::R<CaafrSpec>;
#[doc = "Register `caafr` writer"]
pub type W = crate::W<CaafrSpec>;
#[doc = "Field `caafr` reader - Crypto CAAFR"]
pub type CaafrR = crate::FieldReader<u32>;
#[doc = "Field `caafr` writer - Crypto CAAFR"]
pub type CaafrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Crypto CAAFR"]
    #[inline(always)]
    pub fn caafr(&self) -> CaafrR {
        CaafrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Crypto CAAFR"]
    #[inline(always)]
    #[must_use]
    pub fn caafr(&mut self) -> CaafrW<CaafrSpec> {
        CaafrW::new(self, 0)
    }
}
#[doc = "JH7110 Crypto CAAFR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`caafr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`caafr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CaafrSpec;
impl crate::RegisterSpec for CaafrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`caafr::R`](R) reader structure"]
impl crate::Readable for CaafrSpec {}
#[doc = "`write(|w| ..)` method takes [`caafr::W`](W) writer structure"]
impl crate::Writable for CaafrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets caafr to value 0"]
impl crate::Resettable for CaafrSpec {
    const RESET_VALUE: u32 = 0;
}
