#[doc = "Register `wkr` reader"]
pub type R = crate::R<WkrSpec>;
#[doc = "Register `wkr` writer"]
pub type W = crate::W<WkrSpec>;
#[doc = "Field `wkr` reader - SHA WKR"]
pub type WkrR = crate::FieldReader<u32>;
#[doc = "Field `wkr` writer - SHA WKR"]
pub type WkrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - SHA WKR"]
    #[inline(always)]
    pub fn wkr(&self) -> WkrR {
        WkrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - SHA WKR"]
    #[inline(always)]
    #[must_use]
    pub fn wkr(&mut self) -> WkrW<WkrSpec> {
        WkrW::new(self, 0)
    }
}
#[doc = "JH7110 Crypto SHA WKR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wkr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wkr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WkrSpec;
impl crate::RegisterSpec for WkrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wkr::R`](R) reader structure"]
impl crate::Readable for WkrSpec {}
#[doc = "`write(|w| ..)` method takes [`wkr::W`](W) writer structure"]
impl crate::Writable for WkrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets wkr to value 0"]
impl crate::Resettable for WkrSpec {
    const RESET_VALUE: u32 = 0;
}
