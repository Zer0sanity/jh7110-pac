#[doc = "Register `tic_counter_us` reader"]
pub type R = crate::R<TicCounterUsSpec>;
#[doc = "Register `tic_counter_us` writer"]
pub type W = crate::W<TicCounterUsSpec>;
#[doc = "Field `counter` reader - TIC Counter 1 microsecond"]
pub type CounterR = crate::FieldReader<u32>;
#[doc = "Field `counter` writer - TIC Counter 1 microsecond"]
pub type CounterW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - TIC Counter 1 microsecond"]
    #[inline(always)]
    pub fn counter(&self) -> CounterR {
        CounterR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - TIC Counter 1 microsecond"]
    #[inline(always)]
    #[must_use]
    pub fn counter(&mut self) -> CounterW<TicCounterUsSpec> {
        CounterW::new(self, 0)
    }
}
#[doc = "MAC TIC Counter 1 microsecond\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tic_counter_us::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tic_counter_us::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TicCounterUsSpec;
impl crate::RegisterSpec for TicCounterUsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tic_counter_us::R`](R) reader structure"]
impl crate::Readable for TicCounterUsSpec {}
#[doc = "`write(|w| ..)` method takes [`tic_counter_us::W`](W) writer structure"]
impl crate::Writable for TicCounterUsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets tic_counter_us to value 0"]
impl crate::Resettable for TicCounterUsSpec {
    const RESET_VALUE: u32 = 0;
}
