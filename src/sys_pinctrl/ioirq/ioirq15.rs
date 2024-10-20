#[doc = "Register `ioirq15` reader"]
pub type R = crate::R<Ioirq15Spec>;
#[doc = "Field `in_sync2_0` reader - Status of the gpio_in after synchronization"]
pub type InSync2_0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Status of the gpio_in after synchronization"]
    #[inline(always)]
    pub fn in_sync2_0(&self) -> InSync2_0R {
        InSync2_0R::new(self.bits)
    }
}
#[doc = "SYS IOMUX CFGSAIF SYSCFG IOIRQ 60: GPIO Synchronization Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ioirq15::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ioirq15Spec;
impl crate::RegisterSpec for Ioirq15Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ioirq15::R`](R) reader structure"]
impl crate::Readable for Ioirq15Spec {}
#[doc = "`reset()` method sets ioirq15 to value 0"]
impl crate::Resettable for Ioirq15Spec {
    const RESET_VALUE: u32 = 0;
}
