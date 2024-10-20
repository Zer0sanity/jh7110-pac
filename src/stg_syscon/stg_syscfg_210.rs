#[doc = "Register `stg_syscfg_210` reader"]
pub type R = crate::R<StgSyscfg210Spec>;
#[doc = "Field `u1_pcie_test_out_bridge_383_352` reader - PCIE Test OUT Bridge (little-endian)"]
pub type U1PcieTestOutBridge383_352R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - PCIE Test OUT Bridge (little-endian)"]
    #[inline(always)]
    pub fn u1_pcie_test_out_bridge_383_352(&self) -> U1PcieTestOutBridge383_352R {
        U1PcieTestOutBridge383_352R::new(self.bits)
    }
}
#[doc = "STG SYSCONSAIF SYSCFG 840\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_syscfg_210::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StgSyscfg210Spec;
impl crate::RegisterSpec for StgSyscfg210Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_syscfg_210::R`](R) reader structure"]
impl crate::Readable for StgSyscfg210Spec {}
#[doc = "`reset()` method sets stg_syscfg_210 to value 0"]
impl crate::Resettable for StgSyscfg210Spec {
    const RESET_VALUE: u32 = 0;
}
