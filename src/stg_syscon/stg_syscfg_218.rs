#[doc = "Register `stg_syscfg_218` reader"]
pub type R = crate::R<StgSyscfg218Spec>;
#[doc = "Field `u1_pcie_test_out_127_96` reader - PCIE Test OUT (little-endian)"]
pub type U1PcieTestOut127_96R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - PCIE Test OUT (little-endian)"]
    #[inline(always)]
    pub fn u1_pcie_test_out_127_96(&self) -> U1PcieTestOut127_96R {
        U1PcieTestOut127_96R::new(self.bits)
    }
}
#[doc = "STG SYSCONSAIF SYSCFG 872\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_syscfg_218::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StgSyscfg218Spec;
impl crate::RegisterSpec for StgSyscfg218Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_syscfg_218::R`](R) reader structure"]
impl crate::Readable for StgSyscfg218Spec {}
#[doc = "`reset()` method sets stg_syscfg_218 to value 0"]
impl crate::Resettable for StgSyscfg218Spec {
    const RESET_VALUE: u32 = 0;
}
