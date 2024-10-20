#[doc = "Register `hard_event_turn_on_mask` reader"]
pub type R = crate::R<HardEventTurnOnMaskSpec>;
#[doc = "Register `hard_event_turn_on_mask` writer"]
pub type W = crate::W<HardEventTurnOnMaskSpec>;
#[doc = "Field `hard_event_rtc_on_mask` reader - RTC event encourage turn-on sequence, 0: enable hardware event, 1: mask hardware event"]
pub type HardEventRtcOnMaskR = crate::BitReader;
#[doc = "Field `hard_event_rtc_on_mask` writer - RTC event encourage turn-on sequence, 0: enable hardware event, 1: mask hardware event"]
pub type HardEventRtcOnMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `hard_event_gmac_on_mask` reader - GMAC event encourage turn-on sequence, 0: enable hardware event, 1: mask hardware event"]
pub type HardEventGmacOnMaskR = crate::BitReader;
#[doc = "Field `hard_event_gmac_on_mask` writer - GMAC event encourage turn-on sequence, 0: enable hardware event, 1: mask hardware event"]
pub type HardEventGmacOnMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `hard_event_rfu_on_mask` reader - RFU, 0: enable hardware event, 1: mask hardware event"]
pub type HardEventRfuOnMaskR = crate::BitReader;
#[doc = "Field `hard_event_rfu_on_mask` writer - RFU, 0: enable hardware event, 1: mask hardware event"]
pub type HardEventRfuOnMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `hard_event_rgpio0_on_mask` reader - RGPIO0 event encourage turn-on sequence, 0: enable hardware event, 1: mask hardware event"]
pub type HardEventRgpio0OnMaskR = crate::BitReader;
#[doc = "Field `hard_event_rgpio0_on_mask` writer - RGPIO0 event encourage turn-on sequence, 0: enable hardware event, 1: mask hardware event"]
pub type HardEventRgpio0OnMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `hard_event_rgpio1_on_mask` reader - RGPIO1 event encourage turn-on sequence, 0: enable hardware event, 1: mask hardware event"]
pub type HardEventRgpio1OnMaskR = crate::BitReader;
#[doc = "Field `hard_event_rgpio1_on_mask` writer - RGPIO1 event encourage turn-on sequence, 0: enable hardware event, 1: mask hardware event"]
pub type HardEventRgpio1OnMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `hard_event_rgpio2_on_mask` reader - RGPIO2 event encourage turn-on sequence, 0: enable hardware event, 1: mask hardware event"]
pub type HardEventRgpio2OnMaskR = crate::BitReader;
#[doc = "Field `hard_event_rgpio2_on_mask` writer - RGPIO2 event encourage turn-on sequence, 0: enable hardware event, 1: mask hardware event"]
pub type HardEventRgpio2OnMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `hard_event_rgpio3_on_mask` reader - RGPIO3 event encourage turn-on sequence, 0: enable hardware event, 1: mask hardware event"]
pub type HardEventRgpio3OnMaskR = crate::BitReader;
#[doc = "Field `hard_event_rgpio3_on_mask` writer - RGPIO3 event encourage turn-on sequence, 0: enable hardware event, 1: mask hardware event"]
pub type HardEventRgpio3OnMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `hard_event_gpu_on_mask` reader - GPU event, 0: enable hardware event, 1: mask hardware event"]
pub type HardEventGpuOnMaskR = crate::BitReader;
#[doc = "Field `hard_event_gpu_on_mask` writer - GPU event, 0: enable hardware event, 1: mask hardware event"]
pub type HardEventGpuOnMaskW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - RTC event encourage turn-on sequence, 0: enable hardware event, 1: mask hardware event"]
    #[inline(always)]
    pub fn hard_event_rtc_on_mask(&self) -> HardEventRtcOnMaskR {
        HardEventRtcOnMaskR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - GMAC event encourage turn-on sequence, 0: enable hardware event, 1: mask hardware event"]
    #[inline(always)]
    pub fn hard_event_gmac_on_mask(&self) -> HardEventGmacOnMaskR {
        HardEventGmacOnMaskR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RFU, 0: enable hardware event, 1: mask hardware event"]
    #[inline(always)]
    pub fn hard_event_rfu_on_mask(&self) -> HardEventRfuOnMaskR {
        HardEventRfuOnMaskR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RGPIO0 event encourage turn-on sequence, 0: enable hardware event, 1: mask hardware event"]
    #[inline(always)]
    pub fn hard_event_rgpio0_on_mask(&self) -> HardEventRgpio0OnMaskR {
        HardEventRgpio0OnMaskR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RGPIO1 event encourage turn-on sequence, 0: enable hardware event, 1: mask hardware event"]
    #[inline(always)]
    pub fn hard_event_rgpio1_on_mask(&self) -> HardEventRgpio1OnMaskR {
        HardEventRgpio1OnMaskR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RGPIO2 event encourage turn-on sequence, 0: enable hardware event, 1: mask hardware event"]
    #[inline(always)]
    pub fn hard_event_rgpio2_on_mask(&self) -> HardEventRgpio2OnMaskR {
        HardEventRgpio2OnMaskR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - RGPIO3 event encourage turn-on sequence, 0: enable hardware event, 1: mask hardware event"]
    #[inline(always)]
    pub fn hard_event_rgpio3_on_mask(&self) -> HardEventRgpio3OnMaskR {
        HardEventRgpio3OnMaskR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - GPU event, 0: enable hardware event, 1: mask hardware event"]
    #[inline(always)]
    pub fn hard_event_gpu_on_mask(&self) -> HardEventGpuOnMaskR {
        HardEventGpuOnMaskR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RTC event encourage turn-on sequence, 0: enable hardware event, 1: mask hardware event"]
    #[inline(always)]
    #[must_use]
    pub fn hard_event_rtc_on_mask(&mut self) -> HardEventRtcOnMaskW<HardEventTurnOnMaskSpec> {
        HardEventRtcOnMaskW::new(self, 0)
    }
    #[doc = "Bit 1 - GMAC event encourage turn-on sequence, 0: enable hardware event, 1: mask hardware event"]
    #[inline(always)]
    #[must_use]
    pub fn hard_event_gmac_on_mask(&mut self) -> HardEventGmacOnMaskW<HardEventTurnOnMaskSpec> {
        HardEventGmacOnMaskW::new(self, 1)
    }
    #[doc = "Bit 2 - RFU, 0: enable hardware event, 1: mask hardware event"]
    #[inline(always)]
    #[must_use]
    pub fn hard_event_rfu_on_mask(&mut self) -> HardEventRfuOnMaskW<HardEventTurnOnMaskSpec> {
        HardEventRfuOnMaskW::new(self, 2)
    }
    #[doc = "Bit 3 - RGPIO0 event encourage turn-on sequence, 0: enable hardware event, 1: mask hardware event"]
    #[inline(always)]
    #[must_use]
    pub fn hard_event_rgpio0_on_mask(&mut self) -> HardEventRgpio0OnMaskW<HardEventTurnOnMaskSpec> {
        HardEventRgpio0OnMaskW::new(self, 3)
    }
    #[doc = "Bit 4 - RGPIO1 event encourage turn-on sequence, 0: enable hardware event, 1: mask hardware event"]
    #[inline(always)]
    #[must_use]
    pub fn hard_event_rgpio1_on_mask(&mut self) -> HardEventRgpio1OnMaskW<HardEventTurnOnMaskSpec> {
        HardEventRgpio1OnMaskW::new(self, 4)
    }
    #[doc = "Bit 5 - RGPIO2 event encourage turn-on sequence, 0: enable hardware event, 1: mask hardware event"]
    #[inline(always)]
    #[must_use]
    pub fn hard_event_rgpio2_on_mask(&mut self) -> HardEventRgpio2OnMaskW<HardEventTurnOnMaskSpec> {
        HardEventRgpio2OnMaskW::new(self, 5)
    }
    #[doc = "Bit 6 - RGPIO3 event encourage turn-on sequence, 0: enable hardware event, 1: mask hardware event"]
    #[inline(always)]
    #[must_use]
    pub fn hard_event_rgpio3_on_mask(&mut self) -> HardEventRgpio3OnMaskW<HardEventTurnOnMaskSpec> {
        HardEventRgpio3OnMaskW::new(self, 6)
    }
    #[doc = "Bit 7 - GPU event, 0: enable hardware event, 1: mask hardware event"]
    #[inline(always)]
    #[must_use]
    pub fn hard_event_gpu_on_mask(&mut self) -> HardEventGpuOnMaskW<HardEventTurnOnMaskSpec> {
        HardEventGpuOnMaskW::new(self, 7)
    }
}
#[doc = "Hardware Event Turn-On Mask\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hard_event_turn_on_mask::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hard_event_turn_on_mask::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HardEventTurnOnMaskSpec;
impl crate::RegisterSpec for HardEventTurnOnMaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hard_event_turn_on_mask::R`](R) reader structure"]
impl crate::Readable for HardEventTurnOnMaskSpec {}
#[doc = "`write(|w| ..)` method takes [`hard_event_turn_on_mask::W`](W) writer structure"]
impl crate::Writable for HardEventTurnOnMaskSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets hard_event_turn_on_mask to value 0"]
impl crate::Resettable for HardEventTurnOnMaskSpec {
    const RESET_VALUE: u32 = 0;
}
