#[doc = "Register `sys_syscfg6` reader"]
pub type R = crate::R<SysSyscfg6Spec>;
#[doc = "Register `sys_syscfg6` writer"]
pub type W = crate::W<SysSyscfg6Spec>;
#[doc = "Field `sdio_m_hbig_endian` reader - AHB master bus interface endianess: 1: Big-endian AHB bus interface, 0: Little-endian AHB bus interface"]
pub type SdioMHbigEndianR = crate::BitReader;
#[doc = "Field `sdio_m_hbig_endian` writer - AHB master bus interface endianess: 1: Big-endian AHB bus interface, 0: Little-endian AHB bus interface"]
pub type SdioMHbigEndianW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `i2srx_adc_en` reader - I2S RX ADC enable"]
pub type I2srxAdcEnR = crate::BitReader;
#[doc = "Field `i2srx_adc_en` writer - I2S RX ADC enable"]
pub type I2srxAdcEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `intmem_rom_sram_scfg_disable_rom` reader - Internal Memory ROM SRAM SCFG Disable ROM"]
pub type IntmemRomSramScfgDisableRomR = crate::BitReader;
#[doc = "Field `intmem_rom_sram_scfg_disable_rom` writer - Internal Memory ROM SRAM SCFG Disable ROM"]
pub type IntmemRomSramScfgDisableRomW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `intmem_rom_sram_config_slp` reader - SRAM/ROM configuration. SLP: sleep enable, high active, default is low."]
pub type IntmemRomSramConfigSlpR = crate::BitReader;
#[doc = "Field `intmem_rom_sram_config_slp` writer - SRAM/ROM configuration. SLP: sleep enable, high active, default is low."]
pub type IntmemRomSramConfigSlpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `intmem_rom_sram_config_sd` reader - SRAM/ROM configuration. SD: shutdown enable, high active, default is low."]
pub type IntmemRomSramConfigSdR = crate::BitReader;
#[doc = "Field `intmem_rom_sram_config_sd` writer - SRAM/ROM configuration. SD: shutdown enable, high active, default is low."]
pub type IntmemRomSramConfigSdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `intmem_rom_sram_config_rtsel` reader - SRAM/ROM configuration. RTSEL: timing setting for debug purpose, default is 2'b01."]
pub type IntmemRomSramConfigRtselR = crate::FieldReader;
#[doc = "Field `intmem_rom_sram_config_rtsel` writer - SRAM/ROM configuration. RTSEL: timing setting for debug purpose, default is 2'b01."]
pub type IntmemRomSramConfigRtselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `intmem_rom_sram_config_ptsel` reader - SRAM/ROM configuration. PTSEL: timing setting for debug purpose, default is 2'b01."]
pub type IntmemRomSramConfigPtselR = crate::FieldReader;
#[doc = "Field `intmem_rom_sram_config_ptsel` writer - SRAM/ROM configuration. PTSEL: timing setting for debug purpose, default is 2'b01."]
pub type IntmemRomSramConfigPtselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `intmem_rom_sram_config_trb` reader - SRAM/ROM configuration. TRB: timing setting for debug purpose, default is 2'b01."]
pub type IntmemRomSramConfigTrbR = crate::FieldReader;
#[doc = "Field `intmem_rom_sram_config_trb` writer - SRAM/ROM configuration. TRB: timing setting for debug purpose, default is 2'b01."]
pub type IntmemRomSramConfigTrbW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `intmem_rom_sram_config_wtsel` reader - SRAM/ROM configuration. WTSEL: timing setting for debug purpose, default is 2'b01."]
pub type IntmemRomSramConfigWtselR = crate::FieldReader;
#[doc = "Field `intmem_rom_sram_config_wtsel` writer - SRAM/ROM configuration. WTSEL: timing setting for debug purpose, default is 2'b01."]
pub type IntmemRomSramConfigWtselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `intmem_rom_sram_config_vs` reader - SRAM/ROM configuration. VS: timing setting for debug purpose, default is 1'b1."]
pub type IntmemRomSramConfigVsR = crate::BitReader;
#[doc = "Field `intmem_rom_sram_config_vs` writer - SRAM/ROM configuration. VS: timing setting for debug purpose, default is 1'b1."]
pub type IntmemRomSramConfigVsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `intmem_rom_sram_config_vg` reader - SRAM/ROM configuration. VG: timing setting for debug purpose, default is 1'b1."]
pub type IntmemRomSramConfigVgR = crate::BitReader;
#[doc = "Field `intmem_rom_sram_config_vg` writer - SRAM/ROM configuration. VG: timing setting for debug purpose, default is 1'b1."]
pub type IntmemRomSramConfigVgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `jtag_en(0-1)` reader - JTAG daisy-chain enable"]
pub type JtagEnR = crate::BitReader;
#[doc = "Field `jtag_en(0-1)` writer - JTAG daisy-chain enable"]
pub type JtagEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pdrstn_split_sw_usbpipe_plugen` reader - PD RSTN Split Software USB Pipe Plug enable"]
pub type PdrstnSplitSwUsbpipePlugenR = crate::BitReader;
#[doc = "Field `pdrstn_split_sw_usbpipe_plugen` writer - PD RSTN Split Software USB Pipe Plug enable"]
pub type PdrstnSplitSwUsbpipePlugenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pll0_cpi_bias` reader - PLL0 CPI bias"]
pub type Pll0CpiBiasR = crate::FieldReader;
#[doc = "Field `pll0_cpi_bias` writer - PLL0 CPI bias"]
pub type Pll0CpiBiasW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `pll0_cpp_bias` reader - PLL0 CPP bias"]
pub type Pll0CppBiasR = crate::FieldReader;
#[doc = "Field `pll0_cpp_bias` writer - PLL0 CPP bias"]
pub type Pll0CppBiasW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "PLL0 DACPD.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pll0Dacpd {
    #[doc = "0: Disable PLL0 DACPD."]
    Off = 0,
    #[doc = "1: Enable PLL0 DACPD."]
    On = 1,
}
impl From<Pll0Dacpd> for bool {
    #[inline(always)]
    fn from(variant: Pll0Dacpd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `pll0_dacpd` reader - PLL0 DACPD."]
pub type Pll0DacpdR = crate::BitReader<Pll0Dacpd>;
impl Pll0DacpdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pll0Dacpd {
        match self.bits {
            false => Pll0Dacpd::Off,
            true => Pll0Dacpd::On,
        }
    }
    #[doc = "Disable PLL0 DACPD."]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Pll0Dacpd::Off
    }
    #[doc = "Enable PLL0 DACPD."]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == Pll0Dacpd::On
    }
}
#[doc = "Field `pll0_dacpd` writer - PLL0 DACPD."]
pub type Pll0DacpdW<'a, REG> = crate::BitWriter<'a, REG, Pll0Dacpd>;
impl<'a, REG> Pll0DacpdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable PLL0 DACPD."]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Pll0Dacpd::Off)
    }
    #[doc = "Enable PLL0 DACPD."]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(Pll0Dacpd::On)
    }
}
#[doc = "PLL0 DSMPD.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pll0Dsmpd {
    #[doc = "0: Disable PLL0 DSMPD."]
    Off = 0,
    #[doc = "1: Enable PLL0 DSMPD."]
    On = 1,
}
impl From<Pll0Dsmpd> for bool {
    #[inline(always)]
    fn from(variant: Pll0Dsmpd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `pll0_dsmpd` reader - PLL0 DSMPD."]
pub type Pll0DsmpdR = crate::BitReader<Pll0Dsmpd>;
impl Pll0DsmpdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pll0Dsmpd {
        match self.bits {
            false => Pll0Dsmpd::Off,
            true => Pll0Dsmpd::On,
        }
    }
    #[doc = "Disable PLL0 DSMPD."]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Pll0Dsmpd::Off
    }
    #[doc = "Enable PLL0 DSMPD."]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == Pll0Dsmpd::On
    }
}
#[doc = "Field `pll0_dsmpd` writer - PLL0 DSMPD."]
pub type Pll0DsmpdW<'a, REG> = crate::BitWriter<'a, REG, Pll0Dsmpd>;
impl<'a, REG> Pll0DsmpdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable PLL0 DSMPD."]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Pll0Dsmpd::Off)
    }
    #[doc = "Enable PLL0 DSMPD."]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(Pll0Dsmpd::On)
    }
}
impl R {
    #[doc = "Bit 0 - AHB master bus interface endianess: 1: Big-endian AHB bus interface, 0: Little-endian AHB bus interface"]
    #[inline(always)]
    pub fn sdio_m_hbig_endian(&self) -> SdioMHbigEndianR {
        SdioMHbigEndianR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - I2S RX ADC enable"]
    #[inline(always)]
    pub fn i2srx_adc_en(&self) -> I2srxAdcEnR {
        I2srxAdcEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Internal Memory ROM SRAM SCFG Disable ROM"]
    #[inline(always)]
    pub fn intmem_rom_sram_scfg_disable_rom(&self) -> IntmemRomSramScfgDisableRomR {
        IntmemRomSramScfgDisableRomR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SRAM/ROM configuration. SLP: sleep enable, high active, default is low."]
    #[inline(always)]
    pub fn intmem_rom_sram_config_slp(&self) -> IntmemRomSramConfigSlpR {
        IntmemRomSramConfigSlpR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SRAM/ROM configuration. SD: shutdown enable, high active, default is low."]
    #[inline(always)]
    pub fn intmem_rom_sram_config_sd(&self) -> IntmemRomSramConfigSdR {
        IntmemRomSramConfigSdR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - SRAM/ROM configuration. RTSEL: timing setting for debug purpose, default is 2'b01."]
    #[inline(always)]
    pub fn intmem_rom_sram_config_rtsel(&self) -> IntmemRomSramConfigRtselR {
        IntmemRomSramConfigRtselR::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bits 7:8 - SRAM/ROM configuration. PTSEL: timing setting for debug purpose, default is 2'b01."]
    #[inline(always)]
    pub fn intmem_rom_sram_config_ptsel(&self) -> IntmemRomSramConfigPtselR {
        IntmemRomSramConfigPtselR::new(((self.bits >> 7) & 3) as u8)
    }
    #[doc = "Bits 9:10 - SRAM/ROM configuration. TRB: timing setting for debug purpose, default is 2'b01."]
    #[inline(always)]
    pub fn intmem_rom_sram_config_trb(&self) -> IntmemRomSramConfigTrbR {
        IntmemRomSramConfigTrbR::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bits 11:12 - SRAM/ROM configuration. WTSEL: timing setting for debug purpose, default is 2'b01."]
    #[inline(always)]
    pub fn intmem_rom_sram_config_wtsel(&self) -> IntmemRomSramConfigWtselR {
        IntmemRomSramConfigWtselR::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bit 13 - SRAM/ROM configuration. VS: timing setting for debug purpose, default is 1'b1."]
    #[inline(always)]
    pub fn intmem_rom_sram_config_vs(&self) -> IntmemRomSramConfigVsR {
        IntmemRomSramConfigVsR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - SRAM/ROM configuration. VG: timing setting for debug purpose, default is 1'b1."]
    #[inline(always)]
    pub fn intmem_rom_sram_config_vg(&self) -> IntmemRomSramConfigVgR {
        IntmemRomSramConfigVgR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "JTAG daisy-chain enable"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `jtag_en0` field"]
    #[inline(always)]
    pub fn jtag_en(&self, n: u8) -> JtagEnR {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        JtagEnR::new(((self.bits >> (n + 15)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "JTAG daisy-chain enable"]
    #[inline(always)]
    pub fn jtag_en_iter(&self) -> impl Iterator<Item = JtagEnR> + '_ {
        (0..2).map(move |n| JtagEnR::new(((self.bits >> (n + 15)) & 1) != 0))
    }
    #[doc = "Bit 15 - JTAG daisy-chain enable"]
    #[inline(always)]
    pub fn jtag_en0(&self) -> JtagEnR {
        JtagEnR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - JTAG daisy-chain enable"]
    #[inline(always)]
    pub fn jtag_en1(&self) -> JtagEnR {
        JtagEnR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - PD RSTN Split Software USB Pipe Plug enable"]
    #[inline(always)]
    pub fn pdrstn_split_sw_usbpipe_plugen(&self) -> PdrstnSplitSwUsbpipePlugenR {
        PdrstnSplitSwUsbpipePlugenR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:20 - PLL0 CPI bias"]
    #[inline(always)]
    pub fn pll0_cpi_bias(&self) -> Pll0CpiBiasR {
        Pll0CpiBiasR::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bits 21:23 - PLL0 CPP bias"]
    #[inline(always)]
    pub fn pll0_cpp_bias(&self) -> Pll0CppBiasR {
        Pll0CppBiasR::new(((self.bits >> 21) & 7) as u8)
    }
    #[doc = "Bit 24 - PLL0 DACPD."]
    #[inline(always)]
    pub fn pll0_dacpd(&self) -> Pll0DacpdR {
        Pll0DacpdR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - PLL0 DSMPD."]
    #[inline(always)]
    pub fn pll0_dsmpd(&self) -> Pll0DsmpdR {
        Pll0DsmpdR::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - AHB master bus interface endianess: 1: Big-endian AHB bus interface, 0: Little-endian AHB bus interface"]
    #[inline(always)]
    #[must_use]
    pub fn sdio_m_hbig_endian(&mut self) -> SdioMHbigEndianW<SysSyscfg6Spec> {
        SdioMHbigEndianW::new(self, 0)
    }
    #[doc = "Bit 1 - I2S RX ADC enable"]
    #[inline(always)]
    #[must_use]
    pub fn i2srx_adc_en(&mut self) -> I2srxAdcEnW<SysSyscfg6Spec> {
        I2srxAdcEnW::new(self, 1)
    }
    #[doc = "Bit 2 - Internal Memory ROM SRAM SCFG Disable ROM"]
    #[inline(always)]
    #[must_use]
    pub fn intmem_rom_sram_scfg_disable_rom(
        &mut self,
    ) -> IntmemRomSramScfgDisableRomW<SysSyscfg6Spec> {
        IntmemRomSramScfgDisableRomW::new(self, 2)
    }
    #[doc = "Bit 3 - SRAM/ROM configuration. SLP: sleep enable, high active, default is low."]
    #[inline(always)]
    #[must_use]
    pub fn intmem_rom_sram_config_slp(&mut self) -> IntmemRomSramConfigSlpW<SysSyscfg6Spec> {
        IntmemRomSramConfigSlpW::new(self, 3)
    }
    #[doc = "Bit 4 - SRAM/ROM configuration. SD: shutdown enable, high active, default is low."]
    #[inline(always)]
    #[must_use]
    pub fn intmem_rom_sram_config_sd(&mut self) -> IntmemRomSramConfigSdW<SysSyscfg6Spec> {
        IntmemRomSramConfigSdW::new(self, 4)
    }
    #[doc = "Bits 5:6 - SRAM/ROM configuration. RTSEL: timing setting for debug purpose, default is 2'b01."]
    #[inline(always)]
    #[must_use]
    pub fn intmem_rom_sram_config_rtsel(&mut self) -> IntmemRomSramConfigRtselW<SysSyscfg6Spec> {
        IntmemRomSramConfigRtselW::new(self, 5)
    }
    #[doc = "Bits 7:8 - SRAM/ROM configuration. PTSEL: timing setting for debug purpose, default is 2'b01."]
    #[inline(always)]
    #[must_use]
    pub fn intmem_rom_sram_config_ptsel(&mut self) -> IntmemRomSramConfigPtselW<SysSyscfg6Spec> {
        IntmemRomSramConfigPtselW::new(self, 7)
    }
    #[doc = "Bits 9:10 - SRAM/ROM configuration. TRB: timing setting for debug purpose, default is 2'b01."]
    #[inline(always)]
    #[must_use]
    pub fn intmem_rom_sram_config_trb(&mut self) -> IntmemRomSramConfigTrbW<SysSyscfg6Spec> {
        IntmemRomSramConfigTrbW::new(self, 9)
    }
    #[doc = "Bits 11:12 - SRAM/ROM configuration. WTSEL: timing setting for debug purpose, default is 2'b01."]
    #[inline(always)]
    #[must_use]
    pub fn intmem_rom_sram_config_wtsel(&mut self) -> IntmemRomSramConfigWtselW<SysSyscfg6Spec> {
        IntmemRomSramConfigWtselW::new(self, 11)
    }
    #[doc = "Bit 13 - SRAM/ROM configuration. VS: timing setting for debug purpose, default is 1'b1."]
    #[inline(always)]
    #[must_use]
    pub fn intmem_rom_sram_config_vs(&mut self) -> IntmemRomSramConfigVsW<SysSyscfg6Spec> {
        IntmemRomSramConfigVsW::new(self, 13)
    }
    #[doc = "Bit 14 - SRAM/ROM configuration. VG: timing setting for debug purpose, default is 1'b1."]
    #[inline(always)]
    #[must_use]
    pub fn intmem_rom_sram_config_vg(&mut self) -> IntmemRomSramConfigVgW<SysSyscfg6Spec> {
        IntmemRomSramConfigVgW::new(self, 14)
    }
    #[doc = "JTAG daisy-chain enable"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `jtag_en0` field"]
    #[inline(always)]
    #[must_use]
    pub fn jtag_en(&mut self, n: u8) -> JtagEnW<SysSyscfg6Spec> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        JtagEnW::new(self, n + 15)
    }
    #[doc = "Bit 15 - JTAG daisy-chain enable"]
    #[inline(always)]
    #[must_use]
    pub fn jtag_en0(&mut self) -> JtagEnW<SysSyscfg6Spec> {
        JtagEnW::new(self, 15)
    }
    #[doc = "Bit 16 - JTAG daisy-chain enable"]
    #[inline(always)]
    #[must_use]
    pub fn jtag_en1(&mut self) -> JtagEnW<SysSyscfg6Spec> {
        JtagEnW::new(self, 16)
    }
    #[doc = "Bit 17 - PD RSTN Split Software USB Pipe Plug enable"]
    #[inline(always)]
    #[must_use]
    pub fn pdrstn_split_sw_usbpipe_plugen(
        &mut self,
    ) -> PdrstnSplitSwUsbpipePlugenW<SysSyscfg6Spec> {
        PdrstnSplitSwUsbpipePlugenW::new(self, 17)
    }
    #[doc = "Bits 18:20 - PLL0 CPI bias"]
    #[inline(always)]
    #[must_use]
    pub fn pll0_cpi_bias(&mut self) -> Pll0CpiBiasW<SysSyscfg6Spec> {
        Pll0CpiBiasW::new(self, 18)
    }
    #[doc = "Bits 21:23 - PLL0 CPP bias"]
    #[inline(always)]
    #[must_use]
    pub fn pll0_cpp_bias(&mut self) -> Pll0CppBiasW<SysSyscfg6Spec> {
        Pll0CppBiasW::new(self, 21)
    }
    #[doc = "Bit 24 - PLL0 DACPD."]
    #[inline(always)]
    #[must_use]
    pub fn pll0_dacpd(&mut self) -> Pll0DacpdW<SysSyscfg6Spec> {
        Pll0DacpdW::new(self, 24)
    }
    #[doc = "Bit 25 - PLL0 DSMPD."]
    #[inline(always)]
    #[must_use]
    pub fn pll0_dsmpd(&mut self) -> Pll0DsmpdW<SysSyscfg6Spec> {
        Pll0DsmpdW::new(self, 25)
    }
}
#[doc = "SYS SYSCONSAIF SYSCFG 24\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_syscfg6::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_syscfg6::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SysSyscfg6Spec;
impl crate::RegisterSpec for SysSyscfg6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sys_syscfg6::R`](R) reader structure"]
impl crate::Readable for SysSyscfg6Spec {}
#[doc = "`write(|w| ..)` method takes [`sys_syscfg6::W`](W) writer structure"]
impl crate::Writable for SysSyscfg6Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets sys_syscfg6 to value 0x004d_ea80"]
impl crate::Resettable for SysSyscfg6Spec {
    const RESET_VALUE: u32 = 0x004d_ea80;
}
