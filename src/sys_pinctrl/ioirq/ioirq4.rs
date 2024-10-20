#[doc = "Register `ioirq4` reader"]
pub type R = crate::R<Ioirq4Spec>;
#[doc = "Register `ioirq4` writer"]
pub type W = crate::W<Ioirq4Spec>;
#[doc = "Field `ic1` reader - 1: Do not clear the register, 0: Clear the register"]
pub type Ic1R = crate::FieldReader<u32>;
#[doc = "Field `ic1` writer - 1: Do not clear the register, 0: Clear the register"]
pub type Ic1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 1: Do not clear the register, 0: Clear the register"]
    #[inline(always)]
    pub fn ic1(&self) -> Ic1R {
        Ic1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 1: Do not clear the register, 0: Clear the register"]
    #[inline(always)]
    #[must_use]
    pub fn ic1(&mut self) -> Ic1W<Ioirq4Spec> {
        Ic1W::new(self, 0)
    }
}
#[doc = "SYS IOMUX CFGSAIF SYSCFG IOIRQ 16: GPIO Interrupt Clear\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ioirq4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ioirq4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ioirq4Spec;
impl crate::RegisterSpec for Ioirq4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ioirq4::R`](R) reader structure"]
impl crate::Readable for Ioirq4Spec {}
#[doc = "`write(|w| ..)` method takes [`ioirq4::W`](W) writer structure"]
impl crate::Writable for Ioirq4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ioirq4 to value 0"]
impl crate::Resettable for Ioirq4Spec {
    const RESET_VALUE: u32 = 0;
}
