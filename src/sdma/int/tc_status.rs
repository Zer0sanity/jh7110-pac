#[doc = "Register `tc_status` reader"]
pub type R = crate::R<TcStatusSpec>;
#[doc = "Field `tc_status` reader - Interrupt terminal count request status"]
pub type TcStatusR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Interrupt terminal count request status"]
    #[inline(always)]
    pub fn tc_status(&self) -> TcStatusR {
        TcStatusR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Interrupt Terminal Count Status Register - indicates the status of the terminal count after masking. You must use this register in conjunction with the DMACIntStatus Register if you use the combined interrupt request, DMACINTR, to request interrupts. If you use the DMACINTTC interrupt request, then you only have to read the DMACIntTCStatus Register to ascertain the source of the interrupt request.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tc_status::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TcStatusSpec;
impl crate::RegisterSpec for TcStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tc_status::R`](R) reader structure"]
impl crate::Readable for TcStatusSpec {}
#[doc = "`reset()` method sets tc_status to value 0"]
impl crate::Resettable for TcStatusSpec {
    const RESET_VALUE: u32 = 0;
}
