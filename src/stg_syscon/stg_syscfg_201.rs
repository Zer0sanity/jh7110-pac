#[doc = "Register `stg_syscfg_201` reader"]
pub type R = crate::R<StgSyscfg201Spec>;
#[doc = "Field `u1_pcie_test_out_bridge_95_64` reader - PCIE Test OUT Bridge (little-endian)"]
pub type U1PcieTestOutBridge95_64R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - PCIE Test OUT Bridge (little-endian)"]
    #[inline(always)]
    pub fn u1_pcie_test_out_bridge_95_64(&self) -> U1PcieTestOutBridge95_64R {
        U1PcieTestOutBridge95_64R::new(self.bits)
    }
}
#[doc = "STG SYSCONSAIF SYSCFG 804\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_syscfg_201::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StgSyscfg201Spec;
impl crate::RegisterSpec for StgSyscfg201Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_syscfg_201::R`](R) reader structure"]
impl crate::Readable for StgSyscfg201Spec {}
#[doc = "`reset()` method sets stg_syscfg_201 to value 0"]
impl crate::Resettable for StgSyscfg201Spec {
    const RESET_VALUE: u32 = 0;
}
