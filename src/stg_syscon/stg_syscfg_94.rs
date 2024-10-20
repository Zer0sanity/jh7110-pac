#[doc = "Register `stg_syscfg_94` reader"]
pub type R = crate::R<StgSyscfg94Spec>;
#[doc = "Field `u0_pcie_test_out_bridge_127_96` reader - PCIE Test OUT Bridge (little-endian)"]
pub type U0PcieTestOutBridge127_96R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - PCIE Test OUT Bridge (little-endian)"]
    #[inline(always)]
    pub fn u0_pcie_test_out_bridge_127_96(&self) -> U0PcieTestOutBridge127_96R {
        U0PcieTestOutBridge127_96R::new(self.bits)
    }
}
#[doc = "STG SYSCONSAIF SYSCFG 376\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_syscfg_94::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StgSyscfg94Spec;
impl crate::RegisterSpec for StgSyscfg94Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_syscfg_94::R`](R) reader structure"]
impl crate::Readable for StgSyscfg94Spec {}
#[doc = "`reset()` method sets stg_syscfg_94 to value 0"]
impl crate::Resettable for StgSyscfg94Spec {
    const RESET_VALUE: u32 = 0;
}