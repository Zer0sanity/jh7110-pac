#[doc = "Register `stg_syscfg_197` reader"]
pub type R = crate::R<StgSyscfg197Spec>;
#[doc = "Register `stg_syscfg_197` writer"]
pub type W = crate::W<StgSyscfg197Spec>;
#[doc = "Field `u1_pcie_test_in_31_0` reader - PCIE Test IN (little-endian)"]
pub type U1PcieTestIn31_0R = crate::FieldReader<u32>;
#[doc = "Field `u1_pcie_test_in_31_0` writer - PCIE Test IN (little-endian)"]
pub type U1PcieTestIn31_0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - PCIE Test IN (little-endian)"]
    #[inline(always)]
    pub fn u1_pcie_test_in_31_0(&self) -> U1PcieTestIn31_0R {
        U1PcieTestIn31_0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - PCIE Test IN (little-endian)"]
    #[inline(always)]
    #[must_use]
    pub fn u1_pcie_test_in_31_0(&mut self) -> U1PcieTestIn31_0W<StgSyscfg197Spec> {
        U1PcieTestIn31_0W::new(self, 0)
    }
}
#[doc = "STG SYSCONSAIF SYSCFG 788\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_syscfg_197::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stg_syscfg_197::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StgSyscfg197Spec;
impl crate::RegisterSpec for StgSyscfg197Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_syscfg_197::R`](R) reader structure"]
impl crate::Readable for StgSyscfg197Spec {}
#[doc = "`write(|w| ..)` method takes [`stg_syscfg_197::W`](W) writer structure"]
impl crate::Writable for StgSyscfg197Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets stg_syscfg_197 to value 0"]
impl crate::Resettable for StgSyscfg197Spec {
    const RESET_VALUE: u32 = 0;
}
