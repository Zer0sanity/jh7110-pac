#[doc = "Register `target_time_sec` reader"]
pub type R = crate::R<TargetTimeSecSpec>;
#[doc = "Register `target_time_sec` writer"]
pub type W = crate::W<TargetTimeSecSpec>;
#[doc = "Field `time` reader - Target Time - Seconds"]
pub type TimeR = crate::FieldReader<u32>;
#[doc = "Field `time` writer - Target Time - Seconds"]
pub type TimeW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Target Time - Seconds"]
    #[inline(always)]
    pub fn time(&self) -> TimeR {
        TimeR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Target Time - Seconds"]
    #[inline(always)]
    #[must_use]
    pub fn time(&mut self) -> TimeW<TargetTimeSecSpec> {
        TimeW::new(self, 0)
    }
}
#[doc = "PPS Target Time - Seconds\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`target_time_sec::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`target_time_sec::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TargetTimeSecSpec;
impl crate::RegisterSpec for TargetTimeSecSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`target_time_sec::R`](R) reader structure"]
impl crate::Readable for TargetTimeSecSpec {}
#[doc = "`write(|w| ..)` method takes [`target_time_sec::W`](W) writer structure"]
impl crate::Writable for TargetTimeSecSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets target_time_sec to value 0"]
impl crate::Resettable for TargetTimeSecSpec {
    const RESET_VALUE: u32 = 0;
}
