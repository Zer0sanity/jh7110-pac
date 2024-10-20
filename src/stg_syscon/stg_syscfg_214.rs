#[doc = "Register `stg_syscfg_214` reader"]
pub type R = crate::R<StgSyscfg214Spec>;
#[doc = "Field `u1_pcie_test_out_bridge_511_480` reader - PCIE Test OUT Bridge (little-endian)"]
pub type U1PcieTestOutBridge511_480R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - PCIE Test OUT Bridge (little-endian)"]
    #[inline(always)]
    pub fn u1_pcie_test_out_bridge_511_480(&self) -> U1PcieTestOutBridge511_480R {
        U1PcieTestOutBridge511_480R::new(self.bits)
    }
}
#[doc = "STG SYSCONSAIF SYSCFG 856\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_syscfg_214::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StgSyscfg214Spec;
impl crate::RegisterSpec for StgSyscfg214Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_syscfg_214::R`](R) reader structure"]
impl crate::Readable for StgSyscfg214Spec {}
#[doc = "`reset()` method sets stg_syscfg_214 to value 0"]
impl crate::Resettable for StgSyscfg214Spec {
    const RESET_VALUE: u32 = 0;
}
