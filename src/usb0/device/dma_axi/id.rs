#[doc = "Register `id` reader"]
pub type R = crate::R<IdSpec>;
#[doc = "Register `id` writer"]
pub type W = crate::W<IdSpec>;
#[doc = "Field `id` reader - DMA AXI ID."]
pub type IdR = crate::FieldReader<u32>;
#[doc = "Field `id` writer - DMA AXI ID."]
pub type IdW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - DMA AXI ID."]
    #[inline(always)]
    pub fn id(&self) -> IdR {
        IdR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - DMA AXI ID."]
    #[inline(always)]
    #[must_use]
    pub fn id(&mut self) -> IdW<IdSpec> {
        IdW::new(self, 0)
    }
}
#[doc = "Device DMA AXI ID.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`id::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`id::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IdSpec;
impl crate::RegisterSpec for IdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`id::R`](R) reader structure"]
impl crate::Readable for IdSpec {}
#[doc = "`write(|w| ..)` method takes [`id::W`](W) writer structure"]
impl crate::Writable for IdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets id to value 0"]
impl crate::Resettable for IdSpec {
    const RESET_VALUE: u32 = 0;
}
