#[doc = "Register `stg_syscfg_11` reader"]
pub type R = crate::R<StgSyscfg11Spec>;
#[doc = "Register `stg_syscfg_11` writer"]
pub type W = crate::W<StgSyscfg11Spec>;
#[doc = "Field `u0_hifi4_altresetvec` reader - Reset Vector Address"]
pub type U0Hifi4AltresetvecR = crate::FieldReader<u32>;
#[doc = "Field `u0_hifi4_altresetvec` writer - Reset Vector Address"]
pub type U0Hifi4AltresetvecW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Reset Vector Address"]
    #[inline(always)]
    pub fn u0_hifi4_altresetvec(&self) -> U0Hifi4AltresetvecR {
        U0Hifi4AltresetvecR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Reset Vector Address"]
    #[inline(always)]
    #[must_use]
    pub fn u0_hifi4_altresetvec(&mut self) -> U0Hifi4AltresetvecW<StgSyscfg11Spec> {
        U0Hifi4AltresetvecW::new(self, 0)
    }
}
#[doc = "STG SYSCONSAIF SYSCFG 44\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_syscfg_11::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stg_syscfg_11::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StgSyscfg11Spec;
impl crate::RegisterSpec for StgSyscfg11Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_syscfg_11::R`](R) reader structure"]
impl crate::Readable for StgSyscfg11Spec {}
#[doc = "`write(|w| ..)` method takes [`stg_syscfg_11::W`](W) writer structure"]
impl crate::Writable for StgSyscfg11Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets stg_syscfg_11 to value 0"]
impl crate::Resettable for StgSyscfg11Spec {
    const RESET_VALUE: u32 = 0;
}
