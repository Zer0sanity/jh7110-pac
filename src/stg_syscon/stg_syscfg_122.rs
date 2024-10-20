#[doc = "Register `stg_syscfg_122` reader"]
pub type R = crate::R<StgSyscfg122Spec>;
#[doc = "Field `u0_pcie_test_out_511_480` reader - PCIE Test OUT (little-endian)"]
pub type U0PcieTestOut511_480R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - PCIE Test OUT (little-endian)"]
    #[inline(always)]
    pub fn u0_pcie_test_out_511_480(&self) -> U0PcieTestOut511_480R {
        U0PcieTestOut511_480R::new(self.bits)
    }
}
#[doc = "STG SYSCONSAIF SYSCFG 488\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_syscfg_122::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StgSyscfg122Spec;
impl crate::RegisterSpec for StgSyscfg122Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_syscfg_122::R`](R) reader structure"]
impl crate::Readable for StgSyscfg122Spec {}
#[doc = "`reset()` method sets stg_syscfg_122 to value 0"]
impl crate::Resettable for StgSyscfg122Spec {
    const RESET_VALUE: u32 = 0;
}
