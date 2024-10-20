#[doc = "Register `threshold` reader"]
pub type R = crate::R<ThresholdSpec>;
#[doc = "Register `threshold` writer"]
pub type W = crate::W<ThresholdSpec>;
#[doc = "Field `threshold` reader - "]
pub type ThresholdR = crate::FieldReader<u32>;
#[doc = "Field `threshold` writer - "]
pub type ThresholdW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn threshold(&self) -> ThresholdR {
        ThresholdR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn threshold(&mut self) -> ThresholdW<ThresholdSpec> {
        ThresholdW::new(self, 0)
    }
}
#[doc = "Interrupt priority threshold of each context. The PLIC will mask all PLIC interrupts of a priority less than or equal to `threshold`.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`threshold::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`threshold::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ThresholdSpec;
impl crate::RegisterSpec for ThresholdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`threshold::R`](R) reader structure"]
impl crate::Readable for ThresholdSpec {}
#[doc = "`write(|w| ..)` method takes [`threshold::W`](W) writer structure"]
impl crate::Writable for ThresholdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets threshold to value 0"]
impl crate::Resettable for ThresholdSpec {
    const RESET_VALUE: u32 = 0;
}
