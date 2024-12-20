#[doc = "Register `stg_syscfg_42` reader"]
pub type R = crate::R<StgSyscfg42Spec>;
#[doc = "Register `stg_syscfg_42` writer"]
pub type W = crate::W<StgSyscfg42Spec>;
#[doc = "Field `u0_pcie_axi4_slv0_aratomop_257_256` reader - PCIE AXI4 ARATOMOP SLV0 (little-endian)"]
pub type U0PcieAxi4Slv0Aratomop257_256R = crate::FieldReader;
#[doc = "Field `u0_pcie_axi4_slv0_aratomop_257_256` writer - PCIE AXI4 ARATOMOP SLV0 (little-endian)"]
pub type U0PcieAxi4Slv0Aratomop257_256W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `u0_pcie_axi4_slv0_arfunc` reader - PCIE AXI4 SLV0 ARFUNC"]
pub type U0PcieAxi4Slv0ArfuncR = crate::FieldReader<u16>;
#[doc = "Field `u0_pcie_axi4_slv0_arfunc` writer - PCIE AXI4 SLV0 ARFUNC"]
pub type U0PcieAxi4Slv0ArfuncW<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
#[doc = "Field `u0_pcie_axi4_slv0_arregion` reader - PCIE AXI4 SLV0 ARREGION"]
pub type U0PcieAxi4Slv0ArregionR = crate::FieldReader;
#[doc = "Field `u0_pcie_axi4_slv0_arregion` writer - PCIE AXI4 SLV0 ARREGION"]
pub type U0PcieAxi4Slv0ArregionW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:1 - PCIE AXI4 ARATOMOP SLV0 (little-endian)"]
    #[inline(always)]
    pub fn u0_pcie_axi4_slv0_aratomop_257_256(&self) -> U0PcieAxi4Slv0Aratomop257_256R {
        U0PcieAxi4Slv0Aratomop257_256R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:16 - PCIE AXI4 SLV0 ARFUNC"]
    #[inline(always)]
    pub fn u0_pcie_axi4_slv0_arfunc(&self) -> U0PcieAxi4Slv0ArfuncR {
        U0PcieAxi4Slv0ArfuncR::new(((self.bits >> 2) & 0x7fff) as u16)
    }
    #[doc = "Bits 17:20 - PCIE AXI4 SLV0 ARREGION"]
    #[inline(always)]
    pub fn u0_pcie_axi4_slv0_arregion(&self) -> U0PcieAxi4Slv0ArregionR {
        U0PcieAxi4Slv0ArregionR::new(((self.bits >> 17) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - PCIE AXI4 ARATOMOP SLV0 (little-endian)"]
    #[inline(always)]
    #[must_use]
    pub fn u0_pcie_axi4_slv0_aratomop_257_256(
        &mut self,
    ) -> U0PcieAxi4Slv0Aratomop257_256W<StgSyscfg42Spec> {
        U0PcieAxi4Slv0Aratomop257_256W::new(self, 0)
    }
    #[doc = "Bits 2:16 - PCIE AXI4 SLV0 ARFUNC"]
    #[inline(always)]
    #[must_use]
    pub fn u0_pcie_axi4_slv0_arfunc(&mut self) -> U0PcieAxi4Slv0ArfuncW<StgSyscfg42Spec> {
        U0PcieAxi4Slv0ArfuncW::new(self, 2)
    }
    #[doc = "Bits 17:20 - PCIE AXI4 SLV0 ARREGION"]
    #[inline(always)]
    #[must_use]
    pub fn u0_pcie_axi4_slv0_arregion(&mut self) -> U0PcieAxi4Slv0ArregionW<StgSyscfg42Spec> {
        U0PcieAxi4Slv0ArregionW::new(self, 17)
    }
}
#[doc = "STG SYSCONSAIF SYSCFG 168\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_syscfg_42::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stg_syscfg_42::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StgSyscfg42Spec;
impl crate::RegisterSpec for StgSyscfg42Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_syscfg_42::R`](R) reader structure"]
impl crate::Readable for StgSyscfg42Spec {}
#[doc = "`write(|w| ..)` method takes [`stg_syscfg_42::W`](W) writer structure"]
impl crate::Writable for StgSyscfg42Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets stg_syscfg_42 to value 0"]
impl crate::Resettable for StgSyscfg42Spec {
    const RESET_VALUE: u32 = 0;
}
