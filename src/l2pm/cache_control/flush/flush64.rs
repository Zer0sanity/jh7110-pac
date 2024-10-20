#[doc = "Register `flush64` writer"]
pub type W = crate::W<Flush64Spec>;
#[doc = "Field `addr` writer - 64-bit address of the cache block to flush."]
pub type AddrW<'a, REG> = crate::FieldWriter<'a, REG, 64, u64>;
impl W {
    #[doc = "Bits 0:63 - 64-bit address of the cache block to flush."]
    #[inline(always)]
    #[must_use]
    pub fn addr(&mut self) -> AddrW<Flush64Spec> {
        AddrW::new(self, 0)
    }
}
#[doc = "L2 Cache Control Flush 64-bit register. Flushes the cache block at the 64-bit address written.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flush64::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Flush64Spec;
impl crate::RegisterSpec for Flush64Spec {
    type Ux = u64;
}
#[doc = "`write(|w| ..)` method takes [`flush64::W`](W) writer structure"]
impl crate::Writable for Flush64Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
}
#[doc = "`reset()` method sets flush64 to value 0"]
impl crate::Resettable for Flush64Spec {
    const RESET_VALUE: u64 = 0;
}
