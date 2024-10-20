#[doc = "Register `ucv` reader"]
pub type R = crate::R<UcvSpec>;
#[doc = "Field `ucv` reader - ASCII value for each number in the version, followed by *. For example 32_30_31_2A represents the version 2.01*"]
pub type UcvR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - ASCII value for each number in the version, followed by *. For example 32_30_31_2A represents the version 2.01*"]
    #[inline(always)]
    pub fn ucv(&self) -> UcvR {
        UcvR::new(self.bits)
    }
}
#[doc = "UART Component Version: This register is only valid when the DW_apb_uart is configured to have additional features implemented (ADDITIONAL_FEATURES == YES). If additional features are not implemented, this register does not exist and reading from this register address returns zero.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ucv::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UcvSpec;
impl crate::RegisterSpec for UcvSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ucv::R`](R) reader structure"]
impl crate::Readable for UcvSpec {}
#[doc = "`reset()` method sets ucv to value 0"]
impl crate::Resettable for UcvSpec {
    const RESET_VALUE: u32 = 0;
}
