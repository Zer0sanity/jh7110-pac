#[doc = "Register `stg_syscfg_63` reader"]
pub type R = crate::R<StgSyscfg63Spec>;
#[doc = "Register `stg_syscfg_63` writer"]
pub type W = crate::W<StgSyscfg63Spec>;
#[doc = "Field `u0_pcie_k_phyparam_447_416` reader - PCIE PHY Parameter (little-endian)"]
pub type U0PcieKPhyparam447_416R = crate::FieldReader<u32>;
#[doc = "Field `u0_pcie_k_phyparam_447_416` writer - PCIE PHY Parameter (little-endian)"]
pub type U0PcieKPhyparam447_416W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - PCIE PHY Parameter (little-endian)"]
    #[inline(always)]
    pub fn u0_pcie_k_phyparam_447_416(&self) -> U0PcieKPhyparam447_416R {
        U0PcieKPhyparam447_416R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - PCIE PHY Parameter (little-endian)"]
    #[inline(always)]
    #[must_use]
    pub fn u0_pcie_k_phyparam_447_416(&mut self) -> U0PcieKPhyparam447_416W<StgSyscfg63Spec> {
        U0PcieKPhyparam447_416W::new(self, 0)
    }
}
#[doc = "STG SYSCONSAIF SYSCFG 252\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_syscfg_63::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stg_syscfg_63::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StgSyscfg63Spec;
impl crate::RegisterSpec for StgSyscfg63Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_syscfg_63::R`](R) reader structure"]
impl crate::Readable for StgSyscfg63Spec {}
#[doc = "`write(|w| ..)` method takes [`stg_syscfg_63::W`](W) writer structure"]
impl crate::Writable for StgSyscfg63Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets stg_syscfg_63 to value 0"]
impl crate::Resettable for StgSyscfg63Spec {
    const RESET_VALUE: u32 = 0;
}
