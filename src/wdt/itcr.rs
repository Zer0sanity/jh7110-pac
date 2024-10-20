#[doc = "Register `itcr` reader"]
pub type R = crate::R<ItcrSpec>;
#[doc = "Register `itcr` writer"]
pub type W = crate::W<ItcrSpec>;
#[doc = "Field `enable` reader - Integration test mode enable - 0: disable, 1: enable"]
pub type EnableR = crate::BitReader;
#[doc = "Field `enable` writer - Integration test mode enable - 0: disable, 1: enable"]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Integration test mode enable - 0: disable, 1: enable"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Integration test mode enable - 0: disable, 1: enable"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> EnableW<ItcrSpec> {
        EnableW::new(self, 0)
    }
}
#[doc = "StarFive JH7110 Watchdog Integration Test Control register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`itcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`itcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ItcrSpec;
impl crate::RegisterSpec for ItcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`itcr::R`](R) reader structure"]
impl crate::Readable for ItcrSpec {}
#[doc = "`write(|w| ..)` method takes [`itcr::W`](W) writer structure"]
impl crate::Writable for ItcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets itcr to value 0xe533_1aae"]
impl crate::Resettable for ItcrSpec {
    const RESET_VALUE: u32 = 0xe533_1aae;
}
