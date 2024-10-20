#[doc = "Register `cfg` reader"]
pub type R = crate::R<CfgSpec>;
#[doc = "Register `cfg` writer"]
pub type W = crate::W<CfgSpec>;
#[doc = "Field `en` reader - DMAC Enable value - 0: disable DMAC, 1: enable DMAC"]
pub type EnR = crate::BitReader;
#[doc = "Field `en` writer - DMAC Enable value - 0: disable DMAC, 1: enable DMAC"]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ie` reader - DMAC Interrupt Enable value - 0: disable interrupt, 1: enable interrupt"]
pub type IeR = crate::BitReader;
#[doc = "Field `ie` writer - DMAC Interrupt Enable value - 0: disable interrupt, 1: enable interrupt"]
pub type IeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - DMAC Enable value - 0: disable DMAC, 1: enable DMAC"]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMAC Interrupt Enable value - 0: disable interrupt, 1: enable interrupt"]
    #[inline(always)]
    pub fn ie(&self) -> IeR {
        IeR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMAC Enable value - 0: disable DMAC, 1: enable DMAC"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EnW<CfgSpec> {
        EnW::new(self, 0)
    }
    #[doc = "Bit 1 - DMAC Interrupt Enable value - 0: disable interrupt, 1: enable interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn ie(&mut self) -> IeW<CfgSpec> {
        IeW::new(self, 1)
    }
}
#[doc = "DMAC Configuration register contains the DMAC config settings.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgSpec;
impl crate::RegisterSpec for CfgSpec {
    type Ux = u64;
}
#[doc = "`read()` method returns [`cfg::R`](R) reader structure"]
impl crate::Readable for CfgSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg::W`](W) writer structure"]
impl crate::Writable for CfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
}
#[doc = "`reset()` method sets cfg to value 0"]
impl crate::Resettable for CfgSpec {
    const RESET_VALUE: u64 = 0;
}
