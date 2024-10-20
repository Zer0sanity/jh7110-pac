#[doc = "Register `syscfg0` reader"]
pub type R = crate::R<Syscfg0Spec>;
#[doc = "Register `syscfg0` writer"]
pub type W = crate::W<Syscfg0Spec>;
#[doc = "Field `u0_cdns_dsitx_slp` reader - SRAM/ROM configuration. SLP: sleep enable, high active, default is low."]
pub type U0CdnsDsitxSlpR = crate::BitReader;
#[doc = "Field `u0_cdns_dsitx_slp` writer - SRAM/ROM configuration. SLP: sleep enable, high active, default is low."]
pub type U0CdnsDsitxSlpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_cdns_dsitx_sd` reader - SRAM/ROM configuration. SD: shutdown enable, high active, default is low."]
pub type U0CdnsDsitxSdR = crate::BitReader;
#[doc = "Field `u0_cdns_dsitx_sd` writer - SRAM/ROM configuration. SD: shutdown enable, high active, default is low."]
pub type U0CdnsDsitxSdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_cdns_dsitx_rtsel` reader - SRAM/ROM configuration. RTSEL: timing setting for debug purpose, default is 2'b01."]
pub type U0CdnsDsitxRtselR = crate::FieldReader;
#[doc = "Field `u0_cdns_dsitx_rtsel` writer - SRAM/ROM configuration. RTSEL: timing setting for debug purpose, default is 2'b01."]
pub type U0CdnsDsitxRtselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `u0_cdns_dsitx_ptsel` reader - SRAM/ROM configuration. PTSEL: timing setting for debug purpose, default is 2'b01."]
pub type U0CdnsDsitxPtselR = crate::FieldReader;
#[doc = "Field `u0_cdns_dsitx_ptsel` writer - SRAM/ROM configuration. PTSEL: timing setting for debug purpose, default is 2'b01."]
pub type U0CdnsDsitxPtselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `u0_cdns_dsitx_trb` reader - SRAM/ROM configuration. TRB: timing setting for debug purpose, default is 2'b01."]
pub type U0CdnsDsitxTrbR = crate::FieldReader;
#[doc = "Field `u0_cdns_dsitx_trb` writer - SRAM/ROM configuration. TRB: timing setting for debug purpose, default is 2'b01."]
pub type U0CdnsDsitxTrbW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `u0_cdns_dsitx_wtsel` reader - SRAM/ROM configuration. WTSEL: timing setting for debug purpose, default is 2'b01."]
pub type U0CdnsDsitxWtselR = crate::FieldReader;
#[doc = "Field `u0_cdns_dsitx_wtsel` writer - SRAM/ROM configuration. WTSEL: timing setting for debug purpose, default is 2'b01."]
pub type U0CdnsDsitxWtselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `u0_cdns_dsitx_dsi_test_generic_ctrl` reader - "]
pub type U0CdnsDsitxDsiTestGenericCtrlR = crate::FieldReader<u16>;
#[doc = "Field `u0_cdns_dsitx_vs` reader - SRAM/ROM configuration. VS: timing setting for debug purpose, default is 1'b1."]
pub type U0CdnsDsitxVsR = crate::BitReader;
#[doc = "Field `u0_cdns_dsitx_vs` writer - SRAM/ROM configuration. VS: timing setting for debug purpose, default is 1'b1."]
pub type U0CdnsDsitxVsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_cdns_dsitx_vg` reader - SRAM/ROM configuration. VG: timing setting for debug purpose, default is 1'b1."]
pub type U0CdnsDsitxVgR = crate::BitReader;
#[doc = "Field `u0_cdns_dsitx_vg` writer - SRAM/ROM configuration. VG: timing setting for debug purpose, default is 1'b1."]
pub type U0CdnsDsitxVgW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SRAM/ROM configuration. SLP: sleep enable, high active, default is low."]
    #[inline(always)]
    pub fn u0_cdns_dsitx_slp(&self) -> U0CdnsDsitxSlpR {
        U0CdnsDsitxSlpR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SRAM/ROM configuration. SD: shutdown enable, high active, default is low."]
    #[inline(always)]
    pub fn u0_cdns_dsitx_sd(&self) -> U0CdnsDsitxSdR {
        U0CdnsDsitxSdR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - SRAM/ROM configuration. RTSEL: timing setting for debug purpose, default is 2'b01."]
    #[inline(always)]
    pub fn u0_cdns_dsitx_rtsel(&self) -> U0CdnsDsitxRtselR {
        U0CdnsDsitxRtselR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - SRAM/ROM configuration. PTSEL: timing setting for debug purpose, default is 2'b01."]
    #[inline(always)]
    pub fn u0_cdns_dsitx_ptsel(&self) -> U0CdnsDsitxPtselR {
        U0CdnsDsitxPtselR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - SRAM/ROM configuration. TRB: timing setting for debug purpose, default is 2'b01."]
    #[inline(always)]
    pub fn u0_cdns_dsitx_trb(&self) -> U0CdnsDsitxTrbR {
        U0CdnsDsitxTrbR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - SRAM/ROM configuration. WTSEL: timing setting for debug purpose, default is 2'b01."]
    #[inline(always)]
    pub fn u0_cdns_dsitx_wtsel(&self) -> U0CdnsDsitxWtselR {
        U0CdnsDsitxWtselR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 8:23"]
    #[inline(always)]
    pub fn u0_cdns_dsitx_dsi_test_generic_ctrl(&self) -> U0CdnsDsitxDsiTestGenericCtrlR {
        U0CdnsDsitxDsiTestGenericCtrlR::new(((self.bits >> 8) & 0xffff) as u16)
    }
    #[doc = "Bit 10 - SRAM/ROM configuration. VS: timing setting for debug purpose, default is 1'b1."]
    #[inline(always)]
    pub fn u0_cdns_dsitx_vs(&self) -> U0CdnsDsitxVsR {
        U0CdnsDsitxVsR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - SRAM/ROM configuration. VG: timing setting for debug purpose, default is 1'b1."]
    #[inline(always)]
    pub fn u0_cdns_dsitx_vg(&self) -> U0CdnsDsitxVgR {
        U0CdnsDsitxVgR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SRAM/ROM configuration. SLP: sleep enable, high active, default is low."]
    #[inline(always)]
    #[must_use]
    pub fn u0_cdns_dsitx_slp(&mut self) -> U0CdnsDsitxSlpW<Syscfg0Spec> {
        U0CdnsDsitxSlpW::new(self, 0)
    }
    #[doc = "Bit 1 - SRAM/ROM configuration. SD: shutdown enable, high active, default is low."]
    #[inline(always)]
    #[must_use]
    pub fn u0_cdns_dsitx_sd(&mut self) -> U0CdnsDsitxSdW<Syscfg0Spec> {
        U0CdnsDsitxSdW::new(self, 1)
    }
    #[doc = "Bits 2:3 - SRAM/ROM configuration. RTSEL: timing setting for debug purpose, default is 2'b01."]
    #[inline(always)]
    #[must_use]
    pub fn u0_cdns_dsitx_rtsel(&mut self) -> U0CdnsDsitxRtselW<Syscfg0Spec> {
        U0CdnsDsitxRtselW::new(self, 2)
    }
    #[doc = "Bits 4:5 - SRAM/ROM configuration. PTSEL: timing setting for debug purpose, default is 2'b01."]
    #[inline(always)]
    #[must_use]
    pub fn u0_cdns_dsitx_ptsel(&mut self) -> U0CdnsDsitxPtselW<Syscfg0Spec> {
        U0CdnsDsitxPtselW::new(self, 4)
    }
    #[doc = "Bits 6:7 - SRAM/ROM configuration. TRB: timing setting for debug purpose, default is 2'b01."]
    #[inline(always)]
    #[must_use]
    pub fn u0_cdns_dsitx_trb(&mut self) -> U0CdnsDsitxTrbW<Syscfg0Spec> {
        U0CdnsDsitxTrbW::new(self, 6)
    }
    #[doc = "Bits 8:9 - SRAM/ROM configuration. WTSEL: timing setting for debug purpose, default is 2'b01."]
    #[inline(always)]
    #[must_use]
    pub fn u0_cdns_dsitx_wtsel(&mut self) -> U0CdnsDsitxWtselW<Syscfg0Spec> {
        U0CdnsDsitxWtselW::new(self, 8)
    }
    #[doc = "Bit 10 - SRAM/ROM configuration. VS: timing setting for debug purpose, default is 1'b1."]
    #[inline(always)]
    #[must_use]
    pub fn u0_cdns_dsitx_vs(&mut self) -> U0CdnsDsitxVsW<Syscfg0Spec> {
        U0CdnsDsitxVsW::new(self, 10)
    }
    #[doc = "Bit 11 - SRAM/ROM configuration. VG: timing setting for debug purpose, default is 1'b1."]
    #[inline(always)]
    #[must_use]
    pub fn u0_cdns_dsitx_vg(&mut self) -> U0CdnsDsitxVgW<Syscfg0Spec> {
        U0CdnsDsitxVgW::new(self, 11)
    }
}
#[doc = "VOUT SYSCFG 0: dom_vout_sysconsaif_0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syscfg0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`syscfg0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Syscfg0Spec;
impl crate::RegisterSpec for Syscfg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`syscfg0::R`](R) reader structure"]
impl crate::Readable for Syscfg0Spec {}
#[doc = "`write(|w| ..)` method takes [`syscfg0::W`](W) writer structure"]
impl crate::Writable for Syscfg0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets syscfg0 to value 0"]
impl crate::Resettable for Syscfg0Spec {
    const RESET_VALUE: u32 = 0;
}
