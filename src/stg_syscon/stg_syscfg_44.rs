#[doc = "Register `stg_syscfg_44` reader"]
pub type R = crate::R<StgSyscfg44Spec>;
#[doc = "Register `stg_syscfg_44` writer"]
pub type W = crate::W<StgSyscfg44Spec>;
#[doc = "Field `u0_pcie_axi4_slv0_aruser_40_32` reader - PCIE AXI4 SLV0 ARUSER (little-endian)"]
pub type U0PcieAxi4Slv0Aruser40_32R = crate::FieldReader<u16>;
#[doc = "Field `u0_pcie_axi4_slv0_aruser_40_32` writer - PCIE AXI4 SLV0 ARUSER (little-endian)"]
pub type U0PcieAxi4Slv0Aruser40_32W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `u0_pcie_axi4_slv0_awfunc` reader - PCIE AXI SLV0 AWFUNC"]
pub type U0PcieAxi4Slv0AwfuncR = crate::FieldReader<u16>;
#[doc = "Field `u0_pcie_axi4_slv0_awfunc` writer - PCIE AXI SLV0 AWFUNC"]
pub type U0PcieAxi4Slv0AwfuncW<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
#[doc = "Field `u0_pcie_axi4_slv0_awregion` reader - PCIE AXI4 SLV0 AWREGION"]
pub type U0PcieAxi4Slv0AwregionR = crate::FieldReader;
#[doc = "Field `u0_pcie_axi4_slv0_awregion` writer - PCIE AXI4 SLV0 AWREGION"]
pub type U0PcieAxi4Slv0AwregionW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:8 - PCIE AXI4 SLV0 ARUSER (little-endian)"]
    #[inline(always)]
    pub fn u0_pcie_axi4_slv0_aruser_40_32(&self) -> U0PcieAxi4Slv0Aruser40_32R {
        U0PcieAxi4Slv0Aruser40_32R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 9:23 - PCIE AXI SLV0 AWFUNC"]
    #[inline(always)]
    pub fn u0_pcie_axi4_slv0_awfunc(&self) -> U0PcieAxi4Slv0AwfuncR {
        U0PcieAxi4Slv0AwfuncR::new(((self.bits >> 9) & 0x7fff) as u16)
    }
    #[doc = "Bits 24:27 - PCIE AXI4 SLV0 AWREGION"]
    #[inline(always)]
    pub fn u0_pcie_axi4_slv0_awregion(&self) -> U0PcieAxi4Slv0AwregionR {
        U0PcieAxi4Slv0AwregionR::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:8 - PCIE AXI4 SLV0 ARUSER (little-endian)"]
    #[inline(always)]
    #[must_use]
    pub fn u0_pcie_axi4_slv0_aruser_40_32(
        &mut self,
    ) -> U0PcieAxi4Slv0Aruser40_32W<StgSyscfg44Spec> {
        U0PcieAxi4Slv0Aruser40_32W::new(self, 0)
    }
    #[doc = "Bits 9:23 - PCIE AXI SLV0 AWFUNC"]
    #[inline(always)]
    #[must_use]
    pub fn u0_pcie_axi4_slv0_awfunc(&mut self) -> U0PcieAxi4Slv0AwfuncW<StgSyscfg44Spec> {
        U0PcieAxi4Slv0AwfuncW::new(self, 9)
    }
    #[doc = "Bits 24:27 - PCIE AXI4 SLV0 AWREGION"]
    #[inline(always)]
    #[must_use]
    pub fn u0_pcie_axi4_slv0_awregion(&mut self) -> U0PcieAxi4Slv0AwregionW<StgSyscfg44Spec> {
        U0PcieAxi4Slv0AwregionW::new(self, 24)
    }
}
#[doc = "STG SYSCONSAIF SYSCFG 176\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_syscfg_44::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stg_syscfg_44::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StgSyscfg44Spec;
impl crate::RegisterSpec for StgSyscfg44Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_syscfg_44::R`](R) reader structure"]
impl crate::Readable for StgSyscfg44Spec {}
#[doc = "`write(|w| ..)` method takes [`stg_syscfg_44::W`](W) writer structure"]
impl crate::Writable for StgSyscfg44Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets stg_syscfg_44 to value 0"]
impl crate::Resettable for StgSyscfg44Spec {
    const RESET_VALUE: u32 = 0;
}
