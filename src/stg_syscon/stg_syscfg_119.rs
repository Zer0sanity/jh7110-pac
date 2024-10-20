#[doc = "Register `stg_syscfg_119` reader"]
pub type R = crate::R<StgSyscfg119Spec>;
#[doc = "Field `u0_pcie_test_out_415_384` reader - PCIE Test OUT (little-endian)"]
pub type U0PcieTestOut415_384R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - PCIE Test OUT (little-endian)"]
    #[inline(always)]
    pub fn u0_pcie_test_out_415_384(&self) -> U0PcieTestOut415_384R {
        U0PcieTestOut415_384R::new(self.bits)
    }
}
#[doc = "STG SYSCONSAIF SYSCFG 476\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_syscfg_119::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StgSyscfg119Spec;
impl crate::RegisterSpec for StgSyscfg119Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_syscfg_119::R`](R) reader structure"]
impl crate::Readable for StgSyscfg119Spec {}
#[doc = "`reset()` method sets stg_syscfg_119 to value 0"]
impl crate::Resettable for StgSyscfg119Spec {
    const RESET_VALUE: u32 = 0;
}
