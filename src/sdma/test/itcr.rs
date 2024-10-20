#[doc = "Register `itcr` reader"]
pub type R = crate::R<ItcrSpec>;
#[doc = "Register `itcr` writer"]
pub type W = crate::W<ItcrSpec>;
#[doc = "Field `test` reader - Test mode enable - 0: normal operation, 1: test registers multiplexed onto inputs and outputs. Multiplex the test registers to control the input and output lines."]
pub type TestR = crate::BitReader;
#[doc = "Field `test` writer - Test mode enable - 0: normal operation, 1: test registers multiplexed onto inputs and outputs. Multiplex the test registers to control the input and output lines."]
pub type TestW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Test mode enable - 0: normal operation, 1: test registers multiplexed onto inputs and outputs. Multiplex the test registers to control the input and output lines."]
    #[inline(always)]
    pub fn test(&self) -> TestR {
        TestR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Test mode enable - 0: normal operation, 1: test registers multiplexed onto inputs and outputs. Multiplex the test registers to control the input and output lines."]
    #[inline(always)]
    #[must_use]
    pub fn test(&mut self) -> TestW<ItcrSpec> {
        TestW::new(self, 0)
    }
}
#[doc = "DMA Test Control register - enables you to test the DMAC using TIC block-level tests and Built-In Self-Test (BIST) integration and system level tests.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`itcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`itcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets itcr to value 0"]
impl crate::Resettable for ItcrSpec {
    const RESET_VALUE: u32 = 0;
}
