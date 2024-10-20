#[doc = "Register `stg_syscfg_182` reader"]
pub type R = crate::R<StgSyscfg182Spec>;
#[doc = "Register `stg_syscfg_182` writer"]
pub type W = crate::W<StgSyscfg182Spec>;
#[doc = "Field `u1_pcie_k_phyparam_799_768` reader - PCIE PHY Parameter (little-endian)"]
pub type U1PcieKPhyparam799_768R = crate::FieldReader<u32>;
#[doc = "Field `u1_pcie_k_phyparam_799_768` writer - PCIE PHY Parameter (little-endian)"]
pub type U1PcieKPhyparam799_768W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - PCIE PHY Parameter (little-endian)"]
    #[inline(always)]
    pub fn u1_pcie_k_phyparam_799_768(&self) -> U1PcieKPhyparam799_768R {
        U1PcieKPhyparam799_768R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - PCIE PHY Parameter (little-endian)"]
    #[inline(always)]
    #[must_use]
    pub fn u1_pcie_k_phyparam_799_768(&mut self) -> U1PcieKPhyparam799_768W<StgSyscfg182Spec> {
        U1PcieKPhyparam799_768W::new(self, 0)
    }
}
#[doc = "STG SYSCONSAIF SYSCFG 728\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_syscfg_182::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stg_syscfg_182::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StgSyscfg182Spec;
impl crate::RegisterSpec for StgSyscfg182Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_syscfg_182::R`](R) reader structure"]
impl crate::Readable for StgSyscfg182Spec {}
#[doc = "`write(|w| ..)` method takes [`stg_syscfg_182::W`](W) writer structure"]
impl crate::Writable for StgSyscfg182Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets stg_syscfg_182 to value 0"]
impl crate::Resettable for StgSyscfg182Spec {
    const RESET_VALUE: u32 = 0;
}
