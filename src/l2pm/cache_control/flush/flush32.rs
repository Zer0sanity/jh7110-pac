#[doc = "Register `flush32` writer"]
pub type W = crate::W<Flush32Spec>;
#[doc = "Field `addr` writer - 32-bit address of the cache block to flush."]
pub type AddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - 32-bit address of the cache block to flush."]
    #[inline(always)]
    #[must_use]
    pub fn addr(&mut self) -> AddrW<Flush32Spec> {
        AddrW::new(self, 0)
    }
}
#[doc = "L2 Cache Control Flush 32-bit register. Flushes the cache block at the 32-bit address shifted left by 4 bytes.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flush32::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Flush32Spec;
impl crate::RegisterSpec for Flush32Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`flush32::W`](W) writer structure"]
impl crate::Writable for Flush32Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets flush32 to value 0"]
impl crate::Resettable for Flush32Spec {
    const RESET_VALUE: u32 = 0;
}
