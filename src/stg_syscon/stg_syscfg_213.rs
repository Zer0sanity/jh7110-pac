#[doc = "Register `stg_syscfg_213` reader"]
pub type R = crate::R<StgSyscfg213Spec>;
#[doc = "Field `u1_pcie_test_out_bridge_479_448` reader - PCIE Test OUT Bridge (little-endian)"]
pub type U1PcieTestOutBridge479_448R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - PCIE Test OUT Bridge (little-endian)"]
    #[inline(always)]
    pub fn u1_pcie_test_out_bridge_479_448(&self) -> U1PcieTestOutBridge479_448R {
        U1PcieTestOutBridge479_448R::new(self.bits)
    }
}
#[doc = "STG SYSCONSAIF SYSCFG 852\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_syscfg_213::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StgSyscfg213Spec;
impl crate::RegisterSpec for StgSyscfg213Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_syscfg_213::R`](R) reader structure"]
impl crate::Readable for StgSyscfg213Spec {}
#[doc = "`reset()` method sets stg_syscfg_213 to value 0"]
impl crate::Resettable for StgSyscfg213Spec {
    const RESET_VALUE: u32 = 0;
}
