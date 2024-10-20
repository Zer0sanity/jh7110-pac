#[doc = "Register `stg_syscfg_227` reader"]
pub type R = crate::R<StgSyscfg227Spec>;
#[doc = "Field `u1_pcie_test_out_415_384` reader - PCIE Test OUT (little-endian)"]
pub type U1PcieTestOut415_384R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - PCIE Test OUT (little-endian)"]
    #[inline(always)]
    pub fn u1_pcie_test_out_415_384(&self) -> U1PcieTestOut415_384R {
        U1PcieTestOut415_384R::new(self.bits)
    }
}
#[doc = "STG SYSCONSAIF SYSCFG 908\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_syscfg_227::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StgSyscfg227Spec;
impl crate::RegisterSpec for StgSyscfg227Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_syscfg_227::R`](R) reader structure"]
impl crate::Readable for StgSyscfg227Spec {}
#[doc = "`reset()` method sets stg_syscfg_227 to value 0"]
impl crate::Resettable for StgSyscfg227Spec {
    const RESET_VALUE: u32 = 0;
}
