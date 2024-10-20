#[doc = "Register `caer` reader"]
pub type R = crate::R<CaerSpec>;
#[doc = "Register `caer` writer"]
pub type W = crate::W<CaerSpec>;
#[doc = "Field `caer` reader - Crypto CAER"]
pub type CaerR = crate::FieldReader<u32>;
#[doc = "Field `caer` writer - Crypto CAER"]
pub type CaerW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Crypto CAER"]
    #[inline(always)]
    pub fn caer(&self) -> CaerR {
        CaerR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Crypto CAER"]
    #[inline(always)]
    #[must_use]
    pub fn caer(&mut self) -> CaerW<CaerSpec> {
        CaerW::new(self, 0)
    }
}
#[doc = "JH7110 Crypto CAER\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`caer::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`caer::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CaerSpec;
impl crate::RegisterSpec for CaerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`caer::R`](R) reader structure"]
impl crate::Readable for CaerSpec {}
#[doc = "`write(|w| ..)` method takes [`caer::W`](W) writer structure"]
impl crate::Writable for CaerSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets caer to value 0"]
impl crate::Resettable for CaerSpec {
    const RESET_VALUE: u32 = 0;
}
