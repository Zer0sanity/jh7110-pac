#[doc = "Register `ioirq14` reader"]
pub type R = crate::R<Ioirq14Spec>;
#[doc = "Field `mis1` reader - The masked GPIO IRQ status"]
pub type Mis1R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - The masked GPIO IRQ status"]
    #[inline(always)]
    pub fn mis1(&self) -> Mis1R {
        Mis1R::new(self.bits)
    }
}
#[doc = "SYS IOMUX CFGSAIF SYSCFG IOIRQ 56: GPIO Masked Interrupt Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ioirq14::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ioirq14Spec;
impl crate::RegisterSpec for Ioirq14Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ioirq14::R`](R) reader structure"]
impl crate::Readable for Ioirq14Spec {}
#[doc = "`reset()` method sets ioirq14 to value 0"]
impl crate::Resettable for Ioirq14Spec {
    const RESET_VALUE: u32 = 0;
}
