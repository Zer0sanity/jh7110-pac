#[doc = "Register `stg_syscfg_31` reader"]
pub type R = crate::R<StgSyscfg31Spec>;
#[doc = "Register `stg_syscfg_31` writer"]
pub type W = crate::W<StgSyscfg31Spec>;
#[doc = "Field `u0_pcie_axi4_mst0_awuser_42_32` reader - PCIE AXI4 MST0 AWUSER"]
pub type U0PcieAxi4Mst0Awuser42_32R = crate::FieldReader<u16>;
#[doc = "Field `u0_pcie_axi4_mst0_rderr` reader - PCIE AXI4 MST0 RDERR"]
pub type U0PcieAxi4Mst0RderrR = crate::FieldReader;
#[doc = "Field `u0_pcie_axi4_mst0_rderr` writer - PCIE AXI4 MST0 RDERR"]
pub type U0PcieAxi4Mst0RderrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:10 - PCIE AXI4 MST0 AWUSER"]
    #[inline(always)]
    pub fn u0_pcie_axi4_mst0_awuser_42_32(&self) -> U0PcieAxi4Mst0Awuser42_32R {
        U0PcieAxi4Mst0Awuser42_32R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 11:18 - PCIE AXI4 MST0 RDERR"]
    #[inline(always)]
    pub fn u0_pcie_axi4_mst0_rderr(&self) -> U0PcieAxi4Mst0RderrR {
        U0PcieAxi4Mst0RderrR::new(((self.bits >> 11) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 11:18 - PCIE AXI4 MST0 RDERR"]
    #[inline(always)]
    #[must_use]
    pub fn u0_pcie_axi4_mst0_rderr(&mut self) -> U0PcieAxi4Mst0RderrW<StgSyscfg31Spec> {
        U0PcieAxi4Mst0RderrW::new(self, 11)
    }
}
#[doc = "STG SYSCONSAIF SYSCFG 124\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_syscfg_31::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stg_syscfg_31::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StgSyscfg31Spec;
impl crate::RegisterSpec for StgSyscfg31Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_syscfg_31::R`](R) reader structure"]
impl crate::Readable for StgSyscfg31Spec {}
#[doc = "`write(|w| ..)` method takes [`stg_syscfg_31::W`](W) writer structure"]
impl crate::Writable for StgSyscfg31Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets stg_syscfg_31 to value 0"]
impl crate::Resettable for StgSyscfg31Spec {
    const RESET_VALUE: u32 = 0;
}
