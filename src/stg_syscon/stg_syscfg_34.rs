#[doc = "Register `stg_syscfg_34` reader"]
pub type R = crate::R<StgSyscfg34Spec>;
#[doc = "Register `stg_syscfg_34` writer"]
pub type W = crate::W<StgSyscfg34Spec>;
#[doc = "Field `u0_pcie_axi4_slv0_aratomop_31_0` reader - PCIE AXI4 ARATOMOP SLV0 (little-endian)"]
pub type U0PcieAxi4Slv0Aratomop31_0R = crate::FieldReader<u32>;
#[doc = "Field `u0_pcie_axi4_slv0_aratomop_31_0` writer - PCIE AXI4 ARATOMOP SLV0 (little-endian)"]
pub type U0PcieAxi4Slv0Aratomop31_0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - PCIE AXI4 ARATOMOP SLV0 (little-endian)"]
    #[inline(always)]
    pub fn u0_pcie_axi4_slv0_aratomop_31_0(&self) -> U0PcieAxi4Slv0Aratomop31_0R {
        U0PcieAxi4Slv0Aratomop31_0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - PCIE AXI4 ARATOMOP SLV0 (little-endian)"]
    #[inline(always)]
    #[must_use]
    pub fn u0_pcie_axi4_slv0_aratomop_31_0(
        &mut self,
    ) -> U0PcieAxi4Slv0Aratomop31_0W<StgSyscfg34Spec> {
        U0PcieAxi4Slv0Aratomop31_0W::new(self, 0)
    }
}
#[doc = "STG SYSCONSAIF SYSCFG 136\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_syscfg_34::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stg_syscfg_34::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StgSyscfg34Spec;
impl crate::RegisterSpec for StgSyscfg34Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_syscfg_34::R`](R) reader structure"]
impl crate::Readable for StgSyscfg34Spec {}
#[doc = "`write(|w| ..)` method takes [`stg_syscfg_34::W`](W) writer structure"]
impl crate::Writable for StgSyscfg34Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets stg_syscfg_34 to value 0"]
impl crate::Resettable for StgSyscfg34Spec {
    const RESET_VALUE: u32 = 0;
}
