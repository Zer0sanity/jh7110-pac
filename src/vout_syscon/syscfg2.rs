#[doc = "Register `syscfg2` reader"]
pub type R = crate::R<Syscfg2Spec>;
#[doc = "Register `syscfg2` writer"]
pub type W = crate::W<Syscfg2Spec>;
#[doc = "Field `u0_lcd_data_mapping_dp_rgb_fmt` reader - RGB format in DP data - 0: RGB888, 1: RGB666, 2: RGB565"]
pub type U0LcdDataMappingDpRgbFmtR = crate::FieldReader;
#[doc = "Field `u0_lcd_data_mapping_dp_rgb_fmt` writer - RGB format in DP data - 0: RGB888, 1: RGB666, 2: RGB565"]
pub type U0LcdDataMappingDpRgbFmtW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `u0_lcd_data_mapping_dpi_dp_sel` reader - DPI or DP - 0: DPI, 1: DP"]
pub type U0LcdDataMappingDpiDpSelR = crate::BitReader;
#[doc = "Field `u0_lcd_data_mapping_dpi_dp_sel` writer - DPI or DP - 0: DPI, 1: DP"]
pub type U0LcdDataMappingDpiDpSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u1_display_panel_sel` reader - DC8200 panel - 0: Panel 0, 1: Panel 1"]
pub type U1DisplayPanelSelR = crate::BitReader;
#[doc = "Field `u1_display_panel_sel` writer - DC8200 panel - 0: Panel 0, 1: Panel 1"]
pub type U1DisplayPanelSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u2_display_panel_sel` reader - DC8200 panel - 0: Panel 0, 1: Panel 1"]
pub type U2DisplayPanelSelR = crate::BitReader;
#[doc = "Field `u2_display_panel_sel` writer - DC8200 panel - 0: Panel 0, 1: Panel 1"]
pub type U2DisplayPanelSelW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - RGB format in DP data - 0: RGB888, 1: RGB666, 2: RGB565"]
    #[inline(always)]
    pub fn u0_lcd_data_mapping_dp_rgb_fmt(&self) -> U0LcdDataMappingDpRgbFmtR {
        U0LcdDataMappingDpRgbFmtR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - DPI or DP - 0: DPI, 1: DP"]
    #[inline(always)]
    pub fn u0_lcd_data_mapping_dpi_dp_sel(&self) -> U0LcdDataMappingDpiDpSelR {
        U0LcdDataMappingDpiDpSelR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DC8200 panel - 0: Panel 0, 1: Panel 1"]
    #[inline(always)]
    pub fn u1_display_panel_sel(&self) -> U1DisplayPanelSelR {
        U1DisplayPanelSelR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - DC8200 panel - 0: Panel 0, 1: Panel 1"]
    #[inline(always)]
    pub fn u2_display_panel_sel(&self) -> U2DisplayPanelSelR {
        U2DisplayPanelSelR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - RGB format in DP data - 0: RGB888, 1: RGB666, 2: RGB565"]
    #[inline(always)]
    #[must_use]
    pub fn u0_lcd_data_mapping_dp_rgb_fmt(&mut self) -> U0LcdDataMappingDpRgbFmtW<Syscfg2Spec> {
        U0LcdDataMappingDpRgbFmtW::new(self, 0)
    }
    #[doc = "Bit 2 - DPI or DP - 0: DPI, 1: DP"]
    #[inline(always)]
    #[must_use]
    pub fn u0_lcd_data_mapping_dpi_dp_sel(&mut self) -> U0LcdDataMappingDpiDpSelW<Syscfg2Spec> {
        U0LcdDataMappingDpiDpSelW::new(self, 2)
    }
    #[doc = "Bit 3 - DC8200 panel - 0: Panel 0, 1: Panel 1"]
    #[inline(always)]
    #[must_use]
    pub fn u1_display_panel_sel(&mut self) -> U1DisplayPanelSelW<Syscfg2Spec> {
        U1DisplayPanelSelW::new(self, 3)
    }
    #[doc = "Bit 4 - DC8200 panel - 0: Panel 0, 1: Panel 1"]
    #[inline(always)]
    #[must_use]
    pub fn u2_display_panel_sel(&mut self) -> U2DisplayPanelSelW<Syscfg2Spec> {
        U2DisplayPanelSelW::new(self, 4)
    }
}
#[doc = "VOUT SYSCFG 2: dom_vout_sysconsaif_8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syscfg2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`syscfg2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Syscfg2Spec;
impl crate::RegisterSpec for Syscfg2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`syscfg2::R`](R) reader structure"]
impl crate::Readable for Syscfg2Spec {}
#[doc = "`write(|w| ..)` method takes [`syscfg2::W`](W) writer structure"]
impl crate::Writable for Syscfg2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets syscfg2 to value 0"]
impl crate::Resettable for Syscfg2Spec {
    const RESET_VALUE: u32 = 0;
}
