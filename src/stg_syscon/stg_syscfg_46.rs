#[doc = "Register `stg_syscfg_46` reader"]
pub type R = crate::R<StgSyscfg46Spec>;
#[doc = "Register `stg_syscfg_46` writer"]
pub type W = crate::W<StgSyscfg46Spec>;
#[doc = "Field `u0_pcie_axi4_slv0_awuser_40_32` reader - PCIE AXI4 SLV0 AWUSER (little-endian)"]
pub type U0PcieAxi4Slv0Awuser40_32R = crate::FieldReader<u16>;
#[doc = "Field `u0_pcie_axi4_slv0_awuser_40_32` writer - PCIE AXI4 SLV0 AWUSER (little-endian)"]
pub type U0PcieAxi4Slv0Awuser40_32W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `u0_pcie_axi4_slv0_rderr` reader - PCIE AXI4 SLV0 RDERR"]
pub type U0PcieAxi4Slv0RderrR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:8 - PCIE AXI4 SLV0 AWUSER (little-endian)"]
    #[inline(always)]
    pub fn u0_pcie_axi4_slv0_awuser_40_32(&self) -> U0PcieAxi4Slv0Awuser40_32R {
        U0PcieAxi4Slv0Awuser40_32R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 9:16 - PCIE AXI4 SLV0 RDERR"]
    #[inline(always)]
    pub fn u0_pcie_axi4_slv0_rderr(&self) -> U0PcieAxi4Slv0RderrR {
        U0PcieAxi4Slv0RderrR::new(((self.bits >> 9) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:8 - PCIE AXI4 SLV0 AWUSER (little-endian)"]
    #[inline(always)]
    #[must_use]
    pub fn u0_pcie_axi4_slv0_awuser_40_32(
        &mut self,
    ) -> U0PcieAxi4Slv0Awuser40_32W<StgSyscfg46Spec> {
        U0PcieAxi4Slv0Awuser40_32W::new(self, 0)
    }
}
#[doc = "STG SYSCONSAIF SYSCFG 184\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_syscfg_46::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stg_syscfg_46::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StgSyscfg46Spec;
impl crate::RegisterSpec for StgSyscfg46Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_syscfg_46::R`](R) reader structure"]
impl crate::Readable for StgSyscfg46Spec {}
#[doc = "`write(|w| ..)` method takes [`stg_syscfg_46::W`](W) writer structure"]
impl crate::Writable for StgSyscfg46Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets stg_syscfg_46 to value 0"]
impl crate::Resettable for StgSyscfg46Spec {
    const RESET_VALUE: u32 = 0;
}
