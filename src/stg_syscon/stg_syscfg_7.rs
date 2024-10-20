#[doc = "Register `stg_syscfg_7` reader"]
pub type R = crate::R<StgSyscfg7Spec>;
#[doc = "Register `stg_syscfg_7` writer"]
pub type W = crate::W<StgSyscfg7Spec>;
#[doc = "Field `u0_e2_nmi_exception_vector` reader - "]
pub type U0E2NmiExceptionVectorR = crate::FieldReader<u32>;
#[doc = "Field `u0_e2_nmi_exception_vector` writer - "]
pub type U0E2NmiExceptionVectorW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn u0_e2_nmi_exception_vector(&self) -> U0E2NmiExceptionVectorR {
        U0E2NmiExceptionVectorR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn u0_e2_nmi_exception_vector(&mut self) -> U0E2NmiExceptionVectorW<StgSyscfg7Spec> {
        U0E2NmiExceptionVectorW::new(self, 0)
    }
}
#[doc = "STG SYSCONSAIF SYSCFG 28\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_syscfg_7::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stg_syscfg_7::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StgSyscfg7Spec;
impl crate::RegisterSpec for StgSyscfg7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_syscfg_7::R`](R) reader structure"]
impl crate::Readable for StgSyscfg7Spec {}
#[doc = "`write(|w| ..)` method takes [`stg_syscfg_7::W`](W) writer structure"]
impl crate::Writable for StgSyscfg7Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets stg_syscfg_7 to value 0"]
impl crate::Resettable for StgSyscfg7Spec {
    const RESET_VALUE: u32 = 0;
}
