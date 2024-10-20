#[doc = "Register `stg_syscfg_32` reader"]
pub type R = crate::R<StgSyscfg32Spec>;
#[doc = "Register `stg_syscfg_32` writer"]
pub type W = crate::W<StgSyscfg32Spec>;
#[doc = "Field `u0_pcie_axi4_mst0_ruser` reader - PCIE AXI4 RUSER"]
pub type U0PcieAxi4Mst0RuserR = crate::FieldReader<u32>;
#[doc = "Field `u0_pcie_axi4_mst0_ruser` writer - PCIE AXI4 RUSER"]
pub type U0PcieAxi4Mst0RuserW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - PCIE AXI4 RUSER"]
    #[inline(always)]
    pub fn u0_pcie_axi4_mst0_ruser(&self) -> U0PcieAxi4Mst0RuserR {
        U0PcieAxi4Mst0RuserR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - PCIE AXI4 RUSER"]
    #[inline(always)]
    #[must_use]
    pub fn u0_pcie_axi4_mst0_ruser(&mut self) -> U0PcieAxi4Mst0RuserW<StgSyscfg32Spec> {
        U0PcieAxi4Mst0RuserW::new(self, 0)
    }
}
#[doc = "STG SYSCONSAIF SYSCFG 128\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_syscfg_32::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stg_syscfg_32::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StgSyscfg32Spec;
impl crate::RegisterSpec for StgSyscfg32Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_syscfg_32::R`](R) reader structure"]
impl crate::Readable for StgSyscfg32Spec {}
#[doc = "`write(|w| ..)` method takes [`stg_syscfg_32::W`](W) writer structure"]
impl crate::Writable for StgSyscfg32Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets stg_syscfg_32 to value 0"]
impl crate::Resettable for StgSyscfg32Spec {
    const RESET_VALUE: u32 = 0;
}
