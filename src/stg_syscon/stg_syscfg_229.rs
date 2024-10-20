#[doc = "Register `stg_syscfg_229` reader"]
pub type R = crate::R<StgSyscfg229Spec>;
#[doc = "Field `u1_pcie_test_out_479_448` reader - PCIE Test OUT (little-endian)"]
pub type U1PcieTestOut479_448R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - PCIE Test OUT (little-endian)"]
    #[inline(always)]
    pub fn u1_pcie_test_out_479_448(&self) -> U1PcieTestOut479_448R {
        U1PcieTestOut479_448R::new(self.bits)
    }
}
#[doc = "STG SYSCONSAIF SYSCFG 916\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_syscfg_229::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StgSyscfg229Spec;
impl crate::RegisterSpec for StgSyscfg229Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_syscfg_229::R`](R) reader structure"]
impl crate::Readable for StgSyscfg229Spec {}
#[doc = "`reset()` method sets stg_syscfg_229 to value 0"]
impl crate::Resettable for StgSyscfg229Spec {
    const RESET_VALUE: u32 = 0;
}
