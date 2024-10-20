#[doc = "Register `mtime` reader"]
pub type R = crate::R<MtimeSpec>;
#[doc = "Register `mtime` writer"]
pub type W = crate::W<MtimeSpec>;
#[doc = "Field `cycles` reader - "]
pub type CyclesR = crate::FieldReader<u64>;
#[doc = "Field `cycles` writer - "]
pub type CyclesW<'a, REG> = crate::FieldWriter<'a, REG, 64, u64>;
impl R {
    #[doc = "Bits 0:63"]
    #[inline(always)]
    pub fn cycles(&self) -> CyclesR {
        CyclesR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:63"]
    #[inline(always)]
    #[must_use]
    pub fn cycles(&mut self) -> CyclesW<MtimeSpec> {
        CyclesW::new(self, 0)
    }
}
#[doc = "CLINT MTIME (Machine Time) register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mtime::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mtime::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MtimeSpec;
impl crate::RegisterSpec for MtimeSpec {
    type Ux = u64;
}
#[doc = "`read()` method returns [`mtime::R`](R) reader structure"]
impl crate::Readable for MtimeSpec {}
#[doc = "`write(|w| ..)` method takes [`mtime::W`](W) writer structure"]
impl crate::Writable for MtimeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
}
#[doc = "`reset()` method sets mtime to value 0"]
impl crate::Resettable for MtimeSpec {
    const RESET_VALUE: u64 = 0;
}
