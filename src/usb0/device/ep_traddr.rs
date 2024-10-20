#[doc = "Register `ep_traddr` reader"]
pub type R = crate::R<EpTraddrSpec>;
#[doc = "Register `ep_traddr` writer"]
pub type W = crate::W<EpTraddrSpec>;
#[doc = "Field `ep_traddr` reader - Endpoint transfer address"]
pub type EpTraddrR = crate::FieldReader<u32>;
#[doc = "Field `ep_traddr` writer - Endpoint transfer address"]
pub type EpTraddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Endpoint transfer address"]
    #[inline(always)]
    pub fn ep_traddr(&self) -> EpTraddrR {
        EpTraddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Endpoint transfer address"]
    #[inline(always)]
    #[must_use]
    pub fn ep_traddr(&mut self) -> EpTraddrW<EpTraddrSpec> {
        EpTraddrW::new(self, 0)
    }
}
#[doc = "USB3 Endpoint transfer address.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ep_traddr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ep_traddr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EpTraddrSpec;
impl crate::RegisterSpec for EpTraddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ep_traddr::R`](R) reader structure"]
impl crate::Readable for EpTraddrSpec {}
#[doc = "`write(|w| ..)` method takes [`ep_traddr::W`](W) writer structure"]
impl crate::Writable for EpTraddrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ep_traddr to value 0"]
impl crate::Resettable for EpTraddrSpec {
    const RESET_VALUE: u32 = 0;
}
