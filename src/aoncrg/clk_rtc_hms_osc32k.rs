#[doc = "Register `clk_rtc_hms_osc32k` reader"]
pub type R = crate::R<ClkRtcHmsOsc32kSpec>;
#[doc = "Register `clk_rtc_hms_osc32k` writer"]
pub type W = crate::W<ClkRtcHmsOsc32kSpec>;
#[doc = "Clock multiplexing selector: clk_rtc, clk_rtc_internal\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ClkMuxSel {
    #[doc = "0: Select `clk_rtc` as the RTC HMC Oscillator 32K clock."]
    ClkRtc = 0,
    #[doc = "1: Select `clk_rtc_internal` as the RTC HMC Oscillator 32K clock."]
    ClkRtcInternal = 1,
}
impl From<ClkMuxSel> for u8 {
    #[inline(always)]
    fn from(variant: ClkMuxSel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ClkMuxSel {
    type Ux = u8;
}
#[doc = "Field `clk_mux_sel` reader - Clock multiplexing selector: clk_rtc, clk_rtc_internal"]
pub type ClkMuxSelR = crate::FieldReader<ClkMuxSel>;
impl ClkMuxSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<ClkMuxSel> {
        match self.bits {
            0 => Some(ClkMuxSel::ClkRtc),
            1 => Some(ClkMuxSel::ClkRtcInternal),
            _ => None,
        }
    }
    #[doc = "Select `clk_rtc` as the RTC HMC Oscillator 32K clock."]
    #[inline(always)]
    pub fn is_clk_rtc(&self) -> bool {
        *self == ClkMuxSel::ClkRtc
    }
    #[doc = "Select `clk_rtc_internal` as the RTC HMC Oscillator 32K clock."]
    #[inline(always)]
    pub fn is_clk_rtc_internal(&self) -> bool {
        *self == ClkMuxSel::ClkRtcInternal
    }
}
#[doc = "Field `clk_mux_sel` writer - Clock multiplexing selector: clk_rtc, clk_rtc_internal"]
pub type ClkMuxSelW<'a, REG> = crate::FieldWriter<'a, REG, 6, ClkMuxSel>;
impl<'a, REG> ClkMuxSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Select `clk_rtc` as the RTC HMC Oscillator 32K clock."]
    #[inline(always)]
    pub fn clk_rtc(self) -> &'a mut crate::W<REG> {
        self.variant(ClkMuxSel::ClkRtc)
    }
    #[doc = "Select `clk_rtc_internal` as the RTC HMC Oscillator 32K clock."]
    #[inline(always)]
    pub fn clk_rtc_internal(self) -> &'a mut crate::W<REG> {
        self.variant(ClkMuxSel::ClkRtcInternal)
    }
}
impl R {
    #[doc = "Bits 24:29 - Clock multiplexing selector: clk_rtc, clk_rtc_internal"]
    #[inline(always)]
    pub fn clk_mux_sel(&self) -> ClkMuxSelR {
        ClkMuxSelR::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 24:29 - Clock multiplexing selector: clk_rtc, clk_rtc_internal"]
    #[inline(always)]
    #[must_use]
    pub fn clk_mux_sel(&mut self) -> ClkMuxSelW<ClkRtcHmsOsc32kSpec> {
        ClkMuxSelW::new(self, 24)
    }
}
#[doc = "RTC HMS Clock Oscillator 32K\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_rtc_hms_osc32k::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_rtc_hms_osc32k::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkRtcHmsOsc32kSpec;
impl crate::RegisterSpec for ClkRtcHmsOsc32kSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk_rtc_hms_osc32k::R`](R) reader structure"]
impl crate::Readable for ClkRtcHmsOsc32kSpec {}
#[doc = "`write(|w| ..)` method takes [`clk_rtc_hms_osc32k::W`](W) writer structure"]
impl crate::Writable for ClkRtcHmsOsc32kSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets clk_rtc_hms_osc32k to value 0"]
impl crate::Resettable for ClkRtcHmsOsc32kSpec {
    const RESET_VALUE: u32 = 0;
}
