#[doc = "Register `caefr` reader"]
pub type R = crate::R<CaefrSpec>;
#[doc = "Register `caefr` writer"]
pub type W = crate::W<CaefrSpec>;
#[doc = "Field `caefr` reader - Crypto CAEFR"]
pub type CaefrR = crate::FieldReader<u32>;
#[doc = "Field `caefr` writer - Crypto CAEFR"]
pub type CaefrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Crypto CAEFR"]
    #[inline(always)]
    pub fn caefr(&self) -> CaefrR {
        CaefrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Crypto CAEFR"]
    #[inline(always)]
    #[must_use]
    pub fn caefr(&mut self) -> CaefrW<CaefrSpec> {
        CaefrW::new(self, 0)
    }
}
#[doc = "JH7110 Crypto CAEFR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`caefr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`caefr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CaefrSpec;
impl crate::RegisterSpec for CaefrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`caefr::R`](R) reader structure"]
impl crate::Readable for CaefrSpec {}
#[doc = "`write(|w| ..)` method takes [`caefr::W`](W) writer structure"]
impl crate::Writable for CaefrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets caefr to value 0"]
impl crate::Resettable for CaefrSpec {
    const RESET_VALUE: u32 = 0;
}
