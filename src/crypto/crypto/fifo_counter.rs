#[doc = "Register `fifo_counter` reader"]
pub type R = crate::R<FifoCounterSpec>;
#[doc = "Register `fifo_counter` writer"]
pub type W = crate::W<FifoCounterSpec>;
#[doc = "Field `fifo_counter` reader - Crypto FIFO Counter"]
pub type FifoCounterR = crate::FieldReader<u32>;
#[doc = "Field `fifo_counter` writer - Crypto FIFO Counter"]
pub type FifoCounterW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Crypto FIFO Counter"]
    #[inline(always)]
    pub fn fifo_counter(&self) -> FifoCounterR {
        FifoCounterR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Crypto FIFO Counter"]
    #[inline(always)]
    #[must_use]
    pub fn fifo_counter(&mut self) -> FifoCounterW<FifoCounterSpec> {
        FifoCounterW::new(self, 0)
    }
}
#[doc = "JH7110 Crypto FIFO Counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo_counter::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo_counter::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FifoCounterSpec;
impl crate::RegisterSpec for FifoCounterSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fifo_counter::R`](R) reader structure"]
impl crate::Readable for FifoCounterSpec {}
#[doc = "`write(|w| ..)` method takes [`fifo_counter::W`](W) writer structure"]
impl crate::Writable for FifoCounterSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets fifo_counter to value 0"]
impl crate::Resettable for FifoCounterSpec {
    const RESET_VALUE: u32 = 0;
}
