#[doc = "Register `stg_syscfg_37` reader"]
pub type R = crate::R<StgSyscfg37Spec>;
#[doc = "Register `stg_syscfg_37` writer"]
pub type W = crate::W<StgSyscfg37Spec>;
#[doc = "Field `u0_pcie_axi4_slv0_aratomop_127_96` reader - PCIE AXI4 ARATOMOP SLV0 (little-endian)"]
pub type U0PcieAxi4Slv0Aratomop127_96R = crate::FieldReader<u32>;
#[doc = "Field `u0_pcie_axi4_slv0_aratomop_127_96` writer - PCIE AXI4 ARATOMOP SLV0 (little-endian)"]
pub type U0PcieAxi4Slv0Aratomop127_96W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - PCIE AXI4 ARATOMOP SLV0 (little-endian)"]
    #[inline(always)]
    pub fn u0_pcie_axi4_slv0_aratomop_127_96(&self) -> U0PcieAxi4Slv0Aratomop127_96R {
        U0PcieAxi4Slv0Aratomop127_96R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - PCIE AXI4 ARATOMOP SLV0 (little-endian)"]
    #[inline(always)]
    #[must_use]
    pub fn u0_pcie_axi4_slv0_aratomop_127_96(
        &mut self,
    ) -> U0PcieAxi4Slv0Aratomop127_96W<StgSyscfg37Spec> {
        U0PcieAxi4Slv0Aratomop127_96W::new(self, 0)
    }
}
#[doc = "STG SYSCONSAIF SYSCFG 148\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_syscfg_37::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stg_syscfg_37::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StgSyscfg37Spec;
impl crate::RegisterSpec for StgSyscfg37Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_syscfg_37::R`](R) reader structure"]
impl crate::Readable for StgSyscfg37Spec {}
#[doc = "`write(|w| ..)` method takes [`stg_syscfg_37::W`](W) writer structure"]
impl crate::Writable for StgSyscfg37Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets stg_syscfg_37 to value 0"]
impl crate::Resettable for StgSyscfg37Spec {
    const RESET_VALUE: u32 = 0;
}
