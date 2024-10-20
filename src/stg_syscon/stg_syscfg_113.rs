#[doc = "Register `stg_syscfg_113` reader"]
pub type R = crate::R<StgSyscfg113Spec>;
#[doc = "Field `u0_pcie_test_out_223_192` reader - PCIE Test OUT (little-endian)"]
pub type U0PcieTestOut223_192R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - PCIE Test OUT (little-endian)"]
    #[inline(always)]
    pub fn u0_pcie_test_out_223_192(&self) -> U0PcieTestOut223_192R {
        U0PcieTestOut223_192R::new(self.bits)
    }
}
#[doc = "STG SYSCONSAIF SYSCFG 452\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_syscfg_113::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StgSyscfg113Spec;
impl crate::RegisterSpec for StgSyscfg113Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_syscfg_113::R`](R) reader structure"]
impl crate::Readable for StgSyscfg113Spec {}
#[doc = "`reset()` method sets stg_syscfg_113 to value 0"]
impl crate::Resettable for StgSyscfg113Spec {
    const RESET_VALUE: u32 = 0;
}
