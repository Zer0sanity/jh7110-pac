#[doc = "Register `ioirq2` reader"]
pub type R = crate::R<Ioirq2Spec>;
#[doc = "Register `ioirq2` writer"]
pub type W = crate::W<Ioirq2Spec>;
#[doc = "Field `is1` reader - 1: Edge trigger, 0: Level trigger"]
pub type Is1R = crate::FieldReader<u32>;
#[doc = "Field `is1` writer - 1: Edge trigger, 0: Level trigger"]
pub type Is1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 1: Edge trigger, 0: Level trigger"]
    #[inline(always)]
    pub fn is1(&self) -> Is1R {
        Is1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 1: Edge trigger, 0: Level trigger"]
    #[inline(always)]
    #[must_use]
    pub fn is1(&mut self) -> Is1W<Ioirq2Spec> {
        Is1W::new(self, 0)
    }
}
#[doc = "SYS IOMUX CFGSAIF SYSCFG IOIRQ 8: GPIO Interrupt Edge Trigger Selector\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ioirq2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ioirq2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ioirq2Spec;
impl crate::RegisterSpec for Ioirq2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ioirq2::R`](R) reader structure"]
impl crate::Readable for Ioirq2Spec {}
#[doc = "`write(|w| ..)` method takes [`ioirq2::W`](W) writer structure"]
impl crate::Writable for Ioirq2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ioirq2 to value 0"]
impl crate::Resettable for Ioirq2Spec {
    const RESET_VALUE: u32 = 0;
}
