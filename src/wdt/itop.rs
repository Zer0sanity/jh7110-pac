#[doc = "Register `itop` writer"]
pub type W = crate::W<ItopSpec>;
#[doc = "Field `wdogres` writer - Integration test value output on WDOGRES in Integration Test mode - 0: disable, 1: enable"]
pub type WdogresW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wdogint` writer - Integration test value output on WDOGINT in Integration Test mode - 0: disable, 1: enable"]
pub type WdogintW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Integration test value output on WDOGRES in Integration Test mode - 0: disable, 1: enable"]
    #[inline(always)]
    #[must_use]
    pub fn wdogres(&mut self) -> WdogresW<ItopSpec> {
        WdogresW::new(self, 0)
    }
    #[doc = "Bit 1 - Integration test value output on WDOGINT in Integration Test mode - 0: disable, 1: enable"]
    #[inline(always)]
    #[must_use]
    pub fn wdogint(&mut self) -> WdogintW<ItopSpec> {
        WdogintW::new(self, 1)
    }
}
#[doc = "StarFive JH7110 Watchdog Integration Test Operation register.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`itop::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ItopSpec;
impl crate::RegisterSpec for ItopSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`itop::W`](W) writer structure"]
impl crate::Writable for ItopSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets itop to value 0xe533_1aae"]
impl crate::Resettable for ItopSpec {
    const RESET_VALUE: u32 = 0xe533_1aae;
}
