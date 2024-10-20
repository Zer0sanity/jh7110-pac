#[doc = "Register `ioirq13` reader"]
pub type R = crate::R<Ioirq13Spec>;
#[doc = "Field `mis0` reader - The masked GPIO IRQ status"]
pub type Mis0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - The masked GPIO IRQ status"]
    #[inline(always)]
    pub fn mis0(&self) -> Mis0R {
        Mis0R::new(self.bits)
    }
}
#[doc = "SYS IOMUX CFGSAIF SYSCFG IOIRQ 52: GPIO Masked Interrupt Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ioirq13::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ioirq13Spec;
impl crate::RegisterSpec for Ioirq13Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ioirq13::R`](R) reader structure"]
impl crate::Readable for Ioirq13Spec {}
#[doc = "`reset()` method sets ioirq13 to value 0"]
impl crate::Resettable for Ioirq13Spec {
    const RESET_VALUE: u32 = 0;
}
