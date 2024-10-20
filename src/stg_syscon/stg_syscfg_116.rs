#[doc = "Register `stg_syscfg_116` reader"]
pub type R = crate::R<StgSyscfg116Spec>;
#[doc = "Field `u0_pcie_test_out_319_288` reader - PCIE Test OUT (little-endian)"]
pub type U0PcieTestOut319_288R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - PCIE Test OUT (little-endian)"]
    #[inline(always)]
    pub fn u0_pcie_test_out_319_288(&self) -> U0PcieTestOut319_288R {
        U0PcieTestOut319_288R::new(self.bits)
    }
}
#[doc = "STG SYSCONSAIF SYSCFG 464\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_syscfg_116::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StgSyscfg116Spec;
impl crate::RegisterSpec for StgSyscfg116Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_syscfg_116::R`](R) reader structure"]
impl crate::Readable for StgSyscfg116Spec {}
#[doc = "`reset()` method sets stg_syscfg_116 to value 0"]
impl crate::Resettable for StgSyscfg116Spec {
    const RESET_VALUE: u32 = 0;
}
