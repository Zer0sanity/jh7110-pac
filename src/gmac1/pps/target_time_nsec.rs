#[doc = "Register `target_time_nsec` reader"]
pub type R = crate::R<TargetTimeNsecSpec>;
#[doc = "Register `target_time_nsec` writer"]
pub type W = crate::W<TargetTimeNsecSpec>;
#[doc = "Field `time` reader - Target Time - Nanoseconds"]
pub type TimeR = crate::FieldReader<u32>;
#[doc = "Field `time` writer - Target Time - Nanoseconds"]
pub type TimeW<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
#[doc = "Field `busy` reader - Target Busy"]
pub type BusyR = crate::BitReader;
#[doc = "Field `busy` writer - Target Busy"]
pub type BusyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:30 - Target Time - Nanoseconds"]
    #[inline(always)]
    pub fn time(&self) -> TimeR {
        TimeR::new(self.bits & 0x7fff_ffff)
    }
    #[doc = "Bit 31 - Target Busy"]
    #[inline(always)]
    pub fn busy(&self) -> BusyR {
        BusyR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:30 - Target Time - Nanoseconds"]
    #[inline(always)]
    #[must_use]
    pub fn time(&mut self) -> TimeW<TargetTimeNsecSpec> {
        TimeW::new(self, 0)
    }
    #[doc = "Bit 31 - Target Busy"]
    #[inline(always)]
    #[must_use]
    pub fn busy(&mut self) -> BusyW<TargetTimeNsecSpec> {
        BusyW::new(self, 31)
    }
}
#[doc = "PPS Target Time - Nanoseconds\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`target_time_nsec::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`target_time_nsec::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TargetTimeNsecSpec;
impl crate::RegisterSpec for TargetTimeNsecSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`target_time_nsec::R`](R) reader structure"]
impl crate::Readable for TargetTimeNsecSpec {}
#[doc = "`write(|w| ..)` method takes [`target_time_nsec::W`](W) writer structure"]
impl crate::Writable for TargetTimeNsecSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets target_time_nsec to value 0"]
impl crate::Resettable for TargetTimeNsecSpec {
    const RESET_VALUE: u32 = 0;
}
