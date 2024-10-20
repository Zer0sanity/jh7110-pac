#[doc = "Register `sys_syscfg5` reader"]
pub type R = crate::R<SysSyscfg5Spec>;
#[doc = "Register `sys_syscfg5` writer"]
pub type W = crate::W<SysSyscfg5Spec>;
#[doc = "Field `u0_cdns_qspi_scfg_sram_config_slp` reader - SRAM/ROM configuration. SLP: sleep enable, high active, default is low."]
pub type U0CdnsQspiScfgSramConfigSlpR = crate::BitReader;
#[doc = "Field `u0_cdns_qspi_scfg_sram_config_slp` writer - SRAM/ROM configuration. SLP: sleep enable, high active, default is low."]
pub type U0CdnsQspiScfgSramConfigSlpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_cdns_qspi_scfg_sram_config_sd` reader - SRAM/ROM configuration. SD: shutdown enable, high active, default is low."]
pub type U0CdnsQspiScfgSramConfigSdR = crate::BitReader;
#[doc = "Field `u0_cdns_qspi_scfg_sram_config_sd` writer - SRAM/ROM configuration. SD: shutdown enable, high active, default is low."]
pub type U0CdnsQspiScfgSramConfigSdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_cdns_qspi_scfg_sram_config_rtsel` reader - SRAM/ROM configuration. RTSEL: timing setting for debug purpose, default is 2'b01."]
pub type U0CdnsQspiScfgSramConfigRtselR = crate::FieldReader;
#[doc = "Field `u0_cdns_qspi_scfg_sram_config_rtsel` writer - SRAM/ROM configuration. RTSEL: timing setting for debug purpose, default is 2'b01."]
pub type U0CdnsQspiScfgSramConfigRtselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `u0_cdns_qspi_scfg_sram_config_ptsel` reader - SRAM/ROM configuration. PTSEL: timing setting for debug purpose, default is 2'b01."]
pub type U0CdnsQspiScfgSramConfigPtselR = crate::FieldReader;
#[doc = "Field `u0_cdns_qspi_scfg_sram_config_ptsel` writer - SRAM/ROM configuration. PTSEL: timing setting for debug purpose, default is 2'b01."]
pub type U0CdnsQspiScfgSramConfigPtselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `u0_cdns_qspi_scfg_sram_config_trb` reader - SRAM/ROM configuration. TRB: timing setting for debug purpose, default is 2'b01."]
pub type U0CdnsQspiScfgSramConfigTrbR = crate::FieldReader;
#[doc = "Field `u0_cdns_qspi_scfg_sram_config_trb` writer - SRAM/ROM configuration. TRB: timing setting for debug purpose, default is 2'b01."]
pub type U0CdnsQspiScfgSramConfigTrbW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `u0_cdns_qspi_scfg_sram_config_wtsel` reader - SRAM/ROM configuration. WTSEL: timing setting for debug purpose, default is 2'b01."]
pub type U0CdnsQspiScfgSramConfigWtselR = crate::FieldReader;
#[doc = "Field `u0_cdns_qspi_scfg_sram_config_wtsel` writer - SRAM/ROM configuration. WTSEL: timing setting for debug purpose, default is 2'b01."]
pub type U0CdnsQspiScfgSramConfigWtselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `u0_cdns_qspi_scfg_sram_config_vs` reader - SRAM/ROM configuration. VS: timing setting for debug purpose, default is 1'b1."]
pub type U0CdnsQspiScfgSramConfigVsR = crate::BitReader;
#[doc = "Field `u0_cdns_qspi_scfg_sram_config_vs` writer - SRAM/ROM configuration. VS: timing setting for debug purpose, default is 1'b1."]
pub type U0CdnsQspiScfgSramConfigVsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_cdns_qspi_scfg_sram_config_vg` reader - SRAM/ROM configuration. VG: timing setting for debug purpose, default is 1'b1."]
pub type U0CdnsQspiScfgSramConfigVgR = crate::BitReader;
#[doc = "Field `u0_cdns_qspi_scfg_sram_config_vg` writer - SRAM/ROM configuration. VG: timing setting for debug purpose, default is 1'b1."]
pub type U0CdnsQspiScfgSramConfigVgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_cdns_spdif_scfg_sram_config_slp` reader - SRAM/ROM configuration. SLP: sleep enable, high active, default is low."]
pub type U0CdnsSpdifScfgSramConfigSlpR = crate::BitReader;
#[doc = "Field `u0_cdns_spdif_scfg_sram_config_slp` writer - SRAM/ROM configuration. SLP: sleep enable, high active, default is low."]
pub type U0CdnsSpdifScfgSramConfigSlpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_cdns_spdif_scfg_sram_config_sd` reader - SRAM/ROM configuration. SD: shutdown enable, high active, default is low."]
pub type U0CdnsSpdifScfgSramConfigSdR = crate::BitReader;
#[doc = "Field `u0_cdns_spdif_scfg_sram_config_sd` writer - SRAM/ROM configuration. SD: shutdown enable, high active, default is low."]
pub type U0CdnsSpdifScfgSramConfigSdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_cdns_spdif_scfg_sram_config_rtsel` reader - SRAM/ROM configuration. RTSEL: timing setting for debug purpose, default is 2'b01."]
pub type U0CdnsSpdifScfgSramConfigRtselR = crate::FieldReader;
#[doc = "Field `u0_cdns_spdif_scfg_sram_config_rtsel` writer - SRAM/ROM configuration. RTSEL: timing setting for debug purpose, default is 2'b01."]
pub type U0CdnsSpdifScfgSramConfigRtselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `u0_cdns_spdif_scfg_sram_config_ptsel` reader - SRAM/ROM configuration. PTSEL: timing setting for debug purpose, default is 2'b01."]
pub type U0CdnsSpdifScfgSramConfigPtselR = crate::FieldReader;
#[doc = "Field `u0_cdns_spdif_scfg_sram_config_ptsel` writer - SRAM/ROM configuration. PTSEL: timing setting for debug purpose, default is 2'b01."]
pub type U0CdnsSpdifScfgSramConfigPtselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `u0_cdns_spdif_scfg_sram_config_trb` reader - SRAM/ROM configuration. TRB: timing setting for debug purpose, default is 2'b01."]
pub type U0CdnsSpdifScfgSramConfigTrbR = crate::FieldReader;
#[doc = "Field `u0_cdns_spdif_scfg_sram_config_trb` writer - SRAM/ROM configuration. TRB: timing setting for debug purpose, default is 2'b01."]
pub type U0CdnsSpdifScfgSramConfigTrbW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `u0_cdns_spdif_scfg_sram_config_wtsel` reader - SRAM/ROM configuration. WTSEL: timing setting for debug purpose, default is 2'b01."]
pub type U0CdnsSpdifScfgSramConfigWtselR = crate::FieldReader;
#[doc = "Field `u0_cdns_spdif_scfg_sram_config_wtsel` writer - SRAM/ROM configuration. WTSEL: timing setting for debug purpose, default is 2'b01."]
pub type U0CdnsSpdifScfgSramConfigWtselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `u0_cdns_spdif_scfg_sram_config_vs` reader - SRAM/ROM configuration. VS: timing setting for debug purpose, default is 1'b1."]
pub type U0CdnsSpdifScfgSramConfigVsR = crate::BitReader;
#[doc = "Field `u0_cdns_spdif_scfg_sram_config_vs` writer - SRAM/ROM configuration. VS: timing setting for debug purpose, default is 1'b1."]
pub type U0CdnsSpdifScfgSramConfigVsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_cdns_spdif_scfg_sram_config_vg` reader - SRAM/ROM configuration. VG: timing setting for debug purpose, default is 1'b1."]
pub type U0CdnsSpdifScfgSramConfigVgR = crate::BitReader;
#[doc = "Field `u0_cdns_spdif_scfg_sram_config_vg` writer - SRAM/ROM configuration. VG: timing setting for debug purpose, default is 1'b1."]
pub type U0CdnsSpdifScfgSramConfigVgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `spdif_trmodeo` reader - 1 for transmitter 0 for receiver"]
pub type SpdifTrmodeoR = crate::BitReader;
#[doc = "Field `i2c_ic_en` reader - I2C interface enable"]
pub type I2cIcEnR = crate::BitReader;
#[doc = "Field `sdio_data_strobe_phase_ctrl` reader - Data strobe delay chain select"]
pub type SdioDataStrobePhaseCtrlR = crate::FieldReader;
#[doc = "Field `sdio_data_strobe_phase_ctrl` writer - Data strobe delay chain select"]
pub type SdioDataStrobePhaseCtrlW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `sdio_hbig_endian` reader - AHB bus interface endianness: 1: Big-endian AHB bus interface, 0: Little-endian AHB bus interface"]
pub type SdioHbigEndianR = crate::BitReader;
#[doc = "Field `sdio_hbig_endian` writer - AHB bus interface endianness: 1: Big-endian AHB bus interface, 0: Little-endian AHB bus interface"]
pub type SdioHbigEndianW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SRAM/ROM configuration. SLP: sleep enable, high active, default is low."]
    #[inline(always)]
    pub fn u0_cdns_qspi_scfg_sram_config_slp(&self) -> U0CdnsQspiScfgSramConfigSlpR {
        U0CdnsQspiScfgSramConfigSlpR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SRAM/ROM configuration. SD: shutdown enable, high active, default is low."]
    #[inline(always)]
    pub fn u0_cdns_qspi_scfg_sram_config_sd(&self) -> U0CdnsQspiScfgSramConfigSdR {
        U0CdnsQspiScfgSramConfigSdR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - SRAM/ROM configuration. RTSEL: timing setting for debug purpose, default is 2'b01."]
    #[inline(always)]
    pub fn u0_cdns_qspi_scfg_sram_config_rtsel(&self) -> U0CdnsQspiScfgSramConfigRtselR {
        U0CdnsQspiScfgSramConfigRtselR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - SRAM/ROM configuration. PTSEL: timing setting for debug purpose, default is 2'b01."]
    #[inline(always)]
    pub fn u0_cdns_qspi_scfg_sram_config_ptsel(&self) -> U0CdnsQspiScfgSramConfigPtselR {
        U0CdnsQspiScfgSramConfigPtselR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - SRAM/ROM configuration. TRB: timing setting for debug purpose, default is 2'b01."]
    #[inline(always)]
    pub fn u0_cdns_qspi_scfg_sram_config_trb(&self) -> U0CdnsQspiScfgSramConfigTrbR {
        U0CdnsQspiScfgSramConfigTrbR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - SRAM/ROM configuration. WTSEL: timing setting for debug purpose, default is 2'b01."]
    #[inline(always)]
    pub fn u0_cdns_qspi_scfg_sram_config_wtsel(&self) -> U0CdnsQspiScfgSramConfigWtselR {
        U0CdnsQspiScfgSramConfigWtselR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - SRAM/ROM configuration. VS: timing setting for debug purpose, default is 1'b1."]
    #[inline(always)]
    pub fn u0_cdns_qspi_scfg_sram_config_vs(&self) -> U0CdnsQspiScfgSramConfigVsR {
        U0CdnsQspiScfgSramConfigVsR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - SRAM/ROM configuration. VG: timing setting for debug purpose, default is 1'b1."]
    #[inline(always)]
    pub fn u0_cdns_qspi_scfg_sram_config_vg(&self) -> U0CdnsQspiScfgSramConfigVgR {
        U0CdnsQspiScfgSramConfigVgR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - SRAM/ROM configuration. SLP: sleep enable, high active, default is low."]
    #[inline(always)]
    pub fn u0_cdns_spdif_scfg_sram_config_slp(&self) -> U0CdnsSpdifScfgSramConfigSlpR {
        U0CdnsSpdifScfgSramConfigSlpR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - SRAM/ROM configuration. SD: shutdown enable, high active, default is low."]
    #[inline(always)]
    pub fn u0_cdns_spdif_scfg_sram_config_sd(&self) -> U0CdnsSpdifScfgSramConfigSdR {
        U0CdnsSpdifScfgSramConfigSdR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:15 - SRAM/ROM configuration. RTSEL: timing setting for debug purpose, default is 2'b01."]
    #[inline(always)]
    pub fn u0_cdns_spdif_scfg_sram_config_rtsel(&self) -> U0CdnsSpdifScfgSramConfigRtselR {
        U0CdnsSpdifScfgSramConfigRtselR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - SRAM/ROM configuration. PTSEL: timing setting for debug purpose, default is 2'b01."]
    #[inline(always)]
    pub fn u0_cdns_spdif_scfg_sram_config_ptsel(&self) -> U0CdnsSpdifScfgSramConfigPtselR {
        U0CdnsSpdifScfgSramConfigPtselR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - SRAM/ROM configuration. TRB: timing setting for debug purpose, default is 2'b01."]
    #[inline(always)]
    pub fn u0_cdns_spdif_scfg_sram_config_trb(&self) -> U0CdnsSpdifScfgSramConfigTrbR {
        U0CdnsSpdifScfgSramConfigTrbR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - SRAM/ROM configuration. WTSEL: timing setting for debug purpose, default is 2'b01."]
    #[inline(always)]
    pub fn u0_cdns_spdif_scfg_sram_config_wtsel(&self) -> U0CdnsSpdifScfgSramConfigWtselR {
        U0CdnsSpdifScfgSramConfigWtselR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 22 - SRAM/ROM configuration. VS: timing setting for debug purpose, default is 1'b1."]
    #[inline(always)]
    pub fn u0_cdns_spdif_scfg_sram_config_vs(&self) -> U0CdnsSpdifScfgSramConfigVsR {
        U0CdnsSpdifScfgSramConfigVsR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - SRAM/ROM configuration. VG: timing setting for debug purpose, default is 1'b1."]
    #[inline(always)]
    pub fn u0_cdns_spdif_scfg_sram_config_vg(&self) -> U0CdnsSpdifScfgSramConfigVgR {
        U0CdnsSpdifScfgSramConfigVgR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - 1 for transmitter 0 for receiver"]
    #[inline(always)]
    pub fn spdif_trmodeo(&self) -> SpdifTrmodeoR {
        SpdifTrmodeoR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - I2C interface enable"]
    #[inline(always)]
    pub fn i2c_ic_en(&self) -> I2cIcEnR {
        I2cIcEnR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 26:30 - Data strobe delay chain select"]
    #[inline(always)]
    pub fn sdio_data_strobe_phase_ctrl(&self) -> SdioDataStrobePhaseCtrlR {
        SdioDataStrobePhaseCtrlR::new(((self.bits >> 26) & 0x1f) as u8)
    }
    #[doc = "Bit 31 - AHB bus interface endianness: 1: Big-endian AHB bus interface, 0: Little-endian AHB bus interface"]
    #[inline(always)]
    pub fn sdio_hbig_endian(&self) -> SdioHbigEndianR {
        SdioHbigEndianR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SRAM/ROM configuration. SLP: sleep enable, high active, default is low."]
    #[inline(always)]
    #[must_use]
    pub fn u0_cdns_qspi_scfg_sram_config_slp(
        &mut self,
    ) -> U0CdnsQspiScfgSramConfigSlpW<SysSyscfg5Spec> {
        U0CdnsQspiScfgSramConfigSlpW::new(self, 0)
    }
    #[doc = "Bit 1 - SRAM/ROM configuration. SD: shutdown enable, high active, default is low."]
    #[inline(always)]
    #[must_use]
    pub fn u0_cdns_qspi_scfg_sram_config_sd(
        &mut self,
    ) -> U0CdnsQspiScfgSramConfigSdW<SysSyscfg5Spec> {
        U0CdnsQspiScfgSramConfigSdW::new(self, 1)
    }
    #[doc = "Bits 2:3 - SRAM/ROM configuration. RTSEL: timing setting for debug purpose, default is 2'b01."]
    #[inline(always)]
    #[must_use]
    pub fn u0_cdns_qspi_scfg_sram_config_rtsel(
        &mut self,
    ) -> U0CdnsQspiScfgSramConfigRtselW<SysSyscfg5Spec> {
        U0CdnsQspiScfgSramConfigRtselW::new(self, 2)
    }
    #[doc = "Bits 4:5 - SRAM/ROM configuration. PTSEL: timing setting for debug purpose, default is 2'b01."]
    #[inline(always)]
    #[must_use]
    pub fn u0_cdns_qspi_scfg_sram_config_ptsel(
        &mut self,
    ) -> U0CdnsQspiScfgSramConfigPtselW<SysSyscfg5Spec> {
        U0CdnsQspiScfgSramConfigPtselW::new(self, 4)
    }
    #[doc = "Bits 6:7 - SRAM/ROM configuration. TRB: timing setting for debug purpose, default is 2'b01."]
    #[inline(always)]
    #[must_use]
    pub fn u0_cdns_qspi_scfg_sram_config_trb(
        &mut self,
    ) -> U0CdnsQspiScfgSramConfigTrbW<SysSyscfg5Spec> {
        U0CdnsQspiScfgSramConfigTrbW::new(self, 6)
    }
    #[doc = "Bits 8:9 - SRAM/ROM configuration. WTSEL: timing setting for debug purpose, default is 2'b01."]
    #[inline(always)]
    #[must_use]
    pub fn u0_cdns_qspi_scfg_sram_config_wtsel(
        &mut self,
    ) -> U0CdnsQspiScfgSramConfigWtselW<SysSyscfg5Spec> {
        U0CdnsQspiScfgSramConfigWtselW::new(self, 8)
    }
    #[doc = "Bit 10 - SRAM/ROM configuration. VS: timing setting for debug purpose, default is 1'b1."]
    #[inline(always)]
    #[must_use]
    pub fn u0_cdns_qspi_scfg_sram_config_vs(
        &mut self,
    ) -> U0CdnsQspiScfgSramConfigVsW<SysSyscfg5Spec> {
        U0CdnsQspiScfgSramConfigVsW::new(self, 10)
    }
    #[doc = "Bit 11 - SRAM/ROM configuration. VG: timing setting for debug purpose, default is 1'b1."]
    #[inline(always)]
    #[must_use]
    pub fn u0_cdns_qspi_scfg_sram_config_vg(
        &mut self,
    ) -> U0CdnsQspiScfgSramConfigVgW<SysSyscfg5Spec> {
        U0CdnsQspiScfgSramConfigVgW::new(self, 11)
    }
    #[doc = "Bit 12 - SRAM/ROM configuration. SLP: sleep enable, high active, default is low."]
    #[inline(always)]
    #[must_use]
    pub fn u0_cdns_spdif_scfg_sram_config_slp(
        &mut self,
    ) -> U0CdnsSpdifScfgSramConfigSlpW<SysSyscfg5Spec> {
        U0CdnsSpdifScfgSramConfigSlpW::new(self, 12)
    }
    #[doc = "Bit 13 - SRAM/ROM configuration. SD: shutdown enable, high active, default is low."]
    #[inline(always)]
    #[must_use]
    pub fn u0_cdns_spdif_scfg_sram_config_sd(
        &mut self,
    ) -> U0CdnsSpdifScfgSramConfigSdW<SysSyscfg5Spec> {
        U0CdnsSpdifScfgSramConfigSdW::new(self, 13)
    }
    #[doc = "Bits 14:15 - SRAM/ROM configuration. RTSEL: timing setting for debug purpose, default is 2'b01."]
    #[inline(always)]
    #[must_use]
    pub fn u0_cdns_spdif_scfg_sram_config_rtsel(
        &mut self,
    ) -> U0CdnsSpdifScfgSramConfigRtselW<SysSyscfg5Spec> {
        U0CdnsSpdifScfgSramConfigRtselW::new(self, 14)
    }
    #[doc = "Bits 16:17 - SRAM/ROM configuration. PTSEL: timing setting for debug purpose, default is 2'b01."]
    #[inline(always)]
    #[must_use]
    pub fn u0_cdns_spdif_scfg_sram_config_ptsel(
        &mut self,
    ) -> U0CdnsSpdifScfgSramConfigPtselW<SysSyscfg5Spec> {
        U0CdnsSpdifScfgSramConfigPtselW::new(self, 16)
    }
    #[doc = "Bits 18:19 - SRAM/ROM configuration. TRB: timing setting for debug purpose, default is 2'b01."]
    #[inline(always)]
    #[must_use]
    pub fn u0_cdns_spdif_scfg_sram_config_trb(
        &mut self,
    ) -> U0CdnsSpdifScfgSramConfigTrbW<SysSyscfg5Spec> {
        U0CdnsSpdifScfgSramConfigTrbW::new(self, 18)
    }
    #[doc = "Bits 20:21 - SRAM/ROM configuration. WTSEL: timing setting for debug purpose, default is 2'b01."]
    #[inline(always)]
    #[must_use]
    pub fn u0_cdns_spdif_scfg_sram_config_wtsel(
        &mut self,
    ) -> U0CdnsSpdifScfgSramConfigWtselW<SysSyscfg5Spec> {
        U0CdnsSpdifScfgSramConfigWtselW::new(self, 20)
    }
    #[doc = "Bit 22 - SRAM/ROM configuration. VS: timing setting for debug purpose, default is 1'b1."]
    #[inline(always)]
    #[must_use]
    pub fn u0_cdns_spdif_scfg_sram_config_vs(
        &mut self,
    ) -> U0CdnsSpdifScfgSramConfigVsW<SysSyscfg5Spec> {
        U0CdnsSpdifScfgSramConfigVsW::new(self, 22)
    }
    #[doc = "Bit 23 - SRAM/ROM configuration. VG: timing setting for debug purpose, default is 1'b1."]
    #[inline(always)]
    #[must_use]
    pub fn u0_cdns_spdif_scfg_sram_config_vg(
        &mut self,
    ) -> U0CdnsSpdifScfgSramConfigVgW<SysSyscfg5Spec> {
        U0CdnsSpdifScfgSramConfigVgW::new(self, 23)
    }
    #[doc = "Bits 26:30 - Data strobe delay chain select"]
    #[inline(always)]
    #[must_use]
    pub fn sdio_data_strobe_phase_ctrl(&mut self) -> SdioDataStrobePhaseCtrlW<SysSyscfg5Spec> {
        SdioDataStrobePhaseCtrlW::new(self, 26)
    }
    #[doc = "Bit 31 - AHB bus interface endianness: 1: Big-endian AHB bus interface, 0: Little-endian AHB bus interface"]
    #[inline(always)]
    #[must_use]
    pub fn sdio_hbig_endian(&mut self) -> SdioHbigEndianW<SysSyscfg5Spec> {
        SdioHbigEndianW::new(self, 31)
    }
}
#[doc = "SYS SYSCONSAIF SYSCFG 20\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_syscfg5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_syscfg5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SysSyscfg5Spec;
impl crate::RegisterSpec for SysSyscfg5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sys_syscfg5::R`](R) reader structure"]
impl crate::Readable for SysSyscfg5Spec {}
#[doc = "`write(|w| ..)` method takes [`sys_syscfg5::W`](W) writer structure"]
impl crate::Writable for SysSyscfg5Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets sys_syscfg5 to value 0x00d5_4d54"]
impl crate::Resettable for SysSyscfg5Spec {
    const RESET_VALUE: u32 = 0x00d5_4d54;
}
