#[doc = "Register `stg_syscfg_48` reader"]
pub type R = crate::R<StgSyscfg48Spec>;
#[doc = "Register `stg_syscfg_48` writer"]
pub type W = crate::W<StgSyscfg48Spec>;
#[doc = "Field `u0_pcie_axi4_slv0_wderr` reader - PCIE AXI4 SLV0 WDERR"]
pub type U0PcieAxi4Slv0WderrR = crate::FieldReader;
#[doc = "Field `u0_pcie_axi4_slv0_wderr` writer - PCIE AXI4 SLV0 WDERR"]
pub type U0PcieAxi4Slv0WderrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `u0_pcie_axi4_slvl_arfunc` reader - PCIE AXI4 SLV1 ARFUNC"]
pub type U0PcieAxi4SlvlArfuncR = crate::FieldReader<u16>;
#[doc = "Field `u0_pcie_axi4_slvl_arfunc` writer - PCIE AXI4 SLV1 ARFUNC"]
pub type U0PcieAxi4SlvlArfuncW<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
impl R {
    #[doc = "Bits 0:7 - PCIE AXI4 SLV0 WDERR"]
    #[inline(always)]
    pub fn u0_pcie_axi4_slv0_wderr(&self) -> U0PcieAxi4Slv0WderrR {
        U0PcieAxi4Slv0WderrR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:22 - PCIE AXI4 SLV1 ARFUNC"]
    #[inline(always)]
    pub fn u0_pcie_axi4_slvl_arfunc(&self) -> U0PcieAxi4SlvlArfuncR {
        U0PcieAxi4SlvlArfuncR::new(((self.bits >> 8) & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:7 - PCIE AXI4 SLV0 WDERR"]
    #[inline(always)]
    #[must_use]
    pub fn u0_pcie_axi4_slv0_wderr(&mut self) -> U0PcieAxi4Slv0WderrW<StgSyscfg48Spec> {
        U0PcieAxi4Slv0WderrW::new(self, 0)
    }
    #[doc = "Bits 8:22 - PCIE AXI4 SLV1 ARFUNC"]
    #[inline(always)]
    #[must_use]
    pub fn u0_pcie_axi4_slvl_arfunc(&mut self) -> U0PcieAxi4SlvlArfuncW<StgSyscfg48Spec> {
        U0PcieAxi4SlvlArfuncW::new(self, 8)
    }
}
#[doc = "STG SYSCONSAIF SYSCFG 192\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_syscfg_48::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stg_syscfg_48::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StgSyscfg48Spec;
impl crate::RegisterSpec for StgSyscfg48Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_syscfg_48::R`](R) reader structure"]
impl crate::Readable for StgSyscfg48Spec {}
#[doc = "`write(|w| ..)` method takes [`stg_syscfg_48::W`](W) writer structure"]
impl crate::Writable for StgSyscfg48Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets stg_syscfg_48 to value 0"]
impl crate::Resettable for StgSyscfg48Spec {
    const RESET_VALUE: u32 = 0;
}
