#[doc = "Register `error_status` reader"]
pub type R = crate::R<ErrorStatusSpec>;
#[doc = "Field `error_status` reader - Status of the error interrupt prior to masking."]
pub type ErrorStatusR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Status of the error interrupt prior to masking."]
    #[inline(always)]
    pub fn error_status(&self) -> ErrorStatusR {
        ErrorStatusR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Raw Error Interrupt Status Register - indicates the DMA channels that are requesting an error interrupt prior to masking. A HIGH bit indicates that the error interrupt request is active prior to masking.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`error_status::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ErrorStatusSpec;
impl crate::RegisterSpec for ErrorStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`error_status::R`](R) reader structure"]
impl crate::Readable for ErrorStatusSpec {}
#[doc = "`reset()` method sets error_status to value 0"]
impl crate::Resettable for ErrorStatusSpec {
    const RESET_VALUE: u32 = 0;
}
