#[doc = "Register `syscfg1` reader"]
pub type R = crate::R<Syscfg1Spec>;
#[doc = "Register `syscfg1` writer"]
pub type W = crate::W<Syscfg1Spec>;
#[doc = "Field `u0_cdns_dsitx_dsi_test_generic_status` reader - "]
pub type U0CdnsDsitxDsiTestGenericStatusR = crate::FieldReader<u16>;
#[doc = "Field `u0_cdns_dsitx_dsi_test_generic_status` writer - "]
pub type U0CdnsDsitxDsiTestGenericStatusW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `u0_dc8200_cactive` reader - "]
pub type U0Dc8200CactiveR = crate::BitReader;
#[doc = "Field `u0_dc8200_csysack` reader - "]
pub type U0Dc8200CsysackR = crate::BitReader;
#[doc = "Field `u0_dc8200_csysreq` reader - "]
pub type U0Dc8200CsysreqR = crate::BitReader;
#[doc = "Field `u0_dc8200_csysreq` writer - "]
pub type U0Dc8200CsysreqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_dc8200_disable_ram_clock_gating` reader - "]
pub type U0Dc8200DisableRamClockGatingR = crate::BitReader;
#[doc = "Field `u0_dc8200_disable_ram_clock_gating` writer - "]
pub type U0Dc8200DisableRamClockGatingW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_display_panel_mux_panel_sel` reader - DC8200 Panel - 0: Panel 0, 1: Panel 1"]
pub type U0DisplayPanelMuxPanelSelR = crate::BitReader;
#[doc = "Field `u0_display_panel_mux_panel_sel` writer - DC8200 Panel - 0: Panel 0, 1: Panel 1"]
pub type U0DisplayPanelMuxPanelSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_dsitx_data_mapping_dp_mode` reader - DP color mode - 0: YUV420 CFG1, 1: YUV420 CFG3, 2: YUV422 CFG1 (Reserved), 3: RGB888, 4: RGB666, 5: RGB565"]
pub type U0DsitxDataMappingDpModeR = crate::FieldReader;
#[doc = "Field `u0_dsitx_data_mapping_dp_mode` writer - DP color mode - 0: YUV420 CFG1, 1: YUV420 CFG3, 2: YUV422 CFG1 (Reserved), 3: RGB888, 4: RGB666, 5: RGB565"]
pub type U0DsitxDataMappingDpModeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `u0_dsitx_data_mapping_dpi_dp_sel` reader - DC8200 DP/DPI interface for dsiTx - 0: DPI, 1: DP"]
pub type U0DsitxDataMappingDpiDpSelR = crate::BitReader;
#[doc = "Field `u0_dsitx_data_mapping_dpi_dp_sel` writer - DC8200 DP/DPI interface for dsiTx - 0: DPI, 1: DP"]
pub type U0DsitxDataMappingDpiDpSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_hdmi_data_mapping_dp_bit_depth` reader - DP Bit Depth - 0: 8-bit, 1: 10-bit"]
pub type U0HdmiDataMappingDpBitDepthR = crate::BitReader;
#[doc = "Field `u0_hdmi_data_mapping_dp_bit_depth` writer - DP Bit Depth - 0: 8-bit, 1: 10-bit"]
pub type U0HdmiDataMappingDpBitDepthW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_hdmi_data_mapping_dp_yuv_mode` reader - DP YUV Mode - 0: YUV420, 1: YUV422, 2: YUV444, 3: RGB"]
pub type U0HdmiDataMappingDpYuvModeR = crate::FieldReader;
#[doc = "Field `u0_hdmi_data_mapping_dp_yuv_mode` writer - DP YUV Mode - 0: YUV420, 1: YUV422, 2: YUV444, 3: RGB"]
pub type U0HdmiDataMappingDpYuvModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `u0_hdmi_data_mapping_dpi_bit_depth` reader - DPI Bit Depth - 0: 8-bit, 1: 10-bit, 2: 6-bit CFG1 in DC8200, 3: RGB565 CFG1 in DC8200"]
pub type U0HdmiDataMappingDpiBitDepthR = crate::FieldReader;
#[doc = "Field `u0_hdmi_data_mapping_dpi_bit_depth` writer - DPI Bit Depth - 0: 8-bit, 1: 10-bit, 2: 6-bit CFG1 in DC8200, 3: RGB565 CFG1 in DC8200"]
pub type U0HdmiDataMappingDpiBitDepthW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `u0_hdmi_data_mapping_dpi_dp_sel` reader - DC8200 DP/DPI interface - 0: DPI, 1: DP"]
pub type U0HdmiDataMappingDpiDpSelR = crate::BitReader;
#[doc = "Field `u0_hdmi_data_mapping_dpi_dp_sel` writer - DC8200 DP/DPI interface - 0: DPI, 1: DP"]
pub type U0HdmiDataMappingDpiDpSelW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn u0_cdns_dsitx_dsi_test_generic_status(&self) -> U0CdnsDsitxDsiTestGenericStatusR {
        U0CdnsDsitxDsiTestGenericStatusR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn u0_dc8200_cactive(&self) -> U0Dc8200CactiveR {
        U0Dc8200CactiveR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn u0_dc8200_csysack(&self) -> U0Dc8200CsysackR {
        U0Dc8200CsysackR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn u0_dc8200_csysreq(&self) -> U0Dc8200CsysreqR {
        U0Dc8200CsysreqR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn u0_dc8200_disable_ram_clock_gating(&self) -> U0Dc8200DisableRamClockGatingR {
        U0Dc8200DisableRamClockGatingR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - DC8200 Panel - 0: Panel 0, 1: Panel 1"]
    #[inline(always)]
    pub fn u0_display_panel_mux_panel_sel(&self) -> U0DisplayPanelMuxPanelSelR {
        U0DisplayPanelMuxPanelSelR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:23 - DP color mode - 0: YUV420 CFG1, 1: YUV420 CFG3, 2: YUV422 CFG1 (Reserved), 3: RGB888, 4: RGB666, 5: RGB565"]
    #[inline(always)]
    pub fn u0_dsitx_data_mapping_dp_mode(&self) -> U0DsitxDataMappingDpModeR {
        U0DsitxDataMappingDpModeR::new(((self.bits >> 21) & 7) as u8)
    }
    #[doc = "Bit 24 - DC8200 DP/DPI interface for dsiTx - 0: DPI, 1: DP"]
    #[inline(always)]
    pub fn u0_dsitx_data_mapping_dpi_dp_sel(&self) -> U0DsitxDataMappingDpiDpSelR {
        U0DsitxDataMappingDpiDpSelR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - DP Bit Depth - 0: 8-bit, 1: 10-bit"]
    #[inline(always)]
    pub fn u0_hdmi_data_mapping_dp_bit_depth(&self) -> U0HdmiDataMappingDpBitDepthR {
        U0HdmiDataMappingDpBitDepthR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 26:27 - DP YUV Mode - 0: YUV420, 1: YUV422, 2: YUV444, 3: RGB"]
    #[inline(always)]
    pub fn u0_hdmi_data_mapping_dp_yuv_mode(&self) -> U0HdmiDataMappingDpYuvModeR {
        U0HdmiDataMappingDpYuvModeR::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - DPI Bit Depth - 0: 8-bit, 1: 10-bit, 2: 6-bit CFG1 in DC8200, 3: RGB565 CFG1 in DC8200"]
    #[inline(always)]
    pub fn u0_hdmi_data_mapping_dpi_bit_depth(&self) -> U0HdmiDataMappingDpiBitDepthR {
        U0HdmiDataMappingDpiBitDepthR::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bit 30 - DC8200 DP/DPI interface - 0: DPI, 1: DP"]
    #[inline(always)]
    pub fn u0_hdmi_data_mapping_dpi_dp_sel(&self) -> U0HdmiDataMappingDpiDpSelR {
        U0HdmiDataMappingDpiDpSelR::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    #[must_use]
    pub fn u0_cdns_dsitx_dsi_test_generic_status(
        &mut self,
    ) -> U0CdnsDsitxDsiTestGenericStatusW<Syscfg1Spec> {
        U0CdnsDsitxDsiTestGenericStatusW::new(self, 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    #[must_use]
    pub fn u0_dc8200_csysreq(&mut self) -> U0Dc8200CsysreqW<Syscfg1Spec> {
        U0Dc8200CsysreqW::new(self, 18)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    #[must_use]
    pub fn u0_dc8200_disable_ram_clock_gating(
        &mut self,
    ) -> U0Dc8200DisableRamClockGatingW<Syscfg1Spec> {
        U0Dc8200DisableRamClockGatingW::new(self, 19)
    }
    #[doc = "Bit 20 - DC8200 Panel - 0: Panel 0, 1: Panel 1"]
    #[inline(always)]
    #[must_use]
    pub fn u0_display_panel_mux_panel_sel(&mut self) -> U0DisplayPanelMuxPanelSelW<Syscfg1Spec> {
        U0DisplayPanelMuxPanelSelW::new(self, 20)
    }
    #[doc = "Bits 21:23 - DP color mode - 0: YUV420 CFG1, 1: YUV420 CFG3, 2: YUV422 CFG1 (Reserved), 3: RGB888, 4: RGB666, 5: RGB565"]
    #[inline(always)]
    #[must_use]
    pub fn u0_dsitx_data_mapping_dp_mode(&mut self) -> U0DsitxDataMappingDpModeW<Syscfg1Spec> {
        U0DsitxDataMappingDpModeW::new(self, 21)
    }
    #[doc = "Bit 24 - DC8200 DP/DPI interface for dsiTx - 0: DPI, 1: DP"]
    #[inline(always)]
    #[must_use]
    pub fn u0_dsitx_data_mapping_dpi_dp_sel(&mut self) -> U0DsitxDataMappingDpiDpSelW<Syscfg1Spec> {
        U0DsitxDataMappingDpiDpSelW::new(self, 24)
    }
    #[doc = "Bit 25 - DP Bit Depth - 0: 8-bit, 1: 10-bit"]
    #[inline(always)]
    #[must_use]
    pub fn u0_hdmi_data_mapping_dp_bit_depth(
        &mut self,
    ) -> U0HdmiDataMappingDpBitDepthW<Syscfg1Spec> {
        U0HdmiDataMappingDpBitDepthW::new(self, 25)
    }
    #[doc = "Bits 26:27 - DP YUV Mode - 0: YUV420, 1: YUV422, 2: YUV444, 3: RGB"]
    #[inline(always)]
    #[must_use]
    pub fn u0_hdmi_data_mapping_dp_yuv_mode(&mut self) -> U0HdmiDataMappingDpYuvModeW<Syscfg1Spec> {
        U0HdmiDataMappingDpYuvModeW::new(self, 26)
    }
    #[doc = "Bits 28:29 - DPI Bit Depth - 0: 8-bit, 1: 10-bit, 2: 6-bit CFG1 in DC8200, 3: RGB565 CFG1 in DC8200"]
    #[inline(always)]
    #[must_use]
    pub fn u0_hdmi_data_mapping_dpi_bit_depth(
        &mut self,
    ) -> U0HdmiDataMappingDpiBitDepthW<Syscfg1Spec> {
        U0HdmiDataMappingDpiBitDepthW::new(self, 28)
    }
    #[doc = "Bit 30 - DC8200 DP/DPI interface - 0: DPI, 1: DP"]
    #[inline(always)]
    #[must_use]
    pub fn u0_hdmi_data_mapping_dpi_dp_sel(&mut self) -> U0HdmiDataMappingDpiDpSelW<Syscfg1Spec> {
        U0HdmiDataMappingDpiDpSelW::new(self, 30)
    }
}
#[doc = "VOUT SYSCFG 1: dom_vout_sysconsaif_4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syscfg1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`syscfg1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Syscfg1Spec;
impl crate::RegisterSpec for Syscfg1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`syscfg1::R`](R) reader structure"]
impl crate::Readable for Syscfg1Spec {}
#[doc = "`write(|w| ..)` method takes [`syscfg1::W`](W) writer structure"]
impl crate::Writable for Syscfg1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets syscfg1 to value 0x0008_0000"]
impl crate::Resettable for Syscfg1Spec {
    const RESET_VALUE: u32 = 0x0008_0000;
}
