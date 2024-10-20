#[doc = "Register `tc_status` reader"]
pub type R = crate::R<TcStatusSpec>;
#[doc = "Field `tc_status` reader - Status of the terminal count interrupt prior to masking."]
pub type TcStatusR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Status of the terminal count interrupt prior to masking."]
    #[inline(always)]
    pub fn tc_status(&self) -> TcStatusR {
        TcStatusR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Raw Interrupt Terminal Count Status Register - indicates the DMA channels that are requesting a transfer complete, terminal count interrupt, prior to masking. A HIGH bit indicates that the terminal count interrupt request is active prior to masking.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tc_status::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
