#[doc = "Register `timer_ctrl` reader"]
pub type R = crate::R<TimerCtrlSpec>;
#[doc = "Register `timer_ctrl` writer"]
pub type W = crate::W<TimerCtrlSpec>;
#[doc = "Field `timer_ctrl` reader - LPI Timer Control"]
pub type TimerCtrlR = crate::FieldReader<u32>;
#[doc = "Field `timer_ctrl` writer - LPI Timer Control"]
pub type TimerCtrlW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - LPI Timer Control"]
    #[inline(always)]
    pub fn timer_ctrl(&self) -> TimerCtrlR {
        TimerCtrlR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - LPI Timer Control"]
    #[inline(always)]
    #[must_use]
    pub fn timer_ctrl(&mut self) -> TimerCtrlW<TimerCtrlSpec> {
        TimerCtrlW::new(self, 0)
    }
}
#[doc = "MAC LPI Timer Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timer_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timer_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TimerCtrlSpec;
impl crate::RegisterSpec for TimerCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timer_ctrl::R`](R) reader structure"]
impl crate::Readable for TimerCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`timer_ctrl::W`](W) writer structure"]
impl crate::Writable for TimerCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets timer_ctrl to value 0"]
impl crate::Resettable for TimerCtrlSpec {
    const RESET_VALUE: u32 = 0;
}
