#[doc = "Register `ioirq3` reader"]
pub type R = crate::R<Ioirq3Spec>;
#[doc = "Register `ioirq3` writer"]
pub type W = crate::W<Ioirq3Spec>;
#[doc = "Field `ic0` reader - 1: Do not clear the register, 0: Clear the register"]
pub type Ic0R = crate::FieldReader<u32>;
#[doc = "Field `ic0` writer - 1: Do not clear the register, 0: Clear the register"]
pub type Ic0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 1: Do not clear the register, 0: Clear the register"]
    #[inline(always)]
    pub fn ic0(&self) -> Ic0R {
        Ic0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 1: Do not clear the register, 0: Clear the register"]
    #[inline(always)]
    #[must_use]
    pub fn ic0(&mut self) -> Ic0W<Ioirq3Spec> {
        Ic0W::new(self, 0)
    }
}
#[doc = "SYS IOMUX CFGSAIF SYSCFG IOIRQ 12: GPIO Interrupt Clear\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ioirq3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ioirq3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ioirq3Spec;
impl crate::RegisterSpec for Ioirq3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ioirq3::R`](R) reader structure"]
impl crate::Readable for Ioirq3Spec {}
#[doc = "`write(|w| ..)` method takes [`ioirq3::W`](W) writer structure"]
impl crate::Writable for Ioirq3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ioirq3 to value 0"]
impl crate::Resettable for Ioirq3Spec {
    const RESET_VALUE: u32 = 0;
}
