#[doc = "Register `clk_rtc_internal` reader"]
pub type R = crate::R<ClkRtcInternalSpec>;
#[doc = "Register `clk_rtc_internal` writer"]
pub type W = crate::W<ClkRtcInternalSpec>;
#[doc = "Field `clk_divcfg` reader - Clock divider coefficient: Max=1022, Default=750, Min=750, Typical=750"]
pub type ClkDivcfgR = crate::FieldReader<u32>;
#[doc = "Field `clk_divcfg` writer - Clock divider coefficient: Max=1022, Default=750, Min=750, Typical=750"]
pub type ClkDivcfgW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - Clock divider coefficient: Max=1022, Default=750, Min=750, Typical=750"]
    #[inline(always)]
    pub fn clk_divcfg(&self) -> ClkDivcfgR {
        ClkDivcfgR::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - Clock divider coefficient: Max=1022, Default=750, Min=750, Typical=750"]
    #[inline(always)]
    #[must_use]
    pub fn clk_divcfg(&mut self) -> ClkDivcfgW<ClkRtcInternalSpec> {
        ClkDivcfgW::new(self, 0)
    }
}
#[doc = "RTC Internal Clock\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_rtc_internal::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_rtc_internal::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkRtcInternalSpec;
impl crate::RegisterSpec for ClkRtcInternalSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk_rtc_internal::R`](R) reader structure"]
impl crate::Readable for ClkRtcInternalSpec {}
#[doc = "`write(|w| ..)` method takes [`clk_rtc_internal::W`](W) writer structure"]
impl crate::Writable for ClkRtcInternalSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets clk_rtc_internal to value 0x02ee"]
impl crate::Resettable for ClkRtcInternalSpec {
    const RESET_VALUE: u32 = 0x02ee;
}