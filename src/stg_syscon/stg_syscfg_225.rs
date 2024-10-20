#[doc = "Register `stg_syscfg_225` reader"]
pub type R = crate::R<StgSyscfg225Spec>;
#[doc = "Field `u1_pcie_test_out_351_320` reader - PCIE Test OUT (little-endian)"]
pub type U1PcieTestOut351_320R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - PCIE Test OUT (little-endian)"]
    #[inline(always)]
    pub fn u1_pcie_test_out_351_320(&self) -> U1PcieTestOut351_320R {
        U1PcieTestOut351_320R::new(self.bits)
    }
}
#[doc = "STG SYSCONSAIF SYSCFG 900\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_syscfg_225::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StgSyscfg225Spec;
impl crate::RegisterSpec for StgSyscfg225Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_syscfg_225::R`](R) reader structure"]
impl crate::Readable for StgSyscfg225Spec {}
#[doc = "`reset()` method sets stg_syscfg_225 to value 0"]
impl crate::Resettable for StgSyscfg225Spec {
    const RESET_VALUE: u32 = 0;
}
