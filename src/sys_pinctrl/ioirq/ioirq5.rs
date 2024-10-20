#[doc = "Register `ioirq5` reader"]
pub type R = crate::R<Ioirq5Spec>;
#[doc = "Register `ioirq5` writer"]
pub type W = crate::W<Ioirq5Spec>;
#[doc = "Field `ibe0` reader - 1: Trigger on both edges, 0: Trigger on a single edge"]
pub type Ibe0R = crate::FieldReader<u32>;
#[doc = "Field `ibe0` writer - 1: Trigger on both edges, 0: Trigger on a single edge"]
pub type Ibe0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 1: Trigger on both edges, 0: Trigger on a single edge"]
    #[inline(always)]
    pub fn ibe0(&self) -> Ibe0R {
        Ibe0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 1: Trigger on both edges, 0: Trigger on a single edge"]
    #[inline(always)]
    #[must_use]
    pub fn ibe0(&mut self) -> Ibe0W<Ioirq5Spec> {
        Ibe0W::new(self, 0)
    }
}
#[doc = "SYS IOMUX CFGSAIF SYSCFG IOIRQ 20: GPIO Interrupt Both Edge Trigger Selector\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ioirq5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ioirq5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ioirq5Spec;
impl crate::RegisterSpec for Ioirq5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ioirq5::R`](R) reader structure"]
impl crate::Readable for Ioirq5Spec {}
#[doc = "`write(|w| ..)` method takes [`ioirq5::W`](W) writer structure"]
impl crate::Writable for Ioirq5Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ioirq5 to value 0"]
impl crate::Resettable for Ioirq5Spec {
    const RESET_VALUE: u32 = 0;
}
