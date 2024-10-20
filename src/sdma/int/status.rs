#[doc = "Register `status` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "Field `status` reader - Status of the DMA interrupts after masking"]
pub type StatusR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Status of the DMA interrupts after masking"]
    #[inline(always)]
    pub fn status(&self) -> StatusR {
        StatusR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Interrupt Status Register - shows the status of the interrupts after masking. A HIGH bit indicates that a specific DMA channel interrupt request is active. You can generate the request from either the error or terminal count interrupt requests.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusSpec;
impl crate::RegisterSpec for StatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for StatusSpec {}
#[doc = "`reset()` method sets status to value 0"]
impl crate::Resettable for StatusSpec {
    const RESET_VALUE: u32 = 0;
}
