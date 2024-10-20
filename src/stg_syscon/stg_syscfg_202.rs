#[doc = "Register `stg_syscfg_202` reader"]
pub type R = crate::R<StgSyscfg202Spec>;
#[doc = "Field `u1_pcie_test_out_bridge_127_96` reader - PCIE Test OUT Bridge (little-endian)"]
pub type U1PcieTestOutBridge127_96R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - PCIE Test OUT Bridge (little-endian)"]
    #[inline(always)]
    pub fn u1_pcie_test_out_bridge_127_96(&self) -> U1PcieTestOutBridge127_96R {
        U1PcieTestOutBridge127_96R::new(self.bits)
    }
}
#[doc = "STG SYSCONSAIF SYSCFG 808\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_syscfg_202::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StgSyscfg202Spec;
impl crate::RegisterSpec for StgSyscfg202Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_syscfg_202::R`](R) reader structure"]
impl crate::Readable for StgSyscfg202Spec {}
#[doc = "`reset()` method sets stg_syscfg_202 to value 0"]
impl crate::Resettable for StgSyscfg202Spec {
    const RESET_VALUE: u32 = 0;
}
