#[doc = "Register `stg_syscfg_170` reader"]
pub type R = crate::R<StgSyscfg170Spec>;
#[doc = "Register `stg_syscfg_170` writer"]
pub type W = crate::W<StgSyscfg170Spec>;
#[doc = "Field `u1_pcie_k_phyparam_415_384` reader - PCIE PHY Parameter (little-endian)"]
pub type U1PcieKPhyparam415_384R = crate::FieldReader<u32>;
#[doc = "Field `u1_pcie_k_phyparam_415_384` writer - PCIE PHY Parameter (little-endian)"]
pub type U1PcieKPhyparam415_384W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - PCIE PHY Parameter (little-endian)"]
    #[inline(always)]
    pub fn u1_pcie_k_phyparam_415_384(&self) -> U1PcieKPhyparam415_384R {
        U1PcieKPhyparam415_384R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - PCIE PHY Parameter (little-endian)"]
    #[inline(always)]
    #[must_use]
    pub fn u1_pcie_k_phyparam_415_384(&mut self) -> U1PcieKPhyparam415_384W<StgSyscfg170Spec> {
        U1PcieKPhyparam415_384W::new(self, 0)
    }
}
#[doc = "STG SYSCONSAIF SYSCFG 680\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_syscfg_170::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stg_syscfg_170::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StgSyscfg170Spec;
impl crate::RegisterSpec for StgSyscfg170Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_syscfg_170::R`](R) reader structure"]
impl crate::Readable for StgSyscfg170Spec {}
#[doc = "`write(|w| ..)` method takes [`stg_syscfg_170::W`](W) writer structure"]
impl crate::Writable for StgSyscfg170Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets stg_syscfg_170 to value 0"]
impl crate::Resettable for StgSyscfg170Spec {
    const RESET_VALUE: u32 = 0;
}