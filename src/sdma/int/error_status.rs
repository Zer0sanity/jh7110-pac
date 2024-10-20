#[doc = "Register `error_status` reader"]
pub type R = crate::R<ErrorStatusSpec>;
#[doc = "Field `error_status` reader - Interrupt error status."]
pub type ErrorStatusR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Interrupt error status."]
    #[inline(always)]
    pub fn error_status(&self) -> ErrorStatusR {
        ErrorStatusR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Interrupt Error Status Register - indicates the status of the error request after masking. You must use this register in conjunction with the DMACIntStatus Register if you use the combined interrupt request, DMACINTR, to request interrupts. If you use the DMACINTERR interrupt request, then only read the DMACIntErrorStatus Register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`error_status::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
