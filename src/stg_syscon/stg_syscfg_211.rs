#[doc = "Register `stg_syscfg_211` reader"]
pub type R = crate::R<StgSyscfg211Spec>;
#[doc = "Field `u1_pcie_test_out_bridge_415_384` reader - PCIE Test OUT Bridge (little-endian)"]
pub type U1PcieTestOutBridge415_384R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - PCIE Test OUT Bridge (little-endian)"]
    #[inline(always)]
    pub fn u1_pcie_test_out_bridge_415_384(&self) -> U1PcieTestOutBridge415_384R {
        U1PcieTestOutBridge415_384R::new(self.bits)
    }
}
#[doc = "STG SYSCONSAIF SYSCFG 844\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_syscfg_211::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StgSyscfg211Spec;
impl crate::RegisterSpec for StgSyscfg211Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_syscfg_211::R`](R) reader structure"]
impl crate::Readable for StgSyscfg211Spec {}
#[doc = "`reset()` method sets stg_syscfg_211 to value 0"]
impl crate::Resettable for StgSyscfg211Spec {
    const RESET_VALUE: u32 = 0;
}
