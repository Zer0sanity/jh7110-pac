#[doc = "Register `ioirq6` reader"]
pub type R = crate::R<Ioirq6Spec>;
#[doc = "Register `ioirq6` writer"]
pub type W = crate::W<Ioirq6Spec>;
#[doc = "Field `ibe1` reader - 1: Trigger on both edges, 0: Trigger on a single edge"]
pub type Ibe1R = crate::FieldReader<u32>;
#[doc = "Field `ibe1` writer - 1: Trigger on both edges, 0: Trigger on a single edge"]
pub type Ibe1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 1: Trigger on both edges, 0: Trigger on a single edge"]
    #[inline(always)]
    pub fn ibe1(&self) -> Ibe1R {
        Ibe1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 1: Trigger on both edges, 0: Trigger on a single edge"]
    #[inline(always)]
    #[must_use]
    pub fn ibe1(&mut self) -> Ibe1W<Ioirq6Spec> {
        Ibe1W::new(self, 0)
    }
}
#[doc = "SYS IOMUX CFGSAIF SYSCFG IOIRQ 24: GPIO Interrupt Both Edge Trigger Selector\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ioirq6::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ioirq6::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ioirq6Spec;
impl crate::RegisterSpec for Ioirq6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ioirq6::R`](R) reader structure"]
impl crate::Readable for Ioirq6Spec {}
#[doc = "`write(|w| ..)` method takes [`ioirq6::W`](W) writer structure"]
impl crate::Writable for Ioirq6Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ioirq6 to value 0"]
impl crate::Resettable for Ioirq6Spec {
    const RESET_VALUE: u32 = 0;
}
