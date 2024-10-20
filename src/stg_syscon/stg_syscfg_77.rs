#[doc = "Register `stg_syscfg_77` reader"]
pub type R = crate::R<StgSyscfg77Spec>;
#[doc = "Register `stg_syscfg_77` writer"]
pub type W = crate::W<StgSyscfg77Spec>;
#[doc = "Field `u0_pcie_local_interrupt_in` reader - PCIE Local Interrupt IN"]
pub type U0PcieLocalInterruptInR = crate::FieldReader<u32>;
#[doc = "Field `u0_pcie_local_interrupt_in` writer - PCIE Local Interrupt IN"]
pub type U0PcieLocalInterruptInW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - PCIE Local Interrupt IN"]
    #[inline(always)]
    pub fn u0_pcie_local_interrupt_in(&self) -> U0PcieLocalInterruptInR {
        U0PcieLocalInterruptInR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - PCIE Local Interrupt IN"]
    #[inline(always)]
    #[must_use]
    pub fn u0_pcie_local_interrupt_in(&mut self) -> U0PcieLocalInterruptInW<StgSyscfg77Spec> {
        U0PcieLocalInterruptInW::new(self, 0)
    }
}
#[doc = "STG SYSCONSAIF SYSCFG 308\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_syscfg_77::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stg_syscfg_77::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StgSyscfg77Spec;
impl crate::RegisterSpec for StgSyscfg77Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_syscfg_77::R`](R) reader structure"]
impl crate::Readable for StgSyscfg77Spec {}
#[doc = "`write(|w| ..)` method takes [`stg_syscfg_77::W`](W) writer structure"]
impl crate::Writable for StgSyscfg77Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets stg_syscfg_77 to value 0"]
impl crate::Resettable for StgSyscfg77Spec {
    const RESET_VALUE: u32 = 0;
}
