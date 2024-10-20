#[doc = "Register `stg_syscfg_185` reader"]
pub type R = crate::R<StgSyscfg185Spec>;
#[doc = "Register `stg_syscfg_185` writer"]
pub type W = crate::W<StgSyscfg185Spec>;
#[doc = "Field `u1_pcie_local_interrupt_in` reader - PCIE Local Interrupt IN"]
pub type U1PcieLocalInterruptInR = crate::FieldReader<u32>;
#[doc = "Field `u1_pcie_local_interrupt_in` writer - PCIE Local Interrupt IN"]
pub type U1PcieLocalInterruptInW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - PCIE Local Interrupt IN"]
    #[inline(always)]
    pub fn u1_pcie_local_interrupt_in(&self) -> U1PcieLocalInterruptInR {
        U1PcieLocalInterruptInR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - PCIE Local Interrupt IN"]
    #[inline(always)]
    #[must_use]
    pub fn u1_pcie_local_interrupt_in(&mut self) -> U1PcieLocalInterruptInW<StgSyscfg185Spec> {
        U1PcieLocalInterruptInW::new(self, 0)
    }
}
#[doc = "STG SYSCONSAIF SYSCFG 740\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_syscfg_185::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stg_syscfg_185::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StgSyscfg185Spec;
impl crate::RegisterSpec for StgSyscfg185Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_syscfg_185::R`](R) reader structure"]
impl crate::Readable for StgSyscfg185Spec {}
#[doc = "`write(|w| ..)` method takes [`stg_syscfg_185::W`](W) writer structure"]
impl crate::Writable for StgSyscfg185Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets stg_syscfg_185 to value 0"]
impl crate::Resettable for StgSyscfg185Spec {
    const RESET_VALUE: u32 = 0;
}
