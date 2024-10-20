#[doc = "Register `ioirq8` reader"]
pub type R = crate::R<Ioirq8Spec>;
#[doc = "Register `ioirq8` writer"]
pub type W = crate::W<Ioirq8Spec>;
#[doc = "Field `iev1` reader - 1: Positive/Low, 0: Negative/High"]
pub type Iev1R = crate::FieldReader<u32>;
#[doc = "Field `iev1` writer - 1: Positive/Low, 0: Negative/High"]
pub type Iev1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 1: Positive/Low, 0: Negative/High"]
    #[inline(always)]
    pub fn iev1(&self) -> Iev1R {
        Iev1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 1: Positive/Low, 0: Negative/High"]
    #[inline(always)]
    #[must_use]
    pub fn iev1(&mut self) -> Iev1W<Ioirq8Spec> {
        Iev1W::new(self, 0)
    }
}
#[doc = "SYS IOMUX CFGSAIF SYSCFG IOIRQ 32: GPIO Interrupt Edge Value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ioirq8::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ioirq8::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ioirq8Spec;
impl crate::RegisterSpec for Ioirq8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ioirq8::R`](R) reader structure"]
impl crate::Readable for Ioirq8Spec {}
#[doc = "`write(|w| ..)` method takes [`ioirq8::W`](W) writer structure"]
impl crate::Writable for Ioirq8Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ioirq8 to value 0"]
impl crate::Resettable for Ioirq8Spec {
    const RESET_VALUE: u32 = 0;
}
