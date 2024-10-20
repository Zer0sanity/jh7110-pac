#[doc = "Register `stg_syscfg_8` reader"]
pub type R = crate::R<StgSyscfg8Spec>;
#[doc = "Register `stg_syscfg_8` writer"]
pub type W = crate::W<StgSyscfg8Spec>;
#[doc = "Field `u0_e2_nmi_interrupt_vector` reader - "]
pub type U0E2NmiInterruptVectorR = crate::FieldReader<u32>;
#[doc = "Field `u0_e2_nmi_interrupt_vector` writer - "]
pub type U0E2NmiInterruptVectorW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn u0_e2_nmi_interrupt_vector(&self) -> U0E2NmiInterruptVectorR {
        U0E2NmiInterruptVectorR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn u0_e2_nmi_interrupt_vector(&mut self) -> U0E2NmiInterruptVectorW<StgSyscfg8Spec> {
        U0E2NmiInterruptVectorW::new(self, 0)
    }
}
#[doc = "STG SYSCONSAIF SYSCFG 32\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_syscfg_8::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stg_syscfg_8::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StgSyscfg8Spec;
impl crate::RegisterSpec for StgSyscfg8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_syscfg_8::R`](R) reader structure"]
impl crate::Readable for StgSyscfg8Spec {}
#[doc = "`write(|w| ..)` method takes [`stg_syscfg_8::W`](W) writer structure"]
impl crate::Writable for StgSyscfg8Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets stg_syscfg_8 to value 0"]
impl crate::Resettable for StgSyscfg8Spec {
    const RESET_VALUE: u32 = 0;
}
