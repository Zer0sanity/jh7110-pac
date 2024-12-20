#[doc = "Register `ioirq1` reader"]
pub type R = crate::R<Ioirq1Spec>;
#[doc = "Register `ioirq1` writer"]
pub type W = crate::W<Ioirq1Spec>;
#[doc = "Field `is0` reader - 1: Edge trigger, 0: Level trigger"]
pub type Is0R = crate::FieldReader<u32>;
#[doc = "Field `is0` writer - 1: Edge trigger, 0: Level trigger"]
pub type Is0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 1: Edge trigger, 0: Level trigger"]
    #[inline(always)]
    pub fn is0(&self) -> Is0R {
        Is0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 1: Edge trigger, 0: Level trigger"]
    #[inline(always)]
    #[must_use]
    pub fn is0(&mut self) -> Is0W<Ioirq1Spec> {
        Is0W::new(self, 0)
    }
}
#[doc = "SYS IOMUX CFGSAIF SYSCFG IOIRQ 4: GPIO Interrupt Edge Trigger Selector\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ioirq1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ioirq1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ioirq1Spec;
impl crate::RegisterSpec for Ioirq1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ioirq1::R`](R) reader structure"]
impl crate::Readable for Ioirq1Spec {}
#[doc = "`write(|w| ..)` method takes [`ioirq1::W`](W) writer structure"]
impl crate::Writable for Ioirq1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ioirq1 to value 0"]
impl crate::Resettable for Ioirq1Spec {
    const RESET_VALUE: u32 = 0;
}
