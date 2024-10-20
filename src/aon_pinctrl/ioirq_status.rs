#[doc = "Register `ioirq_status[%s]` reader"]
pub type R = crate::R<IoirqStatusSpec>;
#[doc = "Field `ioirq` reader - ris - Status of the edge trigger, can be cleared by writing gpioic. mis - The masked GPIO IRQ status. in_sync2 - Status of gpio_in after synchronization."]
pub type IoirqR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - ris - Status of the edge trigger, can be cleared by writing gpioic. mis - The masked GPIO IRQ status. in_sync2 - Status of gpio_in after synchronization."]
    #[inline(always)]
    pub fn ioirq(&self) -> IoirqR {
        IoirqR::new((self.bits & 0x0f) as u8)
    }
}
#[doc = "Always-on GPIO IO IRQ configuration.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ioirq_status::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IoirqStatusSpec;
impl crate::RegisterSpec for IoirqStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ioirq_status::R`](R) reader structure"]
impl crate::Readable for IoirqStatusSpec {}
#[doc = "`reset()` method sets ioirq_status[%s]
to value 0"]
impl crate::Resettable for IoirqStatusSpec {
    const RESET_VALUE: u32 = 0;
}
