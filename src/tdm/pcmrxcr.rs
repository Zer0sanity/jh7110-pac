#[doc = "Register `pcmrxcr` reader"]
pub type R = crate::R<PcmrxcrSpec>;
#[doc = "Register `pcmrxcr` writer"]
pub type W = crate::W<PcmrxcrSpec>;
#[doc = "Field `rx_en` reader - TDM RX enable - 0: disable, 1: enable."]
pub type RxEnR = crate::BitReader;
#[doc = "Field `rx_en` writer - TDM RX enable - 0: disable, 1: enable."]
pub type RxEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - TDM RX enable - 0: disable, 1: enable."]
    #[inline(always)]
    pub fn rx_en(&self) -> RxEnR {
        RxEnR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TDM RX enable - 0: disable, 1: enable."]
    #[inline(always)]
    #[must_use]
    pub fn rx_en(&mut self) -> RxEnW<PcmrxcrSpec> {
        RxEnW::new(self, 0)
    }
}
#[doc = "TDM PCM RX Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcmrxcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcmrxcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcmrxcrSpec;
impl crate::RegisterSpec for PcmrxcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcmrxcr::R`](R) reader structure"]
impl crate::Readable for PcmrxcrSpec {}
#[doc = "`write(|w| ..)` method takes [`pcmrxcr::W`](W) writer structure"]
impl crate::Writable for PcmrxcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets pcmrxcr to value 0"]
impl crate::Resettable for PcmrxcrSpec {
    const RESET_VALUE: u32 = 0;
}
