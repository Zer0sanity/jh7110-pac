#[doc = "Register `stg_syscfg_173` reader"]
pub type R = crate::R<StgSyscfg173Spec>;
#[doc = "Register `stg_syscfg_173` writer"]
pub type W = crate::W<StgSyscfg173Spec>;
#[doc = "Field `u1_pcie_k_phyparam_511_480` reader - PCIE PHY Parameter (little-endian)"]
pub type U1PcieKPhyparam511_480R = crate::FieldReader<u32>;
#[doc = "Field `u1_pcie_k_phyparam_511_480` writer - PCIE PHY Parameter (little-endian)"]
pub type U1PcieKPhyparam511_480W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - PCIE PHY Parameter (little-endian)"]
    #[inline(always)]
    pub fn u1_pcie_k_phyparam_511_480(&self) -> U1PcieKPhyparam511_480R {
        U1PcieKPhyparam511_480R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - PCIE PHY Parameter (little-endian)"]
    #[inline(always)]
    #[must_use]
    pub fn u1_pcie_k_phyparam_511_480(&mut self) -> U1PcieKPhyparam511_480W<StgSyscfg173Spec> {
        U1PcieKPhyparam511_480W::new(self, 0)
    }
}
#[doc = "STG SYSCONSAIF SYSCFG 692\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_syscfg_173::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stg_syscfg_173::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StgSyscfg173Spec;
impl crate::RegisterSpec for StgSyscfg173Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_syscfg_173::R`](R) reader structure"]
impl crate::Readable for StgSyscfg173Spec {}
#[doc = "`write(|w| ..)` method takes [`stg_syscfg_173::W`](W) writer structure"]
impl crate::Writable for StgSyscfg173Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets stg_syscfg_173 to value 0"]
impl crate::Resettable for StgSyscfg173Spec {
    const RESET_VALUE: u32 = 0;
}
