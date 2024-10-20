#[doc = "Register `stg_syscfg_92` reader"]
pub type R = crate::R<StgSyscfg92Spec>;
#[doc = "Field `u0_pcie_test_out_bridge_63_32` reader - PCIE Test OUT Bridge (little-endian)"]
pub type U0PcieTestOutBridge63_32R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - PCIE Test OUT Bridge (little-endian)"]
    #[inline(always)]
    pub fn u0_pcie_test_out_bridge_63_32(&self) -> U0PcieTestOutBridge63_32R {
        U0PcieTestOutBridge63_32R::new(self.bits)
    }
}
#[doc = "STG SYSCONSAIF SYSCFG 368\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_syscfg_92::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StgSyscfg92Spec;
impl crate::RegisterSpec for StgSyscfg92Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_syscfg_92::R`](R) reader structure"]
impl crate::Readable for StgSyscfg92Spec {}
#[doc = "`reset()` method sets stg_syscfg_92 to value 0"]
impl crate::Resettable for StgSyscfg92Spec {
    const RESET_VALUE: u32 = 0;
}
