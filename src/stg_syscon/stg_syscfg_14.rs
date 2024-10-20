#[doc = "Register `stg_syscfg_14` reader"]
pub type R = crate::R<StgSyscfg14Spec>;
#[doc = "Register `stg_syscfg_14` writer"]
pub type W = crate::W<StgSyscfg14Spec>;
#[doc = "Field `u0_hifi4_pfaultinfovalid` reader - Fault Handling Signals"]
pub type U0Hifi4PfaultinfovalidR = crate::BitReader;
#[doc = "Field `u0_hifi4_prid` reader - Module ID"]
pub type U0Hifi4PridR = crate::FieldReader<u16>;
#[doc = "Field `u0_hifi4_prid` writer - Module ID"]
pub type U0Hifi4PridW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `u0_hifi4_pwaitmode` reader - Wait Mode"]
pub type U0Hifi4PwaitmodeR = crate::BitReader;
#[doc = "Field `u0_hifi4_runstall` reader - Run Stall"]
pub type U0Hifi4RunstallR = crate::BitReader;
#[doc = "Field `u0_hifi4_runstall` writer - Run Stall"]
pub type U0Hifi4RunstallW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Fault Handling Signals"]
    #[inline(always)]
    pub fn u0_hifi4_pfaultinfovalid(&self) -> U0Hifi4PfaultinfovalidR {
        U0Hifi4PfaultinfovalidR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:16 - Module ID"]
    #[inline(always)]
    pub fn u0_hifi4_prid(&self) -> U0Hifi4PridR {
        U0Hifi4PridR::new(((self.bits >> 1) & 0xffff) as u16)
    }
    #[doc = "Bit 17 - Wait Mode"]
    #[inline(always)]
    pub fn u0_hifi4_pwaitmode(&self) -> U0Hifi4PwaitmodeR {
        U0Hifi4PwaitmodeR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Run Stall"]
    #[inline(always)]
    pub fn u0_hifi4_runstall(&self) -> U0Hifi4RunstallR {
        U0Hifi4RunstallR::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 1:16 - Module ID"]
    #[inline(always)]
    #[must_use]
    pub fn u0_hifi4_prid(&mut self) -> U0Hifi4PridW<StgSyscfg14Spec> {
        U0Hifi4PridW::new(self, 1)
    }
    #[doc = "Bit 18 - Run Stall"]
    #[inline(always)]
    #[must_use]
    pub fn u0_hifi4_runstall(&mut self) -> U0Hifi4RunstallW<StgSyscfg14Spec> {
        U0Hifi4RunstallW::new(self, 18)
    }
}
#[doc = "STG SYSCONSAIF SYSCFG 56\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_syscfg_14::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stg_syscfg_14::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StgSyscfg14Spec;
impl crate::RegisterSpec for StgSyscfg14Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_syscfg_14::R`](R) reader structure"]
impl crate::Readable for StgSyscfg14Spec {}
#[doc = "`write(|w| ..)` method takes [`stg_syscfg_14::W`](W) writer structure"]
impl crate::Writable for StgSyscfg14Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets stg_syscfg_14 to value 0"]
impl crate::Resettable for StgSyscfg14Spec {
    const RESET_VALUE: u32 = 0;
}
