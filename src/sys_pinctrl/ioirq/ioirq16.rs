#[doc = "Register `ioirq16` reader"]
pub type R = crate::R<Ioirq16Spec>;
#[doc = "Field `in_sync2_1` reader - Status of the gpio_in after synchronization"]
pub type InSync2_1R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Status of the gpio_in after synchronization"]
    #[inline(always)]
    pub fn in_sync2_1(&self) -> InSync2_1R {
        InSync2_1R::new(self.bits)
    }
}
#[doc = "SYS IOMUX CFGSAIF SYSCFG IOIRQ 64: GPIO Synchronization Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ioirq16::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ioirq16Spec;
impl crate::RegisterSpec for Ioirq16Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ioirq16::R`](R) reader structure"]
impl crate::Readable for Ioirq16Spec {}
#[doc = "`reset()` method sets ioirq16 to value 0"]
impl crate::Resettable for Ioirq16Spec {
    const RESET_VALUE: u32 = 0;
}
