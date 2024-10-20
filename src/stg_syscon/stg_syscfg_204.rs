#[doc = "Register `stg_syscfg_204` reader"]
pub type R = crate::R<StgSyscfg204Spec>;
#[doc = "Field `u1_pcie_test_out_bridge_191_160` reader - PCIE Test OUT Bridge (little-endian)"]
pub type U1PcieTestOutBridge191_160R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - PCIE Test OUT Bridge (little-endian)"]
    #[inline(always)]
    pub fn u1_pcie_test_out_bridge_191_160(&self) -> U1PcieTestOutBridge191_160R {
        U1PcieTestOutBridge191_160R::new(self.bits)
    }
}
#[doc = "STG SYSCONSAIF SYSCFG 816\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_syscfg_204::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StgSyscfg204Spec;
impl crate::RegisterSpec for StgSyscfg204Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_syscfg_204::R`](R) reader structure"]
impl crate::Readable for StgSyscfg204Spec {}
#[doc = "`reset()` method sets stg_syscfg_204 to value 0"]
impl crate::Resettable for StgSyscfg204Spec {
    const RESET_VALUE: u32 = 0;
}
