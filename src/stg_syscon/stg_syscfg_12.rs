#[doc = "Register `stg_syscfg_12` reader"]
pub type R = crate::R<StgSyscfg12Spec>;
#[doc = "Register `stg_syscfg_12` writer"]
pub type W = crate::W<StgSyscfg12Spec>;
#[doc = "Field `u0_hifi4_breakin` reader - Debug signal"]
pub type U0Hifi4BreakinR = crate::BitReader;
#[doc = "Field `u0_hifi4_breakin` writer - Debug signal"]
pub type U0Hifi4BreakinW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_hifi4_breakinack` reader - Debug signal"]
pub type U0Hifi4BreakinackR = crate::BitReader;
#[doc = "Field `u0_hifi4_breakout` reader - Debug signal"]
pub type U0Hifi4BreakoutR = crate::BitReader;
#[doc = "Field `u0_hifi4_breakoutack` reader - Debug signal"]
pub type U0Hifi4BreakoutackR = crate::BitReader;
#[doc = "Field `u0_hifi4_breakoutack` writer - Debug signal"]
pub type U0Hifi4BreakoutackW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_hifi4_debugmode` reader - Debug signal"]
pub type U0Hifi4DebugmodeR = crate::BitReader;
#[doc = "Field `u0_hifi4_doubleexceptionerror` reader - Fault Handling Signals"]
pub type U0Hifi4DoubleexceptionerrorR = crate::BitReader;
#[doc = "Field `u0_hifi4_iram0loadstore` reader - Indicates that iram0 works"]
pub type U0Hifi4Iram0loadstoreR = crate::BitReader;
#[doc = "Field `u0_hifi4_iram1loadstore` reader - Indicates that iram1 works"]
pub type U0Hifi4Iram1loadstoreR = crate::BitReader;
#[doc = "Field `u0_hifi4_ocdhaltonreset` reader - Debug signal"]
pub type U0Hifi4OcdhaltonresetR = crate::BitReader;
#[doc = "Field `u0_hifi4_ocdhaltonreset` writer - Debug signal"]
pub type U0Hifi4OcdhaltonresetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_hifi4_pfatalerror` reader - Fault Handling Signals"]
pub type U0Hifi4PfatalerrorR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Debug signal"]
    #[inline(always)]
    pub fn u0_hifi4_breakin(&self) -> U0Hifi4BreakinR {
        U0Hifi4BreakinR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Debug signal"]
    #[inline(always)]
    pub fn u0_hifi4_breakinack(&self) -> U0Hifi4BreakinackR {
        U0Hifi4BreakinackR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Debug signal"]
    #[inline(always)]
    pub fn u0_hifi4_breakout(&self) -> U0Hifi4BreakoutR {
        U0Hifi4BreakoutR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Debug signal"]
    #[inline(always)]
    pub fn u0_hifi4_breakoutack(&self) -> U0Hifi4BreakoutackR {
        U0Hifi4BreakoutackR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Debug signal"]
    #[inline(always)]
    pub fn u0_hifi4_debugmode(&self) -> U0Hifi4DebugmodeR {
        U0Hifi4DebugmodeR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Fault Handling Signals"]
    #[inline(always)]
    pub fn u0_hifi4_doubleexceptionerror(&self) -> U0Hifi4DoubleexceptionerrorR {
        U0Hifi4DoubleexceptionerrorR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Indicates that iram0 works"]
    #[inline(always)]
    pub fn u0_hifi4_iram0loadstore(&self) -> U0Hifi4Iram0loadstoreR {
        U0Hifi4Iram0loadstoreR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Indicates that iram1 works"]
    #[inline(always)]
    pub fn u0_hifi4_iram1loadstore(&self) -> U0Hifi4Iram1loadstoreR {
        U0Hifi4Iram1loadstoreR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Debug signal"]
    #[inline(always)]
    pub fn u0_hifi4_ocdhaltonreset(&self) -> U0Hifi4OcdhaltonresetR {
        U0Hifi4OcdhaltonresetR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Fault Handling Signals"]
    #[inline(always)]
    pub fn u0_hifi4_pfatalerror(&self) -> U0Hifi4PfatalerrorR {
        U0Hifi4PfatalerrorR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Debug signal"]
    #[inline(always)]
    #[must_use]
    pub fn u0_hifi4_breakin(&mut self) -> U0Hifi4BreakinW<StgSyscfg12Spec> {
        U0Hifi4BreakinW::new(self, 0)
    }
    #[doc = "Bit 3 - Debug signal"]
    #[inline(always)]
    #[must_use]
    pub fn u0_hifi4_breakoutack(&mut self) -> U0Hifi4BreakoutackW<StgSyscfg12Spec> {
        U0Hifi4BreakoutackW::new(self, 3)
    }
    #[doc = "Bit 8 - Debug signal"]
    #[inline(always)]
    #[must_use]
    pub fn u0_hifi4_ocdhaltonreset(&mut self) -> U0Hifi4OcdhaltonresetW<StgSyscfg12Spec> {
        U0Hifi4OcdhaltonresetW::new(self, 8)
    }
}
#[doc = "STG SYSCONSAIF SYSCFG 48\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_syscfg_12::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stg_syscfg_12::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StgSyscfg12Spec;
impl crate::RegisterSpec for StgSyscfg12Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_syscfg_12::R`](R) reader structure"]
impl crate::Readable for StgSyscfg12Spec {}
#[doc = "`write(|w| ..)` method takes [`stg_syscfg_12::W`](W) writer structure"]
impl crate::Writable for StgSyscfg12Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets stg_syscfg_12 to value 0"]
impl crate::Resettable for StgSyscfg12Spec {
    const RESET_VALUE: u32 = 0;
}
