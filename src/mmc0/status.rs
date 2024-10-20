#[doc = "Register `status` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "Register `status` writer"]
pub type W = crate::W<StatusSpec>;
#[doc = "Field `busy` reader - MMC busy"]
pub type BusyR = crate::BitReader;
#[doc = "Field `busy` writer - MMC busy"]
pub type BusyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `fcnt` reader - MMC FCNT"]
pub type FcntR = crate::FieldReader<u16>;
#[doc = "Field `fcnt` writer - MMC FCNT"]
pub type FcntW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
#[doc = "Field `dma_req` reader - MMC DMA request"]
pub type DmaReqR = crate::BitReader;
#[doc = "Field `dma_req` writer - MMC DMA request"]
pub type DmaReqW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 9 - MMC busy"]
    #[inline(always)]
    pub fn busy(&self) -> BusyR {
        BusyR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 17:29 - MMC FCNT"]
    #[inline(always)]
    pub fn fcnt(&self) -> FcntR {
        FcntR::new(((self.bits >> 17) & 0x1fff) as u16)
    }
    #[doc = "Bit 31 - MMC DMA request"]
    #[inline(always)]
    pub fn dma_req(&self) -> DmaReqR {
        DmaReqR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 9 - MMC busy"]
    #[inline(always)]
    #[must_use]
    pub fn busy(&mut self) -> BusyW<StatusSpec> {
        BusyW::new(self, 9)
    }
    #[doc = "Bits 17:29 - MMC FCNT"]
    #[inline(always)]
    #[must_use]
    pub fn fcnt(&mut self) -> FcntW<StatusSpec> {
        FcntW::new(self, 17)
    }
    #[doc = "Bit 31 - MMC DMA request"]
    #[inline(always)]
    #[must_use]
    pub fn dma_req(&mut self) -> DmaReqW<StatusSpec> {
        DmaReqW::new(self, 31)
    }
}
#[doc = "MMC status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`status::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusSpec;
impl crate::RegisterSpec for StatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for StatusSpec {}
#[doc = "`write(|w| ..)` method takes [`status::W`](W) writer structure"]
impl crate::Writable for StatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets status to value 0"]
impl crate::Resettable for StatusSpec {
    const RESET_VALUE: u32 = 0;
}
