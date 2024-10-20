#[doc = "Register `ioirq9` reader"]
pub type R = crate::R<Ioirq9Spec>;
#[doc = "Register `ioirq9` writer"]
pub type W = crate::W<Ioirq9Spec>;
#[doc = "Field `ie0` reader - 1: Unmask, 0: Mask"]
pub type Ie0R = crate::FieldReader<u32>;
#[doc = "Field `ie0` writer - 1: Unmask, 0: Mask"]
pub type Ie0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 1: Unmask, 0: Mask"]
    #[inline(always)]
    pub fn ie0(&self) -> Ie0R {
        Ie0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 1: Unmask, 0: Mask"]
    #[inline(always)]
    #[must_use]
    pub fn ie0(&mut self) -> Ie0W<Ioirq9Spec> {
        Ie0W::new(self, 0)
    }
}
#[doc = "SYS IOMUX CFGSAIF SYSCFG IOIRQ 36: GPIO Interrupt Edge Mask Selector\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ioirq9::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ioirq9::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ioirq9Spec;
impl crate::RegisterSpec for Ioirq9Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ioirq9::R`](R) reader structure"]
impl crate::Readable for Ioirq9Spec {}
#[doc = "`write(|w| ..)` method takes [`ioirq9::W`](W) writer structure"]
impl crate::Writable for Ioirq9Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ioirq9 to value 0"]
impl crate::Resettable for Ioirq9Spec {
    const RESET_VALUE: u32 = 0;
}
