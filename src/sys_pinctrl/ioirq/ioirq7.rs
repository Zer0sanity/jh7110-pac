#[doc = "Register `ioirq7` reader"]
pub type R = crate::R<Ioirq7Spec>;
#[doc = "Register `ioirq7` writer"]
pub type W = crate::W<Ioirq7Spec>;
#[doc = "Field `iev0` reader - 1: Positive/Low, 0: Negative/High"]
pub type Iev0R = crate::FieldReader<u32>;
#[doc = "Field `iev0` writer - 1: Positive/Low, 0: Negative/High"]
pub type Iev0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 1: Positive/Low, 0: Negative/High"]
    #[inline(always)]
    pub fn iev0(&self) -> Iev0R {
        Iev0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 1: Positive/Low, 0: Negative/High"]
    #[inline(always)]
    #[must_use]
    pub fn iev0(&mut self) -> Iev0W<Ioirq7Spec> {
        Iev0W::new(self, 0)
    }
}
#[doc = "SYS IOMUX CFGSAIF SYSCFG IOIRQ 28: GPIO Interrupt Edge Value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ioirq7::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ioirq7::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ioirq7Spec;
impl crate::RegisterSpec for Ioirq7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ioirq7::R`](R) reader structure"]
impl crate::Readable for Ioirq7Spec {}
#[doc = "`write(|w| ..)` method takes [`ioirq7::W`](W) writer structure"]
impl crate::Writable for Ioirq7Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ioirq7 to value 0"]
impl crate::Resettable for Ioirq7Spec {
    const RESET_VALUE: u32 = 0;
}
