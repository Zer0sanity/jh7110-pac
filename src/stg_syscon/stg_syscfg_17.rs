#[doc = "Register `stg_syscfg_17` reader"]
pub type R = crate::R<StgSyscfg17Spec>;
#[doc = "Register `stg_syscfg_17` writer"]
pub type W = crate::W<StgSyscfg17Spec>;
#[doc = "Field `u0_hifi4_scfg_sram_config_slp` reader - SRAM/ROM configuration. SLP: sleep enable, high active, default is low."]
pub type U0Hifi4ScfgSramConfigSlpR = crate::BitReader;
#[doc = "Field `u0_hifi4_scfg_sram_config_slp` writer - SRAM/ROM configuration. SLP: sleep enable, high active, default is low."]
pub type U0Hifi4ScfgSramConfigSlpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_hifi4_scfg_sram_config_sd` reader - SRAM/ROM configuration. SD: shutdown enable, high active, default is low."]
pub type U0Hifi4ScfgSramConfigSdR = crate::BitReader;
#[doc = "Field `u0_hifi4_scfg_sram_config_sd` writer - SRAM/ROM configuration. SD: shutdown enable, high active, default is low."]
pub type U0Hifi4ScfgSramConfigSdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_hifi4_scfg_sram_config_rtsel` reader - SRAM/ROM configuration. RTSEL: timing setting for debug purpose, default is 2'b01."]
pub type U0Hifi4ScfgSramConfigRtselR = crate::FieldReader;
#[doc = "Field `u0_hifi4_scfg_sram_config_rtsel` writer - SRAM/ROM configuration. RTSEL: timing setting for debug purpose, default is 2'b01."]
pub type U0Hifi4ScfgSramConfigRtselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `u0_hifi4_scfg_sram_config_ptsel` reader - SRAM/ROM configuration. PTSEL: timing setting for debug purpose, default is 2'b01."]
pub type U0Hifi4ScfgSramConfigPtselR = crate::FieldReader;
#[doc = "Field `u0_hifi4_scfg_sram_config_ptsel` writer - SRAM/ROM configuration. PTSEL: timing setting for debug purpose, default is 2'b01."]
pub type U0Hifi4ScfgSramConfigPtselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `u0_hifi4_scfg_sram_config_trb` reader - SRAM/ROM configuration. TRB: timing setting for debug purpose, default is 2'b01."]
pub type U0Hifi4ScfgSramConfigTrbR = crate::FieldReader;
#[doc = "Field `u0_hifi4_scfg_sram_config_trb` writer - SRAM/ROM configuration. TRB: timing setting for debug purpose, default is 2'b01."]
pub type U0Hifi4ScfgSramConfigTrbW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `u0_hifi4_scfg_sram_config_wtsel` reader - SRAM/ROM configuration. WTSEL: timing setting for debug purpose, default is 2'b01."]
pub type U0Hifi4ScfgSramConfigWtselR = crate::FieldReader;
#[doc = "Field `u0_hifi4_scfg_sram_config_wtsel` writer - SRAM/ROM configuration. WTSEL: timing setting for debug purpose, default is 2'b01."]
pub type U0Hifi4ScfgSramConfigWtselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `u0_hifi4_scfg_sram_config_vs` reader - SRAM/ROM configuration. VS: timing setting for debug purpose, default is 1'b1."]
pub type U0Hifi4ScfgSramConfigVsR = crate::BitReader;
#[doc = "Field `u0_hifi4_scfg_sram_config_vs` writer - SRAM/ROM configuration. VS: timing setting for debug purpose, default is 1'b1."]
pub type U0Hifi4ScfgSramConfigVsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_hifi4_scfg_sram_config_vg` reader - SRAM/ROM configuration. VG: timing setting for debug purpose, default is 1'b1."]
pub type U0Hifi4ScfgSramConfigVgR = crate::BitReader;
#[doc = "Field `u0_hifi4_scfg_sram_config_vg` writer - SRAM/ROM configuration. VG: timing setting for debug purpose, default is 1'b1."]
pub type U0Hifi4ScfgSramConfigVgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_hifi4_statvectorsel` reader - When the value is 1, it indicates that the AltResetVec is valid"]
pub type U0Hifi4StatvectorselR = crate::BitReader;
#[doc = "Field `u0_hifi4_statvectorsel` writer - When the value is 1, it indicates that the AltResetVec is valid"]
pub type U0Hifi4StatvectorselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_hifi4_trigin_idma` reader - DMA port trigger"]
pub type U0Hifi4TriginIdmaR = crate::BitReader;
#[doc = "Field `u0_hifi4_trigin_idma` writer - DMA port trigger"]
pub type U0Hifi4TriginIdmaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_hifi4_trigout_idma` reader - DMA port trigger"]
pub type U0Hifi4TrigoutIdmaR = crate::BitReader;
#[doc = "Field `u0_hifi4_xocdmode` reader - Debug signal"]
pub type U0Hifi4XocdmodeR = crate::BitReader;
#[doc = "Field `u0_pcie_align_detect` reader - "]
pub type U0PcieAlignDetectR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - SRAM/ROM configuration. SLP: sleep enable, high active, default is low."]
    #[inline(always)]
    pub fn u0_hifi4_scfg_sram_config_slp(&self) -> U0Hifi4ScfgSramConfigSlpR {
        U0Hifi4ScfgSramConfigSlpR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SRAM/ROM configuration. SD: shutdown enable, high active, default is low."]
    #[inline(always)]
    pub fn u0_hifi4_scfg_sram_config_sd(&self) -> U0Hifi4ScfgSramConfigSdR {
        U0Hifi4ScfgSramConfigSdR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - SRAM/ROM configuration. RTSEL: timing setting for debug purpose, default is 2'b01."]
    #[inline(always)]
    pub fn u0_hifi4_scfg_sram_config_rtsel(&self) -> U0Hifi4ScfgSramConfigRtselR {
        U0Hifi4ScfgSramConfigRtselR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - SRAM/ROM configuration. PTSEL: timing setting for debug purpose, default is 2'b01."]
    #[inline(always)]
    pub fn u0_hifi4_scfg_sram_config_ptsel(&self) -> U0Hifi4ScfgSramConfigPtselR {
        U0Hifi4ScfgSramConfigPtselR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - SRAM/ROM configuration. TRB: timing setting for debug purpose, default is 2'b01."]
    #[inline(always)]
    pub fn u0_hifi4_scfg_sram_config_trb(&self) -> U0Hifi4ScfgSramConfigTrbR {
        U0Hifi4ScfgSramConfigTrbR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - SRAM/ROM configuration. WTSEL: timing setting for debug purpose, default is 2'b01."]
    #[inline(always)]
    pub fn u0_hifi4_scfg_sram_config_wtsel(&self) -> U0Hifi4ScfgSramConfigWtselR {
        U0Hifi4ScfgSramConfigWtselR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - SRAM/ROM configuration. VS: timing setting for debug purpose, default is 1'b1."]
    #[inline(always)]
    pub fn u0_hifi4_scfg_sram_config_vs(&self) -> U0Hifi4ScfgSramConfigVsR {
        U0Hifi4ScfgSramConfigVsR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - SRAM/ROM configuration. VG: timing setting for debug purpose, default is 1'b1."]
    #[inline(always)]
    pub fn u0_hifi4_scfg_sram_config_vg(&self) -> U0Hifi4ScfgSramConfigVgR {
        U0Hifi4ScfgSramConfigVgR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - When the value is 1, it indicates that the AltResetVec is valid"]
    #[inline(always)]
    pub fn u0_hifi4_statvectorsel(&self) -> U0Hifi4StatvectorselR {
        U0Hifi4StatvectorselR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - DMA port trigger"]
    #[inline(always)]
    pub fn u0_hifi4_trigin_idma(&self) -> U0Hifi4TriginIdmaR {
        U0Hifi4TriginIdmaR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - DMA port trigger"]
    #[inline(always)]
    pub fn u0_hifi4_trigout_idma(&self) -> U0Hifi4TrigoutIdmaR {
        U0Hifi4TrigoutIdmaR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Debug signal"]
    #[inline(always)]
    pub fn u0_hifi4_xocdmode(&self) -> U0Hifi4XocdmodeR {
        U0Hifi4XocdmodeR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn u0_pcie_align_detect(&self) -> U0PcieAlignDetectR {
        U0PcieAlignDetectR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SRAM/ROM configuration. SLP: sleep enable, high active, default is low."]
    #[inline(always)]
    #[must_use]
    pub fn u0_hifi4_scfg_sram_config_slp(&mut self) -> U0Hifi4ScfgSramConfigSlpW<StgSyscfg17Spec> {
        U0Hifi4ScfgSramConfigSlpW::new(self, 0)
    }
    #[doc = "Bit 1 - SRAM/ROM configuration. SD: shutdown enable, high active, default is low."]
    #[inline(always)]
    #[must_use]
    pub fn u0_hifi4_scfg_sram_config_sd(&mut self) -> U0Hifi4ScfgSramConfigSdW<StgSyscfg17Spec> {
        U0Hifi4ScfgSramConfigSdW::new(self, 1)
    }
    #[doc = "Bits 2:3 - SRAM/ROM configuration. RTSEL: timing setting for debug purpose, default is 2'b01."]
    #[inline(always)]
    #[must_use]
    pub fn u0_hifi4_scfg_sram_config_rtsel(
        &mut self,
    ) -> U0Hifi4ScfgSramConfigRtselW<StgSyscfg17Spec> {
        U0Hifi4ScfgSramConfigRtselW::new(self, 2)
    }
    #[doc = "Bits 4:5 - SRAM/ROM configuration. PTSEL: timing setting for debug purpose, default is 2'b01."]
    #[inline(always)]
    #[must_use]
    pub fn u0_hifi4_scfg_sram_config_ptsel(
        &mut self,
    ) -> U0Hifi4ScfgSramConfigPtselW<StgSyscfg17Spec> {
        U0Hifi4ScfgSramConfigPtselW::new(self, 4)
    }
    #[doc = "Bits 6:7 - SRAM/ROM configuration. TRB: timing setting for debug purpose, default is 2'b01."]
    #[inline(always)]
    #[must_use]
    pub fn u0_hifi4_scfg_sram_config_trb(&mut self) -> U0Hifi4ScfgSramConfigTrbW<StgSyscfg17Spec> {
        U0Hifi4ScfgSramConfigTrbW::new(self, 6)
    }
    #[doc = "Bits 8:9 - SRAM/ROM configuration. WTSEL: timing setting for debug purpose, default is 2'b01."]
    #[inline(always)]
    #[must_use]
    pub fn u0_hifi4_scfg_sram_config_wtsel(
        &mut self,
    ) -> U0Hifi4ScfgSramConfigWtselW<StgSyscfg17Spec> {
        U0Hifi4ScfgSramConfigWtselW::new(self, 8)
    }
    #[doc = "Bit 10 - SRAM/ROM configuration. VS: timing setting for debug purpose, default is 1'b1."]
    #[inline(always)]
    #[must_use]
    pub fn u0_hifi4_scfg_sram_config_vs(&mut self) -> U0Hifi4ScfgSramConfigVsW<StgSyscfg17Spec> {
        U0Hifi4ScfgSramConfigVsW::new(self, 10)
    }
    #[doc = "Bit 11 - SRAM/ROM configuration. VG: timing setting for debug purpose, default is 1'b1."]
    #[inline(always)]
    #[must_use]
    pub fn u0_hifi4_scfg_sram_config_vg(&mut self) -> U0Hifi4ScfgSramConfigVgW<StgSyscfg17Spec> {
        U0Hifi4ScfgSramConfigVgW::new(self, 11)
    }
    #[doc = "Bit 12 - When the value is 1, it indicates that the AltResetVec is valid"]
    #[inline(always)]
    #[must_use]
    pub fn u0_hifi4_statvectorsel(&mut self) -> U0Hifi4StatvectorselW<StgSyscfg17Spec> {
        U0Hifi4StatvectorselW::new(self, 12)
    }
    #[doc = "Bit 13 - DMA port trigger"]
    #[inline(always)]
    #[must_use]
    pub fn u0_hifi4_trigin_idma(&mut self) -> U0Hifi4TriginIdmaW<StgSyscfg17Spec> {
        U0Hifi4TriginIdmaW::new(self, 13)
    }
}
#[doc = "STG SYSCONSAIF SYSCFG 68\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_syscfg_17::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stg_syscfg_17::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StgSyscfg17Spec;
impl crate::RegisterSpec for StgSyscfg17Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_syscfg_17::R`](R) reader structure"]
impl crate::Readable for StgSyscfg17Spec {}
#[doc = "`write(|w| ..)` method takes [`stg_syscfg_17::W`](W) writer structure"]
impl crate::Writable for StgSyscfg17Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets stg_syscfg_17 to value 0x0d54"]
impl crate::Resettable for StgSyscfg17Spec {
    const RESET_VALUE: u32 = 0x0d54;
}
