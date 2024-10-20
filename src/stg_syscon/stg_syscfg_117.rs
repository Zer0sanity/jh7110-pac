#[doc = "Register `stg_syscfg_117` reader"]
pub type R = crate::R<StgSyscfg117Spec>;
#[doc = "Field `u0_pcie_test_out_351_320` reader - PCIE Test OUT (little-endian)"]
pub type U0PcieTestOut351_320R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - PCIE Test OUT (little-endian)"]
    #[inline(always)]
    pub fn u0_pcie_test_out_351_320(&self) -> U0PcieTestOut351_320R {
        U0PcieTestOut351_320R::new(self.bits)
    }
}
#[doc = "STG SYSCONSAIF SYSCFG 468\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_syscfg_117::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StgSyscfg117Spec;
impl crate::RegisterSpec for StgSyscfg117Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_syscfg_117::R`](R) reader structure"]
impl crate::Readable for StgSyscfg117Spec {}
#[doc = "`reset()` method sets stg_syscfg_117 to value 0"]
impl crate::Resettable for StgSyscfg117Spec {
    const RESET_VALUE: u32 = 0;
}
