#[doc = "Register `dar` reader"]
pub type R = crate::R<DarSpec>;
#[doc = "Register `dar` writer"]
pub type W = crate::W<DarSpec>;
#[doc = "Field `dar` reader - Destination address of DMA transfer"]
pub type DarR = crate::FieldReader<u64>;
#[doc = "Field `dar` writer - Destination address of DMA transfer"]
pub type DarW<'a, REG> = crate::FieldWriter<'a, REG, 64, u64>;
impl R {
    #[doc = "Bits 0:63 - Destination address of DMA transfer"]
    #[inline(always)]
    pub fn dar(&self) -> DarR {
        DarR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:63 - Destination address of DMA transfer"]
    #[inline(always)]
    #[must_use]
    pub fn dar(&mut self) -> DarW<DarSpec> {
        DarW::new(self, 0)
    }
}
#[doc = "DMAC Channel Destination address of DMA transfer.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dar::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dar::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DarSpec;
impl crate::RegisterSpec for DarSpec {
    type Ux = u64;
}
#[doc = "`read()` method returns [`dar::R`](R) reader structure"]
impl crate::Readable for DarSpec {}
#[doc = "`write(|w| ..)` method takes [`dar::W`](W) writer structure"]
impl crate::Writable for DarSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
}
#[doc = "`reset()` method sets dar to value 0"]
impl crate::Resettable for DarSpec {
    const RESET_VALUE: u64 = 0;
}
