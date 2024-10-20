#[doc = "Register `rtc_toggle` reader"]
pub type R = crate::R<RtcToggleSpec>;
#[doc = "Register `rtc_toggle` writer"]
pub type W = crate::W<RtcToggleSpec>;
#[doc = "Field `clk_divcfg` reader - Clock divider coefficient: Max=6, Default=6, Min=6, Typical=6"]
pub type ClkDivcfgR = crate::FieldReader<u32>;
#[doc = "Field `clk_divcfg` writer - Clock divider coefficient: Max=6, Default=6, Min=6, Typical=6"]
pub type ClkDivcfgW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - Clock divider coefficient: Max=6, Default=6, Min=6, Typical=6"]
    #[inline(always)]
    pub fn clk_divcfg(&self) -> ClkDivcfgR {
        ClkDivcfgR::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - Clock divider coefficient: Max=6, Default=6, Min=6, Typical=6"]
    #[inline(always)]
    #[must_use]
    pub fn clk_divcfg(&mut self) -> ClkDivcfgW<RtcToggleSpec> {
        ClkDivcfgW::new(self, 0)
    }
}
#[doc = "Clock U7MC RTC Toggle\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtc_toggle::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_toggle::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RtcToggleSpec;
impl crate::RegisterSpec for RtcToggleSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtc_toggle::R`](R) reader structure"]
impl crate::Readable for RtcToggleSpec {}
#[doc = "`write(|w| ..)` method takes [`rtc_toggle::W`](W) writer structure"]
impl crate::Writable for RtcToggleSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets rtc_toggle to value 0x06"]
impl crate::Resettable for RtcToggleSpec {
    const RESET_VALUE: u32 = 0x06;
}
