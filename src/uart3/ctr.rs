#[doc = "Register `ctr` reader"]
pub type R = crate::R<CtrSpec>;
#[doc = "Field `ctr` reader - This register contains the peripherals identification code."]
pub type CtrR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - This register contains the peripherals identification code."]
    #[inline(always)]
    pub fn ctr(&self) -> CtrR {
        CtrR::new(self.bits)
    }
}
#[doc = "Component Type Register: This register is only valid when the DW_apb_uart is configured to have additional features implemented (ADDITIONAL_FEATURES == YES). If additional features are not implemented, this register does not exist and reading from this register address returns zero.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrSpec;
impl crate::RegisterSpec for CtrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctr::R`](R) reader structure"]
impl crate::Readable for CtrSpec {}
#[doc = "`reset()` method sets ctr to value 0x4457_0110"]
impl crate::Resettable for CtrSpec {
    const RESET_VALUE: u32 = 0x4457_0110;
}
