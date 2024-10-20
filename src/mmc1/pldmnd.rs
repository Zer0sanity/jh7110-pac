#[doc = "Register `pldmnd` reader"]
pub type R = crate::R<PldmndSpec>;
#[doc = "Register `pldmnd` writer"]
pub type W = crate::W<PldmndSpec>;
#[doc = "Field `start` reader - MMC Internal DMAC start"]
pub type StartR = crate::BitReader;
#[doc = "Field `start` writer - MMC Internal DMAC start"]
pub type StartW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - MMC Internal DMAC start"]
    #[inline(always)]
    pub fn start(&self) -> StartR {
        StartR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MMC Internal DMAC start"]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> StartW<PldmndSpec> {
        StartW::new(self, 0)
    }
}
#[doc = "MMC PLDMND\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pldmnd::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pldmnd::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PldmndSpec;
impl crate::RegisterSpec for PldmndSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pldmnd::R`](R) reader structure"]
impl crate::Readable for PldmndSpec {}
#[doc = "`write(|w| ..)` method takes [`pldmnd::W`](W) writer structure"]
impl crate::Writable for PldmndSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets pldmnd to value 0"]
impl crate::Resettable for PldmndSpec {
    const RESET_VALUE: u32 = 0;
}
