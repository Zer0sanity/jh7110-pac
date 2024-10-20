#[doc = "Register `control` reader"]
pub type R = crate::R<ControlSpec>;
#[doc = "Register `control` writer"]
pub type W = crate::W<ControlSpec>;
#[doc = "Field `reset` reader - Watchdog reset enable - 0: disable, 1: enable."]
pub type ResetR = crate::BitReader;
#[doc = "Field `reset` writer - Watchdog reset enable - 0: disable, 1: enable."]
pub type ResetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `enable` reader - Watchdog interrupt enable, WDT enable, reload counter - 0: disable/no-op, 1: enable/reload."]
pub type EnableR = crate::BitReader;
#[doc = "Field `enable` writer - Watchdog interrupt enable, WDT enable, reload counter - 0: disable/no-op, 1: enable/reload."]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Watchdog reset enable - 0: disable, 1: enable."]
    #[inline(always)]
    pub fn reset(&self) -> ResetR {
        ResetR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Watchdog interrupt enable, WDT enable, reload counter - 0: disable/no-op, 1: enable/reload."]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Watchdog reset enable - 0: disable, 1: enable."]
    #[inline(always)]
    #[must_use]
    pub fn reset(&mut self) -> ResetW<ControlSpec> {
        ResetW::new(self, 0)
    }
    #[doc = "Bit 1 - Watchdog interrupt enable, WDT enable, reload counter - 0: disable/no-op, 1: enable/reload."]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> EnableW<ControlSpec> {
        EnableW::new(self, 1)
    }
}
#[doc = "StarFive JH7110 Watchdog Control register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`control::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`control::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ControlSpec;
impl crate::RegisterSpec for ControlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`control::R`](R) reader structure"]
impl crate::Readable for ControlSpec {}
#[doc = "`write(|w| ..)` method takes [`control::W`](W) writer structure"]
impl crate::Writable for ControlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets control to value 0"]
impl crate::Resettable for ControlSpec {
    const RESET_VALUE: u32 = 0;
}
