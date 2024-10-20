#[doc = "Register `entry_timer` reader"]
pub type R = crate::R<EntryTimerSpec>;
#[doc = "Register `entry_timer` writer"]
pub type W = crate::W<EntryTimerSpec>;
#[doc = "Field `entry_timer` reader - LPI Entry Timer"]
pub type EntryTimerR = crate::FieldReader<u32>;
#[doc = "Field `entry_timer` writer - LPI Entry Timer"]
pub type EntryTimerW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - LPI Entry Timer"]
    #[inline(always)]
    pub fn entry_timer(&self) -> EntryTimerR {
        EntryTimerR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - LPI Entry Timer"]
    #[inline(always)]
    #[must_use]
    pub fn entry_timer(&mut self) -> EntryTimerW<EntryTimerSpec> {
        EntryTimerW::new(self, 0)
    }
}
#[doc = "MAC LPI Entry Timer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`entry_timer::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`entry_timer::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EntryTimerSpec;
impl crate::RegisterSpec for EntryTimerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`entry_timer::R`](R) reader structure"]
impl crate::Readable for EntryTimerSpec {}
#[doc = "`write(|w| ..)` method takes [`entry_timer::W`](W) writer structure"]
impl crate::Writable for EntryTimerSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets entry_timer to value 0"]
impl crate::Resettable for EntryTimerSpec {
    const RESET_VALUE: u32 = 0;
}
