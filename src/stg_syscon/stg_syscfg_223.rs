#[doc = "Register `stg_syscfg_223` reader"]
pub type R = crate::R<StgSyscfg223Spec>;
#[doc = "Field `u1_pcie_test_out_287_256` reader - PCIE Test OUT (little-endian)"]
pub type U1PcieTestOut287_256R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - PCIE Test OUT (little-endian)"]
    #[inline(always)]
    pub fn u1_pcie_test_out_287_256(&self) -> U1PcieTestOut287_256R {
        U1PcieTestOut287_256R::new(self.bits)
    }
}
#[doc = "STG SYSCONSAIF SYSCFG 892\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_syscfg_223::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StgSyscfg223Spec;
impl crate::RegisterSpec for StgSyscfg223Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_syscfg_223::R`](R) reader structure"]
impl crate::Readable for StgSyscfg223Spec {}
#[doc = "`reset()` method sets stg_syscfg_223 to value 0"]
impl crate::Resettable for StgSyscfg223Spec {
    const RESET_VALUE: u32 = 0;
}
