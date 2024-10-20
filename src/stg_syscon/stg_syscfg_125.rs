#[doc = "Register `stg_syscfg_125` reader"]
pub type R = crate::R<StgSyscfg125Spec>;
#[doc = "Register `stg_syscfg_125` writer"]
pub type W = crate::W<StgSyscfg125Spec>;
#[doc = "Field `u0_pcie_tx_pattern` reader - PCIE TX Pattern"]
pub type U0PcieTxPatternR = crate::FieldReader;
#[doc = "Field `u0_pcie_tx_pattern` writer - PCIE TX Pattern"]
pub type U0PcieTxPatternW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `u0_pcie_usb3_bus_width` reader - PCIE USB3 Bus Width"]
pub type U0PcieUsb3BusWidthR = crate::FieldReader;
#[doc = "Field `u0_pcie_usb3_bus_width` writer - PCIE USB3 Bus Width"]
pub type U0PcieUsb3BusWidthW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `u0_pcie_usb3_phy_enable` reader - PCIE USB3 PHY Enable"]
pub type U0PcieUsb3PhyEnableR = crate::BitReader;
#[doc = "Field `u0_pcie_usb3_phy_enable` writer - PCIE USB3 PHY Enable"]
pub type U0PcieUsb3PhyEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_pcie_usb3_rate` reader - PCIE USB3 Rate"]
pub type U0PcieUsb3RateR = crate::FieldReader;
#[doc = "Field `u0_pcie_usb3_rate` writer - PCIE USB3 Rate"]
pub type U0PcieUsb3RateW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `u0_pcie_usb3_rx_standby` reader - PCIE USB3 RX Standby"]
pub type U0PcieUsb3RxStandbyR = crate::BitReader;
#[doc = "Field `u0_pcie_usb3_rx_standby` writer - PCIE USB3 RX Standby"]
pub type U0PcieUsb3RxStandbyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_pcie_xwdecerr` reader - PCIE XWDECERR"]
pub type U0PcieXwdecerrR = crate::BitReader;
#[doc = "Field `u0_pcie_xwerrclr` reader - PCIE XWERRCLR"]
pub type U0PcieXwerrclrR = crate::BitReader;
#[doc = "Field `u0_pcie_xwerrclr` writer - PCIE XWERRCLR"]
pub type U0PcieXwerrclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_pcie_xwslverr` reader - PCIE XWSLVERR"]
pub type U0PcieXwslverrR = crate::BitReader;
#[doc = "Field `u0_sec_top_sramcfg_slp` reader - SRAM/ROM configuration. SLP: sleep enable, high active, default is low."]
pub type U0SecTopSramcfgSlpR = crate::BitReader;
#[doc = "Field `u0_sec_top_sramcfg_slp` writer - SRAM/ROM configuration. SLP: sleep enable, high active, default is low."]
pub type U0SecTopSramcfgSlpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_sec_top_sramcfg_sd` reader - SRAM/ROM configuration. SD: shutdown enable, high active, default is low."]
pub type U0SecTopSramcfgSdR = crate::BitReader;
#[doc = "Field `u0_sec_top_sramcfg_sd` writer - SRAM/ROM configuration. SD: shutdown enable, high active, default is low."]
pub type U0SecTopSramcfgSdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_sec_top_sramcfg_rtsel` reader - SRAM/ROM configuration. RTSEL: timing setting for debug purpose, default is 2'b01."]
pub type U0SecTopSramcfgRtselR = crate::FieldReader;
#[doc = "Field `u0_sec_top_sramcfg_rtsel` writer - SRAM/ROM configuration. RTSEL: timing setting for debug purpose, default is 2'b01."]
pub type U0SecTopSramcfgRtselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `u0_sec_top_sramcfg_ptsel` reader - SRAM/ROM configuration. PTSEL: timing setting for debug purpose, default is 2'b01."]
pub type U0SecTopSramcfgPtselR = crate::FieldReader;
#[doc = "Field `u0_sec_top_sramcfg_ptsel` writer - SRAM/ROM configuration. PTSEL: timing setting for debug purpose, default is 2'b01."]
pub type U0SecTopSramcfgPtselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `u0_sec_top_sramcfg_trb` reader - SRAM/ROM configuration. TRB: timing setting for debug purpose, default is 2'b01."]
pub type U0SecTopSramcfgTrbR = crate::FieldReader;
#[doc = "Field `u0_sec_top_sramcfg_trb` writer - SRAM/ROM configuration. TRB: timing setting for debug purpose, default is 2'b01."]
pub type U0SecTopSramcfgTrbW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `u0_sec_top_sramcfg_wtsel` reader - SRAM/ROM configuration. WTSEL: timing setting for debug purpose, default is 2'b01."]
pub type U0SecTopSramcfgWtselR = crate::FieldReader;
#[doc = "Field `u0_sec_top_sramcfg_wtsel` writer - SRAM/ROM configuration. WTSEL: timing setting for debug purpose, default is 2'b01."]
pub type U0SecTopSramcfgWtselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `u0_sec_top_sramcfg_vs` reader - SRAM/ROM configuration. VS: timing setting for debug purpose, default is 1'b1."]
pub type U0SecTopSramcfgVsR = crate::BitReader;
#[doc = "Field `u0_sec_top_sramcfg_vs` writer - SRAM/ROM configuration. VS: timing setting for debug purpose, default is 1'b1."]
pub type U0SecTopSramcfgVsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_sec_top_sramcfg_vg` reader - SRAM/ROM configuration. VG: timing setting for debug purpose, default is 1'b1."]
pub type U0SecTopSramcfgVgR = crate::BitReader;
#[doc = "Field `u0_sec_top_sramcfg_vg` writer - SRAM/ROM configuration. VG: timing setting for debug purpose, default is 1'b1."]
pub type U0SecTopSramcfgVgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_pcie_align_detect` reader - PCIE Align Detect"]
pub type U0PcieAlignDetectR = crate::BitReader;
impl R {
    #[doc = "Bits 0:1 - PCIE TX Pattern"]
    #[inline(always)]
    pub fn u0_pcie_tx_pattern(&self) -> U0PcieTxPatternR {
        U0PcieTxPatternR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - PCIE USB3 Bus Width"]
    #[inline(always)]
    pub fn u0_pcie_usb3_bus_width(&self) -> U0PcieUsb3BusWidthR {
        U0PcieUsb3BusWidthR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - PCIE USB3 PHY Enable"]
    #[inline(always)]
    pub fn u0_pcie_usb3_phy_enable(&self) -> U0PcieUsb3PhyEnableR {
        U0PcieUsb3PhyEnableR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - PCIE USB3 Rate"]
    #[inline(always)]
    pub fn u0_pcie_usb3_rate(&self) -> U0PcieUsb3RateR {
        U0PcieUsb3RateR::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - PCIE USB3 RX Standby"]
    #[inline(always)]
    pub fn u0_pcie_usb3_rx_standby(&self) -> U0PcieUsb3RxStandbyR {
        U0PcieUsb3RxStandbyR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - PCIE XWDECERR"]
    #[inline(always)]
    pub fn u0_pcie_xwdecerr(&self) -> U0PcieXwdecerrR {
        U0PcieXwdecerrR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - PCIE XWERRCLR"]
    #[inline(always)]
    pub fn u0_pcie_xwerrclr(&self) -> U0PcieXwerrclrR {
        U0PcieXwerrclrR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - PCIE XWSLVERR"]
    #[inline(always)]
    pub fn u0_pcie_xwslverr(&self) -> U0PcieXwslverrR {
        U0PcieXwslverrR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - SRAM/ROM configuration. SLP: sleep enable, high active, default is low."]
    #[inline(always)]
    pub fn u0_sec_top_sramcfg_slp(&self) -> U0SecTopSramcfgSlpR {
        U0SecTopSramcfgSlpR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - SRAM/ROM configuration. SD: shutdown enable, high active, default is low."]
    #[inline(always)]
    pub fn u0_sec_top_sramcfg_sd(&self) -> U0SecTopSramcfgSdR {
        U0SecTopSramcfgSdR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:14 - SRAM/ROM configuration. RTSEL: timing setting for debug purpose, default is 2'b01."]
    #[inline(always)]
    pub fn u0_sec_top_sramcfg_rtsel(&self) -> U0SecTopSramcfgRtselR {
        U0SecTopSramcfgRtselR::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bits 15:16 - SRAM/ROM configuration. PTSEL: timing setting for debug purpose, default is 2'b01."]
    #[inline(always)]
    pub fn u0_sec_top_sramcfg_ptsel(&self) -> U0SecTopSramcfgPtselR {
        U0SecTopSramcfgPtselR::new(((self.bits >> 15) & 3) as u8)
    }
    #[doc = "Bits 17:18 - SRAM/ROM configuration. TRB: timing setting for debug purpose, default is 2'b01."]
    #[inline(always)]
    pub fn u0_sec_top_sramcfg_trb(&self) -> U0SecTopSramcfgTrbR {
        U0SecTopSramcfgTrbR::new(((self.bits >> 17) & 3) as u8)
    }
    #[doc = "Bits 19:20 - SRAM/ROM configuration. WTSEL: timing setting for debug purpose, default is 2'b01."]
    #[inline(always)]
    pub fn u0_sec_top_sramcfg_wtsel(&self) -> U0SecTopSramcfgWtselR {
        U0SecTopSramcfgWtselR::new(((self.bits >> 19) & 3) as u8)
    }
    #[doc = "Bit 21 - SRAM/ROM configuration. VS: timing setting for debug purpose, default is 1'b1."]
    #[inline(always)]
    pub fn u0_sec_top_sramcfg_vs(&self) -> U0SecTopSramcfgVsR {
        U0SecTopSramcfgVsR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - SRAM/ROM configuration. VG: timing setting for debug purpose, default is 1'b1."]
    #[inline(always)]
    pub fn u0_sec_top_sramcfg_vg(&self) -> U0SecTopSramcfgVgR {
        U0SecTopSramcfgVgR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - PCIE Align Detect"]
    #[inline(always)]
    pub fn u0_pcie_align_detect(&self) -> U0PcieAlignDetectR {
        U0PcieAlignDetectR::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - PCIE TX Pattern"]
    #[inline(always)]
    #[must_use]
    pub fn u0_pcie_tx_pattern(&mut self) -> U0PcieTxPatternW<StgSyscfg125Spec> {
        U0PcieTxPatternW::new(self, 0)
    }
    #[doc = "Bits 2:3 - PCIE USB3 Bus Width"]
    #[inline(always)]
    #[must_use]
    pub fn u0_pcie_usb3_bus_width(&mut self) -> U0PcieUsb3BusWidthW<StgSyscfg125Spec> {
        U0PcieUsb3BusWidthW::new(self, 2)
    }
    #[doc = "Bit 4 - PCIE USB3 PHY Enable"]
    #[inline(always)]
    #[must_use]
    pub fn u0_pcie_usb3_phy_enable(&mut self) -> U0PcieUsb3PhyEnableW<StgSyscfg125Spec> {
        U0PcieUsb3PhyEnableW::new(self, 4)
    }
    #[doc = "Bits 5:6 - PCIE USB3 Rate"]
    #[inline(always)]
    #[must_use]
    pub fn u0_pcie_usb3_rate(&mut self) -> U0PcieUsb3RateW<StgSyscfg125Spec> {
        U0PcieUsb3RateW::new(self, 5)
    }
    #[doc = "Bit 7 - PCIE USB3 RX Standby"]
    #[inline(always)]
    #[must_use]
    pub fn u0_pcie_usb3_rx_standby(&mut self) -> U0PcieUsb3RxStandbyW<StgSyscfg125Spec> {
        U0PcieUsb3RxStandbyW::new(self, 7)
    }
    #[doc = "Bit 9 - PCIE XWERRCLR"]
    #[inline(always)]
    #[must_use]
    pub fn u0_pcie_xwerrclr(&mut self) -> U0PcieXwerrclrW<StgSyscfg125Spec> {
        U0PcieXwerrclrW::new(self, 9)
    }
    #[doc = "Bit 11 - SRAM/ROM configuration. SLP: sleep enable, high active, default is low."]
    #[inline(always)]
    #[must_use]
    pub fn u0_sec_top_sramcfg_slp(&mut self) -> U0SecTopSramcfgSlpW<StgSyscfg125Spec> {
        U0SecTopSramcfgSlpW::new(self, 11)
    }
    #[doc = "Bit 12 - SRAM/ROM configuration. SD: shutdown enable, high active, default is low."]
    #[inline(always)]
    #[must_use]
    pub fn u0_sec_top_sramcfg_sd(&mut self) -> U0SecTopSramcfgSdW<StgSyscfg125Spec> {
        U0SecTopSramcfgSdW::new(self, 12)
    }
    #[doc = "Bits 13:14 - SRAM/ROM configuration. RTSEL: timing setting for debug purpose, default is 2'b01."]
    #[inline(always)]
    #[must_use]
    pub fn u0_sec_top_sramcfg_rtsel(&mut self) -> U0SecTopSramcfgRtselW<StgSyscfg125Spec> {
        U0SecTopSramcfgRtselW::new(self, 13)
    }
    #[doc = "Bits 15:16 - SRAM/ROM configuration. PTSEL: timing setting for debug purpose, default is 2'b01."]
    #[inline(always)]
    #[must_use]
    pub fn u0_sec_top_sramcfg_ptsel(&mut self) -> U0SecTopSramcfgPtselW<StgSyscfg125Spec> {
        U0SecTopSramcfgPtselW::new(self, 15)
    }
    #[doc = "Bits 17:18 - SRAM/ROM configuration. TRB: timing setting for debug purpose, default is 2'b01."]
    #[inline(always)]
    #[must_use]
    pub fn u0_sec_top_sramcfg_trb(&mut self) -> U0SecTopSramcfgTrbW<StgSyscfg125Spec> {
        U0SecTopSramcfgTrbW::new(self, 17)
    }
    #[doc = "Bits 19:20 - SRAM/ROM configuration. WTSEL: timing setting for debug purpose, default is 2'b01."]
    #[inline(always)]
    #[must_use]
    pub fn u0_sec_top_sramcfg_wtsel(&mut self) -> U0SecTopSramcfgWtselW<StgSyscfg125Spec> {
        U0SecTopSramcfgWtselW::new(self, 19)
    }
    #[doc = "Bit 21 - SRAM/ROM configuration. VS: timing setting for debug purpose, default is 1'b1."]
    #[inline(always)]
    #[must_use]
    pub fn u0_sec_top_sramcfg_vs(&mut self) -> U0SecTopSramcfgVsW<StgSyscfg125Spec> {
        U0SecTopSramcfgVsW::new(self, 21)
    }
    #[doc = "Bit 22 - SRAM/ROM configuration. VG: timing setting for debug purpose, default is 1'b1."]
    #[inline(always)]
    #[must_use]
    pub fn u0_sec_top_sramcfg_vg(&mut self) -> U0SecTopSramcfgVgW<StgSyscfg125Spec> {
        U0SecTopSramcfgVgW::new(self, 22)
    }
}
#[doc = "STG SYSCONSAIF SYSCFG 500\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_syscfg_125::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stg_syscfg_125::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StgSyscfg125Spec;
impl crate::RegisterSpec for StgSyscfg125Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_syscfg_125::R`](R) reader structure"]
impl crate::Readable for StgSyscfg125Spec {}
#[doc = "`write(|w| ..)` method takes [`stg_syscfg_125::W`](W) writer structure"]
impl crate::Writable for StgSyscfg125Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets stg_syscfg_125 to value 0x006a_a008"]
impl crate::Resettable for StgSyscfg125Spec {
    const RESET_VALUE: u32 = 0x006a_a008;
}
