#[doc = "Register `ioirq10` reader"]
pub type R = crate::R<Ioirq10Spec>;
#[doc = "Register `ioirq10` writer"]
pub type W = crate::W<Ioirq10Spec>;
#[doc = "Field `ie1` reader - 1: Unmask, 0: Mask"]
pub type Ie1R = crate::FieldReader<u32>;
#[doc = "Field `ie1` writer - 1: Unmask, 0: Mask"]
pub type Ie1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 1: Unmask, 0: Mask"]
    #[inline(always)]
    pub fn ie1(&self) -> Ie1R {
        Ie1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 1: Unmask, 0: Mask"]
    #[inline(always)]
    #[must_use]
    pub fn ie1(&mut self) -> Ie1W<Ioirq10Spec> {
        Ie1W::new(self, 0)
    }
}
#[doc = "SYS IOMUX CFGSAIF SYSCFG IOIRQ 40: GPIO Interrupt Edge Mask Selector\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ioirq10::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ioirq10::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ioirq10Spec;
impl crate::RegisterSpec for Ioirq10Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ioirq10::R`](R) reader structure"]
impl crate::Readable for Ioirq10Spec {}
#[doc = "`write(|w| ..)` method takes [`ioirq10::W`](W) writer structure"]
impl crate::Writable for Ioirq10Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ioirq10 to value 0"]
impl crate::Resettable for Ioirq10Spec {
    const RESET_VALUE: u32 = 0;
}
