#[doc = "Register `aon_syscfg_3` reader"]
pub type R = crate::R<AonSyscfg3Spec>;
#[doc = "Register `aon_syscfg_3` writer"]
pub type W = crate::W<AonSyscfg3Spec>;
#[doc = "Field `u0_boot_ctrl_boot_vector_32_35` reader - Boot vectors 32-35 (little-endian)"]
pub type U0BootCtrlBootVector32_35R = crate::FieldReader;
#[doc = "Field `gmac5_axi64_scfg_ram_cfg_slp` reader - SRAM/ROM configuration. SLP: sleep enable, high active, default is low."]
pub type Gmac5Axi64ScfgRamCfgSlpR = crate::BitReader;
#[doc = "Field `gmac5_axi64_scfg_ram_cfg_slp` writer - SRAM/ROM configuration. SLP: sleep enable, high active, default is low."]
pub type Gmac5Axi64ScfgRamCfgSlpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `gmac5_axi64_scfg_ram_cfg_sd` reader - SRAM/ROM configuration. SD: shutdown enable, high active, default is low."]
pub type Gmac5Axi64ScfgRamCfgSdR = crate::BitReader;
#[doc = "Field `gmac5_axi64_scfg_ram_cfg_sd` writer - SRAM/ROM configuration. SD: shutdown enable, high active, default is low."]
pub type Gmac5Axi64ScfgRamCfgSdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `gmac5_axi64_scfg_ram_cfg_rtsel` reader - SRAM/ROM configuration. RTSEL: timing setting for debug purpose, default is 2'b01."]
pub type Gmac5Axi64ScfgRamCfgRtselR = crate::FieldReader;
#[doc = "Field `gmac5_axi64_scfg_ram_cfg_rtsel` writer - SRAM/ROM configuration. RTSEL: timing setting for debug purpose, default is 2'b01."]
pub type Gmac5Axi64ScfgRamCfgRtselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `gmac5_axi64_scfg_ram_cfg_ptsel` reader - SRAM/ROM configuration. PTSEL: timing setting for debug purpose, default is 2'b01."]
pub type Gmac5Axi64ScfgRamCfgPtselR = crate::FieldReader;
#[doc = "Field `gmac5_axi64_scfg_ram_cfg_ptsel` writer - SRAM/ROM configuration. PTSEL: timing setting for debug purpose, default is 2'b01."]
pub type Gmac5Axi64ScfgRamCfgPtselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `gmac5_axi64_scfg_ram_cfg_trb` reader - SRAM/ROM configuration. TRB: timing setting for debug purpose, default is 2'b01."]
pub type Gmac5Axi64ScfgRamCfgTrbR = crate::FieldReader;
#[doc = "Field `gmac5_axi64_scfg_ram_cfg_trb` writer - SRAM/ROM configuration. TRB: timing setting for debug purpose, default is 2'b01."]
pub type Gmac5Axi64ScfgRamCfgTrbW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `gmac5_axi64_scfg_ram_cfg_wtsel` reader - SRAM/ROM configuration. WTSEL: timing setting for debug purpose, default is 2'b01."]
pub type Gmac5Axi64ScfgRamCfgWtselR = crate::FieldReader;
#[doc = "Field `gmac5_axi64_scfg_ram_cfg_wtsel` writer - SRAM/ROM configuration. WTSEL: timing setting for debug purpose, default is 2'b01."]
pub type Gmac5Axi64ScfgRamCfgWtselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `gmac5_axi64_scfg_ram_cfg_vs` reader - SRAM/ROM configuration. VS: timing setting for debug purpose, default is 1'b1."]
pub type Gmac5Axi64ScfgRamCfgVsR = crate::BitReader;
#[doc = "Field `gmac5_axi64_scfg_ram_cfg_vs` writer - SRAM/ROM configuration. VS: timing setting for debug purpose, default is 1'b1."]
pub type Gmac5Axi64ScfgRamCfgVsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `gmac5_axi64_scfg_ram_cfg_vg` reader - SRAM/ROM configuration. VG: timing setting for debug purpose, default is 1'b1."]
pub type Gmac5Axi64ScfgRamCfgVgR = crate::BitReader;
#[doc = "Field `gmac5_axi64_scfg_ram_cfg_vg` writer - SRAM/ROM configuration. VG: timing setting for debug purpose, default is 1'b1."]
pub type Gmac5Axi64ScfgRamCfgVgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `gmac5_axi64_mac_speed_o` reader - "]
pub type Gmac5Axi64MacSpeedOR = crate::FieldReader;
#[doc = "Field `gmac5_axi64_phy_intf_sel_i` reader - Active PHY Selected. When you have multiple GMAC PHY interfaces in your configuration, this field indicates the sampled value of the PHY selector during reset de-assertion. Values: 0x0 - GMII or MII, 0x1 - RGMII, 0x2 - SGMII, 0x3 - TBI, 0x4 - RMII, 0x5 - RTBI, 0x6 - SMII, 0x7 - REVMII"]
pub type Gmac5Axi64PhyIntfSelIR = crate::FieldReader;
#[doc = "Field `gmac5_axi64_phy_intf_sel_i` writer - Active PHY Selected. When you have multiple GMAC PHY interfaces in your configuration, this field indicates the sampled value of the PHY selector during reset de-assertion. Values: 0x0 - GMII or MII, 0x1 - RGMII, 0x2 - SGMII, 0x3 - TBI, 0x4 - RMII, 0x5 - RTBI, 0x6 - SMII, 0x7 - REVMII"]
pub type Gmac5Axi64PhyIntfSelIW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:3 - Boot vectors 32-35 (little-endian)"]
    #[inline(always)]
    pub fn u0_boot_ctrl_boot_vector_32_35(&self) -> U0BootCtrlBootVector32_35R {
        U0BootCtrlBootVector32_35R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - SRAM/ROM configuration. SLP: sleep enable, high active, default is low."]
    #[inline(always)]
    pub fn gmac5_axi64_scfg_ram_cfg_slp(&self) -> Gmac5Axi64ScfgRamCfgSlpR {
        Gmac5Axi64ScfgRamCfgSlpR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SRAM/ROM configuration. SD: shutdown enable, high active, default is low."]
    #[inline(always)]
    pub fn gmac5_axi64_scfg_ram_cfg_sd(&self) -> Gmac5Axi64ScfgRamCfgSdR {
        Gmac5Axi64ScfgRamCfgSdR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - SRAM/ROM configuration. RTSEL: timing setting for debug purpose, default is 2'b01."]
    #[inline(always)]
    pub fn gmac5_axi64_scfg_ram_cfg_rtsel(&self) -> Gmac5Axi64ScfgRamCfgRtselR {
        Gmac5Axi64ScfgRamCfgRtselR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - SRAM/ROM configuration. PTSEL: timing setting for debug purpose, default is 2'b01."]
    #[inline(always)]
    pub fn gmac5_axi64_scfg_ram_cfg_ptsel(&self) -> Gmac5Axi64ScfgRamCfgPtselR {
        Gmac5Axi64ScfgRamCfgPtselR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - SRAM/ROM configuration. TRB: timing setting for debug purpose, default is 2'b01."]
    #[inline(always)]
    pub fn gmac5_axi64_scfg_ram_cfg_trb(&self) -> Gmac5Axi64ScfgRamCfgTrbR {
        Gmac5Axi64ScfgRamCfgTrbR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - SRAM/ROM configuration. WTSEL: timing setting for debug purpose, default is 2'b01."]
    #[inline(always)]
    pub fn gmac5_axi64_scfg_ram_cfg_wtsel(&self) -> Gmac5Axi64ScfgRamCfgWtselR {
        Gmac5Axi64ScfgRamCfgWtselR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - SRAM/ROM configuration. VS: timing setting for debug purpose, default is 1'b1."]
    #[inline(always)]
    pub fn gmac5_axi64_scfg_ram_cfg_vs(&self) -> Gmac5Axi64ScfgRamCfgVsR {
        Gmac5Axi64ScfgRamCfgVsR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - SRAM/ROM configuration. VG: timing setting for debug purpose, default is 1'b1."]
    #[inline(always)]
    pub fn gmac5_axi64_scfg_ram_cfg_vg(&self) -> Gmac5Axi64ScfgRamCfgVgR {
        Gmac5Axi64ScfgRamCfgVgR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn gmac5_axi64_mac_speed_o(&self) -> Gmac5Axi64MacSpeedOR {
        Gmac5Axi64MacSpeedOR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:20 - Active PHY Selected. When you have multiple GMAC PHY interfaces in your configuration, this field indicates the sampled value of the PHY selector during reset de-assertion. Values: 0x0 - GMII or MII, 0x1 - RGMII, 0x2 - SGMII, 0x3 - TBI, 0x4 - RMII, 0x5 - RTBI, 0x6 - SMII, 0x7 - REVMII"]
    #[inline(always)]
    pub fn gmac5_axi64_phy_intf_sel_i(&self) -> Gmac5Axi64PhyIntfSelIR {
        Gmac5Axi64PhyIntfSelIR::new(((self.bits >> 18) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 4 - SRAM/ROM configuration. SLP: sleep enable, high active, default is low."]
    #[inline(always)]
    #[must_use]
    pub fn gmac5_axi64_scfg_ram_cfg_slp(&mut self) -> Gmac5Axi64ScfgRamCfgSlpW<AonSyscfg3Spec> {
        Gmac5Axi64ScfgRamCfgSlpW::new(self, 4)
    }
    #[doc = "Bit 5 - SRAM/ROM configuration. SD: shutdown enable, high active, default is low."]
    #[inline(always)]
    #[must_use]
    pub fn gmac5_axi64_scfg_ram_cfg_sd(&mut self) -> Gmac5Axi64ScfgRamCfgSdW<AonSyscfg3Spec> {
        Gmac5Axi64ScfgRamCfgSdW::new(self, 5)
    }
    #[doc = "Bits 6:7 - SRAM/ROM configuration. RTSEL: timing setting for debug purpose, default is 2'b01."]
    #[inline(always)]
    #[must_use]
    pub fn gmac5_axi64_scfg_ram_cfg_rtsel(&mut self) -> Gmac5Axi64ScfgRamCfgRtselW<AonSyscfg3Spec> {
        Gmac5Axi64ScfgRamCfgRtselW::new(self, 6)
    }
    #[doc = "Bits 8:9 - SRAM/ROM configuration. PTSEL: timing setting for debug purpose, default is 2'b01."]
    #[inline(always)]
    #[must_use]
    pub fn gmac5_axi64_scfg_ram_cfg_ptsel(&mut self) -> Gmac5Axi64ScfgRamCfgPtselW<AonSyscfg3Spec> {
        Gmac5Axi64ScfgRamCfgPtselW::new(self, 8)
    }
    #[doc = "Bits 10:11 - SRAM/ROM configuration. TRB: timing setting for debug purpose, default is 2'b01."]
    #[inline(always)]
    #[must_use]
    pub fn gmac5_axi64_scfg_ram_cfg_trb(&mut self) -> Gmac5Axi64ScfgRamCfgTrbW<AonSyscfg3Spec> {
        Gmac5Axi64ScfgRamCfgTrbW::new(self, 10)
    }
    #[doc = "Bits 12:13 - SRAM/ROM configuration. WTSEL: timing setting for debug purpose, default is 2'b01."]
    #[inline(always)]
    #[must_use]
    pub fn gmac5_axi64_scfg_ram_cfg_wtsel(&mut self) -> Gmac5Axi64ScfgRamCfgWtselW<AonSyscfg3Spec> {
        Gmac5Axi64ScfgRamCfgWtselW::new(self, 12)
    }
    #[doc = "Bit 14 - SRAM/ROM configuration. VS: timing setting for debug purpose, default is 1'b1."]
    #[inline(always)]
    #[must_use]
    pub fn gmac5_axi64_scfg_ram_cfg_vs(&mut self) -> Gmac5Axi64ScfgRamCfgVsW<AonSyscfg3Spec> {
        Gmac5Axi64ScfgRamCfgVsW::new(self, 14)
    }
    #[doc = "Bit 15 - SRAM/ROM configuration. VG: timing setting for debug purpose, default is 1'b1."]
    #[inline(always)]
    #[must_use]
    pub fn gmac5_axi64_scfg_ram_cfg_vg(&mut self) -> Gmac5Axi64ScfgRamCfgVgW<AonSyscfg3Spec> {
        Gmac5Axi64ScfgRamCfgVgW::new(self, 15)
    }
    #[doc = "Bits 18:20 - Active PHY Selected. When you have multiple GMAC PHY interfaces in your configuration, this field indicates the sampled value of the PHY selector during reset de-assertion. Values: 0x0 - GMII or MII, 0x1 - RGMII, 0x2 - SGMII, 0x3 - TBI, 0x4 - RMII, 0x5 - RTBI, 0x6 - SMII, 0x7 - REVMII"]
    #[inline(always)]
    #[must_use]
    pub fn gmac5_axi64_phy_intf_sel_i(&mut self) -> Gmac5Axi64PhyIntfSelIW<AonSyscfg3Spec> {
        Gmac5Axi64PhyIntfSelIW::new(self, 18)
    }
}
#[doc = "AON SYSCONSAIF SYSCFG 12\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aon_syscfg_3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aon_syscfg_3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AonSyscfg3Spec;
impl crate::RegisterSpec for AonSyscfg3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aon_syscfg_3::R`](R) reader structure"]
impl crate::Readable for AonSyscfg3Spec {}
#[doc = "`write(|w| ..)` method takes [`aon_syscfg_3::W`](W) writer structure"]
impl crate::Writable for AonSyscfg3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets aon_syscfg_3 to value 0x0004_d540"]
impl crate::Resettable for AonSyscfg3Spec {
    const RESET_VALUE: u32 = 0x0004_d540;
}
