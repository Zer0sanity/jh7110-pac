#[doc = "Register `stg_syscfg_29` reader"]
pub type R = crate::R<StgSyscfg29Spec>;
#[doc = "Field `u0_pcie_axi4_mst0_awfunc` reader - "]
pub type U0PcieAxi4Mst0AwfuncR = crate::FieldReader<u16>;
#[doc = "Field `u0_pcie_axi4_mst0_awregion` reader - "]
pub type U0PcieAxi4Mst0AwregionR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:14"]
    #[inline(always)]
    pub fn u0_pcie_axi4_mst0_awfunc(&self) -> U0PcieAxi4Mst0AwfuncR {
        U0PcieAxi4Mst0AwfuncR::new((self.bits & 0x7fff) as u16)
    }
    #[doc = "Bits 15:18"]
    #[inline(always)]
    pub fn u0_pcie_axi4_mst0_awregion(&self) -> U0PcieAxi4Mst0AwregionR {
        U0PcieAxi4Mst0AwregionR::new(((self.bits >> 15) & 0x0f) as u8)
    }
}
#[doc = "STG SYSCONSAIF SYSCFG 116\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_syscfg_29::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StgSyscfg29Spec;
impl crate::RegisterSpec for StgSyscfg29Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_syscfg_29::R`](R) reader structure"]
impl crate::Readable for StgSyscfg29Spec {}
#[doc = "`reset()` method sets stg_syscfg_29 to value 0"]
impl crate::Resettable for StgSyscfg29Spec {
    const RESET_VALUE: u32 = 0;
}
