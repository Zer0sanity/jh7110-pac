#[doc = "Register `stg_syscfg_45` reader"]
pub type R = crate::R<StgSyscfg45Spec>;
#[doc = "Register `stg_syscfg_45` writer"]
pub type W = crate::W<StgSyscfg45Spec>;
#[doc = "Field `u0_pcie_axi4_slv0_awuser_31_0` reader - PCIE AXI4 SLV0 AWUSER (little-endian)"]
pub type U0PcieAxi4Slv0Awuser31_0R = crate::FieldReader<u32>;
#[doc = "Field `u0_pcie_axi4_slv0_awuser_31_0` writer - PCIE AXI4 SLV0 AWUSER (little-endian)"]
pub type U0PcieAxi4Slv0Awuser31_0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - PCIE AXI4 SLV0 AWUSER (little-endian)"]
    #[inline(always)]
    pub fn u0_pcie_axi4_slv0_awuser_31_0(&self) -> U0PcieAxi4Slv0Awuser31_0R {
        U0PcieAxi4Slv0Awuser31_0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - PCIE AXI4 SLV0 AWUSER (little-endian)"]
    #[inline(always)]
    #[must_use]
    pub fn u0_pcie_axi4_slv0_awuser_31_0(&mut self) -> U0PcieAxi4Slv0Awuser31_0W<StgSyscfg45Spec> {
        U0PcieAxi4Slv0Awuser31_0W::new(self, 0)
    }
}
#[doc = "STG SYSCONSAIF SYSCFG 180\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_syscfg_45::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stg_syscfg_45::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StgSyscfg45Spec;
impl crate::RegisterSpec for StgSyscfg45Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_syscfg_45::R`](R) reader structure"]
impl crate::Readable for StgSyscfg45Spec {}
#[doc = "`write(|w| ..)` method takes [`stg_syscfg_45::W`](W) writer structure"]
impl crate::Writable for StgSyscfg45Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets stg_syscfg_45 to value 0"]
impl crate::Resettable for StgSyscfg45Spec {
    const RESET_VALUE: u32 = 0;
}
