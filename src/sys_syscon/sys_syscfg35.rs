#[doc = "Register `sys_syscfg35` reader"]
pub type R = crate::R<SysSyscfg35Spec>;
#[doc = "Register `sys_syscfg35` writer"]
pub type W = crate::W<SysSyscfg35Spec>;
#[doc = "Field `can_ctrl_host_if_1` reader - "]
pub type CanCtrlHostIf1R = crate::FieldReader<u32>;
#[doc = "Field `u1_gmac5_axi64_scfg_ram_cfg_slp` reader - SRAM/ROM configuration. SLP: sleep enable, high active, default is low."]
pub type U1Gmac5Axi64ScfgRamCfgSlpR = crate::BitReader;
#[doc = "Field `u1_gmac5_axi64_scfg_ram_cfg_slp` writer - SRAM/ROM configuration. SLP: sleep enable, high active, default is low."]
pub type U1Gmac5Axi64ScfgRamCfgSlpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u1_gmac5_axi64_scfg_ram_cfg_sd` reader - SRAM/ROM configuration. SD: shutdown enable, high active, default is low."]
pub type U1Gmac5Axi64ScfgRamCfgSdR = crate::BitReader;
#[doc = "Field `u1_gmac5_axi64_scfg_ram_cfg_sd` writer - SRAM/ROM configuration. SD: shutdown enable, high active, default is low."]
pub type U1Gmac5Axi64ScfgRamCfgSdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u1_gmac5_axi64_scfg_ram_cfg_rtsel` reader - SRAM/ROM configuration. RTSEL: timing setting for debug purpose, default is 2'b01."]
pub type U1Gmac5Axi64ScfgRamCfgRtselR = crate::FieldReader;
#[doc = "Field `u1_gmac5_axi64_scfg_ram_cfg_rtsel` writer - SRAM/ROM configuration. RTSEL: timing setting for debug purpose, default is 2'b01."]
pub type U1Gmac5Axi64ScfgRamCfgRtselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `u1_gmac5_axi64_scfg_ram_cfg_ptsel` reader - SRAM/ROM configuration. PTSEL: timing setting for debug purpose, default is 2'b01."]
pub type U1Gmac5Axi64ScfgRamCfgPtselR = crate::FieldReader;
#[doc = "Field `u1_gmac5_axi64_scfg_ram_cfg_ptsel` writer - SRAM/ROM configuration. PTSEL: timing setting for debug purpose, default is 2'b01."]
pub type U1Gmac5Axi64ScfgRamCfgPtselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `u1_gmac5_axi64_scfg_ram_cfg_trb` reader - SRAM/ROM configuration. TRB: timing setting for debug purpose, default is 2'b01."]
pub type U1Gmac5Axi64ScfgRamCfgTrbR = crate::FieldReader;
#[doc = "Field `u1_gmac5_axi64_scfg_ram_cfg_trb` writer - SRAM/ROM configuration. TRB: timing setting for debug purpose, default is 2'b01."]
pub type U1Gmac5Axi64ScfgRamCfgTrbW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `u1_gmac5_axi64_scfg_ram_cfg_wtsel` reader - SRAM/ROM configuration. WTSEL: timing setting for debug purpose, default is 2'b01."]
pub type U1Gmac5Axi64ScfgRamCfgWtselR = crate::FieldReader;
#[doc = "Field `u1_gmac5_axi64_scfg_ram_cfg_wtsel` writer - SRAM/ROM configuration. WTSEL: timing setting for debug purpose, default is 2'b01."]
pub type U1Gmac5Axi64ScfgRamCfgWtselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `u1_gmac5_axi64_scfg_ram_cfg_vs` reader - SRAM/ROM configuration. VS: timing setting for debug purpose, default is 1'b1."]
pub type U1Gmac5Axi64ScfgRamCfgVsR = crate::BitReader;
#[doc = "Field `u1_gmac5_axi64_scfg_ram_cfg_vs` writer - SRAM/ROM configuration. VS: timing setting for debug purpose, default is 1'b1."]
pub type U1Gmac5Axi64ScfgRamCfgVsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u1_gmac5_axi64_scfg_ram_cfg_vg` reader - SRAM/ROM configuration. VG: timing setting for debug purpose, default is 1'b1."]
pub type U1Gmac5Axi64ScfgRamCfgVgR = crate::BitReader;
#[doc = "Field `u1_gmac5_axi64_scfg_ram_cfg_vg` writer - SRAM/ROM configuration. VG: timing setting for debug purpose, default is 1'b1."]
pub type U1Gmac5Axi64ScfgRamCfgVgW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:18"]
    #[inline(always)]
    pub fn can_ctrl_host_if_1(&self) -> CanCtrlHostIf1R {
        CanCtrlHostIf1R::new(self.bits & 0x0007_ffff)
    }
    #[doc = "Bit 19 - SRAM/ROM configuration. SLP: sleep enable, high active, default is low."]
    #[inline(always)]
    pub fn u1_gmac5_axi64_scfg_ram_cfg_slp(&self) -> U1Gmac5Axi64ScfgRamCfgSlpR {
        U1Gmac5Axi64ScfgRamCfgSlpR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - SRAM/ROM configuration. SD: shutdown enable, high active, default is low."]
    #[inline(always)]
    pub fn u1_gmac5_axi64_scfg_ram_cfg_sd(&self) -> U1Gmac5Axi64ScfgRamCfgSdR {
        U1Gmac5Axi64ScfgRamCfgSdR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:22 - SRAM/ROM configuration. RTSEL: timing setting for debug purpose, default is 2'b01."]
    #[inline(always)]
    pub fn u1_gmac5_axi64_scfg_ram_cfg_rtsel(&self) -> U1Gmac5Axi64ScfgRamCfgRtselR {
        U1Gmac5Axi64ScfgRamCfgRtselR::new(((self.bits >> 21) & 3) as u8)
    }
    #[doc = "Bits 23:24 - SRAM/ROM configuration. PTSEL: timing setting for debug purpose, default is 2'b01."]
    #[inline(always)]
    pub fn u1_gmac5_axi64_scfg_ram_cfg_ptsel(&self) -> U1Gmac5Axi64ScfgRamCfgPtselR {
        U1Gmac5Axi64ScfgRamCfgPtselR::new(((self.bits >> 23) & 3) as u8)
    }
    #[doc = "Bits 25:26 - SRAM/ROM configuration. TRB: timing setting for debug purpose, default is 2'b01."]
    #[inline(always)]
    pub fn u1_gmac5_axi64_scfg_ram_cfg_trb(&self) -> U1Gmac5Axi64ScfgRamCfgTrbR {
        U1Gmac5Axi64ScfgRamCfgTrbR::new(((self.bits >> 25) & 3) as u8)
    }
    #[doc = "Bits 27:28 - SRAM/ROM configuration. WTSEL: timing setting for debug purpose, default is 2'b01."]
    #[inline(always)]
    pub fn u1_gmac5_axi64_scfg_ram_cfg_wtsel(&self) -> U1Gmac5Axi64ScfgRamCfgWtselR {
        U1Gmac5Axi64ScfgRamCfgWtselR::new(((self.bits >> 27) & 3) as u8)
    }
    #[doc = "Bit 29 - SRAM/ROM configuration. VS: timing setting for debug purpose, default is 1'b1."]
    #[inline(always)]
    pub fn u1_gmac5_axi64_scfg_ram_cfg_vs(&self) -> U1Gmac5Axi64ScfgRamCfgVsR {
        U1Gmac5Axi64ScfgRamCfgVsR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - SRAM/ROM configuration. VG: timing setting for debug purpose, default is 1'b1."]
    #[inline(always)]
    pub fn u1_gmac5_axi64_scfg_ram_cfg_vg(&self) -> U1Gmac5Axi64ScfgRamCfgVgR {
        U1Gmac5Axi64ScfgRamCfgVgR::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 19 - SRAM/ROM configuration. SLP: sleep enable, high active, default is low."]
    #[inline(always)]
    #[must_use]
    pub fn u1_gmac5_axi64_scfg_ram_cfg_slp(
        &mut self,
    ) -> U1Gmac5Axi64ScfgRamCfgSlpW<SysSyscfg35Spec> {
        U1Gmac5Axi64ScfgRamCfgSlpW::new(self, 19)
    }
    #[doc = "Bit 20 - SRAM/ROM configuration. SD: shutdown enable, high active, default is low."]
    #[inline(always)]
    #[must_use]
    pub fn u1_gmac5_axi64_scfg_ram_cfg_sd(&mut self) -> U1Gmac5Axi64ScfgRamCfgSdW<SysSyscfg35Spec> {
        U1Gmac5Axi64ScfgRamCfgSdW::new(self, 20)
    }
    #[doc = "Bits 21:22 - SRAM/ROM configuration. RTSEL: timing setting for debug purpose, default is 2'b01."]
    #[inline(always)]
    #[must_use]
    pub fn u1_gmac5_axi64_scfg_ram_cfg_rtsel(
        &mut self,
    ) -> U1Gmac5Axi64ScfgRamCfgRtselW<SysSyscfg35Spec> {
        U1Gmac5Axi64ScfgRamCfgRtselW::new(self, 21)
    }
    #[doc = "Bits 23:24 - SRAM/ROM configuration. PTSEL: timing setting for debug purpose, default is 2'b01."]
    #[inline(always)]
    #[must_use]
    pub fn u1_gmac5_axi64_scfg_ram_cfg_ptsel(
        &mut self,
    ) -> U1Gmac5Axi64ScfgRamCfgPtselW<SysSyscfg35Spec> {
        U1Gmac5Axi64ScfgRamCfgPtselW::new(self, 23)
    }
    #[doc = "Bits 25:26 - SRAM/ROM configuration. TRB: timing setting for debug purpose, default is 2'b01."]
    #[inline(always)]
    #[must_use]
    pub fn u1_gmac5_axi64_scfg_ram_cfg_trb(
        &mut self,
    ) -> U1Gmac5Axi64ScfgRamCfgTrbW<SysSyscfg35Spec> {
        U1Gmac5Axi64ScfgRamCfgTrbW::new(self, 25)
    }
    #[doc = "Bits 27:28 - SRAM/ROM configuration. WTSEL: timing setting for debug purpose, default is 2'b01."]
    #[inline(always)]
    #[must_use]
    pub fn u1_gmac5_axi64_scfg_ram_cfg_wtsel(
        &mut self,
    ) -> U1Gmac5Axi64ScfgRamCfgWtselW<SysSyscfg35Spec> {
        U1Gmac5Axi64ScfgRamCfgWtselW::new(self, 27)
    }
    #[doc = "Bit 29 - SRAM/ROM configuration. VS: timing setting for debug purpose, default is 1'b1."]
    #[inline(always)]
    #[must_use]
    pub fn u1_gmac5_axi64_scfg_ram_cfg_vs(&mut self) -> U1Gmac5Axi64ScfgRamCfgVsW<SysSyscfg35Spec> {
        U1Gmac5Axi64ScfgRamCfgVsW::new(self, 29)
    }
    #[doc = "Bit 30 - SRAM/ROM configuration. VG: timing setting for debug purpose, default is 1'b1."]
    #[inline(always)]
    #[must_use]
    pub fn u1_gmac5_axi64_scfg_ram_cfg_vg(&mut self) -> U1Gmac5Axi64ScfgRamCfgVgW<SysSyscfg35Spec> {
        U1Gmac5Axi64ScfgRamCfgVgW::new(self, 30)
    }
}
#[doc = "SYS SYSCONSAIF SYSCFG 140\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_syscfg35::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_syscfg35::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SysSyscfg35Spec;
impl crate::RegisterSpec for SysSyscfg35Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sys_syscfg35::R`](R) reader structure"]
impl crate::Readable for SysSyscfg35Spec {}
#[doc = "`write(|w| ..)` method takes [`sys_syscfg35::W`](W) writer structure"]
impl crate::Writable for SysSyscfg35Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets sys_syscfg35 to value 0x6aa0_0000"]
impl crate::Resettable for SysSyscfg35Spec {
    const RESET_VALUE: u32 = 0x6aa0_0000;
}
