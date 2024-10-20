#[doc = "Register `sys_syscfg33` reader"]
pub type R = crate::R<SysSyscfg33Spec>;
#[doc = "Register `sys_syscfg33` writer"]
pub type W = crate::W<SysSyscfg33Spec>;
#[doc = "Field `reset_vector_35_32_4` reader - Reset vector bits"]
pub type ResetVector35_32_4R = crate::FieldReader;
#[doc = "Field `reset_vector_35_32_4` writer - Reset vector bits"]
pub type ResetVector35_32_4W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `u0_suppress_fetch1` reader - "]
pub type U0SuppressFetch1R = crate::BitReader;
#[doc = "Field `u0_suppress_fetch1` writer - "]
pub type U0SuppressFetch1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_suppress_fetch2` reader - "]
pub type U0SuppressFetch2R = crate::BitReader;
#[doc = "Field `u0_suppress_fetch2` writer - "]
pub type U0SuppressFetch2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_suppress_fetch3` reader - "]
pub type U0SuppressFetch3R = crate::BitReader;
#[doc = "Field `u0_suppress_fetch3` writer - "]
pub type U0SuppressFetch3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_suppress_fetch4` reader - "]
pub type U0SuppressFetch4R = crate::BitReader;
#[doc = "Field `u0_suppress_fetch4` writer - "]
pub type U0SuppressFetch4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_wfi_from_tile0` reader - "]
pub type U0WfiFromTile0R = crate::BitReader;
#[doc = "Field `u0_wfi_from_tile0` writer - "]
pub type U0WfiFromTile0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_wfi_from_tile1` reader - "]
pub type U0WfiFromTile1R = crate::BitReader;
#[doc = "Field `u0_wfi_from_tile1` writer - "]
pub type U0WfiFromTile1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_wfi_from_tile2` reader - "]
pub type U0WfiFromTile2R = crate::BitReader;
#[doc = "Field `u0_wfi_from_tile2` writer - "]
pub type U0WfiFromTile2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_wfi_from_tile3` reader - "]
pub type U0WfiFromTile3R = crate::BitReader;
#[doc = "Field `u0_wfi_from_tile3` writer - "]
pub type U0WfiFromTile3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_wfi_from_tile4` reader - "]
pub type U0WfiFromTile4R = crate::BitReader;
#[doc = "Field `u0_wfi_from_tile4` writer - "]
pub type U0WfiFromTile4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_vdec_int_sram_config_slp` reader - SRAM/ROM configuration. SLP: sleep enable, high active, default is low."]
pub type U0VdecIntSramConfigSlpR = crate::BitReader;
#[doc = "Field `u0_vdec_int_sram_config_slp` writer - SRAM/ROM configuration. SLP: sleep enable, high active, default is low."]
pub type U0VdecIntSramConfigSlpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_vdec_int_sram_config_sd` reader - SRAM/ROM configuration. SD: shutdown enable, high active, default is low."]
pub type U0VdecIntSramConfigSdR = crate::BitReader;
#[doc = "Field `u0_vdec_int_sram_config_sd` writer - SRAM/ROM configuration. SD: shutdown enable, high active, default is low."]
pub type U0VdecIntSramConfigSdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_vdec_int_sram_config_rtsel` reader - SRAM/ROM configuration. RTSEL: timing setting for debug purpose, default is 2'b01."]
pub type U0VdecIntSramConfigRtselR = crate::FieldReader;
#[doc = "Field `u0_vdec_int_sram_config_rtsel` writer - SRAM/ROM configuration. RTSEL: timing setting for debug purpose, default is 2'b01."]
pub type U0VdecIntSramConfigRtselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `u0_vdec_int_sram_config_ptsel` reader - SRAM/ROM configuration. PTSEL: timing setting for debug purpose, default is 2'b01."]
pub type U0VdecIntSramConfigPtselR = crate::FieldReader;
#[doc = "Field `u0_vdec_int_sram_config_ptsel` writer - SRAM/ROM configuration. PTSEL: timing setting for debug purpose, default is 2'b01."]
pub type U0VdecIntSramConfigPtselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `u0_vdec_int_sram_config_trb` reader - SRAM/ROM configuration. TRB: timing setting for debug purpose, default is 2'b01."]
pub type U0VdecIntSramConfigTrbR = crate::FieldReader;
#[doc = "Field `u0_vdec_int_sram_config_trb` writer - SRAM/ROM configuration. TRB: timing setting for debug purpose, default is 2'b01."]
pub type U0VdecIntSramConfigTrbW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `u0_vdec_int_sram_config_wtsel` reader - SRAM/ROM configuration. WTSEL: timing setting for debug purpose, default is 2'b01."]
pub type U0VdecIntSramConfigWtselR = crate::FieldReader;
#[doc = "Field `u0_vdec_int_sram_config_wtsel` writer - SRAM/ROM configuration. WTSEL: timing setting for debug purpose, default is 2'b01."]
pub type U0VdecIntSramConfigWtselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `u0_vdec_int_sram_config_vs` reader - SRAM/ROM configuration. VS: timing setting for debug purpose, default is 1'b1."]
pub type U0VdecIntSramConfigVsR = crate::BitReader;
#[doc = "Field `u0_vdec_int_sram_config_vs` writer - SRAM/ROM configuration. VS: timing setting for debug purpose, default is 1'b1."]
pub type U0VdecIntSramConfigVsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_vdec_int_sram_config_vg` reader - SRAM/ROM configuration. VG: timing setting for debug purpose, default is 1'b1."]
pub type U0VdecIntSramConfigVgR = crate::BitReader;
#[doc = "Field `u0_vdec_int_sram_config_vg` writer - SRAM/ROM configuration. VG: timing setting for debug purpose, default is 1'b1."]
pub type U0VdecIntSramConfigVgW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Reset vector bits"]
    #[inline(always)]
    pub fn reset_vector_35_32_4(&self) -> ResetVector35_32_4R {
        ResetVector35_32_4R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn u0_suppress_fetch1(&self) -> U0SuppressFetch1R {
        U0SuppressFetch1R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn u0_suppress_fetch2(&self) -> U0SuppressFetch2R {
        U0SuppressFetch2R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn u0_suppress_fetch3(&self) -> U0SuppressFetch3R {
        U0SuppressFetch3R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn u0_suppress_fetch4(&self) -> U0SuppressFetch4R {
        U0SuppressFetch4R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn u0_wfi_from_tile0(&self) -> U0WfiFromTile0R {
        U0WfiFromTile0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn u0_wfi_from_tile1(&self) -> U0WfiFromTile1R {
        U0WfiFromTile1R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn u0_wfi_from_tile2(&self) -> U0WfiFromTile2R {
        U0WfiFromTile2R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn u0_wfi_from_tile3(&self) -> U0WfiFromTile3R {
        U0WfiFromTile3R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn u0_wfi_from_tile4(&self) -> U0WfiFromTile4R {
        U0WfiFromTile4R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - SRAM/ROM configuration. SLP: sleep enable, high active, default is low."]
    #[inline(always)]
    pub fn u0_vdec_int_sram_config_slp(&self) -> U0VdecIntSramConfigSlpR {
        U0VdecIntSramConfigSlpR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - SRAM/ROM configuration. SD: shutdown enable, high active, default is low."]
    #[inline(always)]
    pub fn u0_vdec_int_sram_config_sd(&self) -> U0VdecIntSramConfigSdR {
        U0VdecIntSramConfigSdR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 15:16 - SRAM/ROM configuration. RTSEL: timing setting for debug purpose, default is 2'b01."]
    #[inline(always)]
    pub fn u0_vdec_int_sram_config_rtsel(&self) -> U0VdecIntSramConfigRtselR {
        U0VdecIntSramConfigRtselR::new(((self.bits >> 15) & 3) as u8)
    }
    #[doc = "Bits 17:18 - SRAM/ROM configuration. PTSEL: timing setting for debug purpose, default is 2'b01."]
    #[inline(always)]
    pub fn u0_vdec_int_sram_config_ptsel(&self) -> U0VdecIntSramConfigPtselR {
        U0VdecIntSramConfigPtselR::new(((self.bits >> 17) & 3) as u8)
    }
    #[doc = "Bits 19:20 - SRAM/ROM configuration. TRB: timing setting for debug purpose, default is 2'b01."]
    #[inline(always)]
    pub fn u0_vdec_int_sram_config_trb(&self) -> U0VdecIntSramConfigTrbR {
        U0VdecIntSramConfigTrbR::new(((self.bits >> 19) & 3) as u8)
    }
    #[doc = "Bits 21:22 - SRAM/ROM configuration. WTSEL: timing setting for debug purpose, default is 2'b01."]
    #[inline(always)]
    pub fn u0_vdec_int_sram_config_wtsel(&self) -> U0VdecIntSramConfigWtselR {
        U0VdecIntSramConfigWtselR::new(((self.bits >> 21) & 3) as u8)
    }
    #[doc = "Bit 23 - SRAM/ROM configuration. VS: timing setting for debug purpose, default is 1'b1."]
    #[inline(always)]
    pub fn u0_vdec_int_sram_config_vs(&self) -> U0VdecIntSramConfigVsR {
        U0VdecIntSramConfigVsR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - SRAM/ROM configuration. VG: timing setting for debug purpose, default is 1'b1."]
    #[inline(always)]
    pub fn u0_vdec_int_sram_config_vg(&self) -> U0VdecIntSramConfigVgR {
        U0VdecIntSramConfigVgR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Reset vector bits"]
    #[inline(always)]
    #[must_use]
    pub fn reset_vector_35_32_4(&mut self) -> ResetVector35_32_4W<SysSyscfg33Spec> {
        ResetVector35_32_4W::new(self, 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn u0_suppress_fetch1(&mut self) -> U0SuppressFetch1W<SysSyscfg33Spec> {
        U0SuppressFetch1W::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn u0_suppress_fetch2(&mut self) -> U0SuppressFetch2W<SysSyscfg33Spec> {
        U0SuppressFetch2W::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn u0_suppress_fetch3(&mut self) -> U0SuppressFetch3W<SysSyscfg33Spec> {
        U0SuppressFetch3W::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn u0_suppress_fetch4(&mut self) -> U0SuppressFetch4W<SysSyscfg33Spec> {
        U0SuppressFetch4W::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn u0_wfi_from_tile0(&mut self) -> U0WfiFromTile0W<SysSyscfg33Spec> {
        U0WfiFromTile0W::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn u0_wfi_from_tile1(&mut self) -> U0WfiFromTile1W<SysSyscfg33Spec> {
        U0WfiFromTile1W::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn u0_wfi_from_tile2(&mut self) -> U0WfiFromTile2W<SysSyscfg33Spec> {
        U0WfiFromTile2W::new(self, 10)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn u0_wfi_from_tile3(&mut self) -> U0WfiFromTile3W<SysSyscfg33Spec> {
        U0WfiFromTile3W::new(self, 11)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn u0_wfi_from_tile4(&mut self) -> U0WfiFromTile4W<SysSyscfg33Spec> {
        U0WfiFromTile4W::new(self, 12)
    }
    #[doc = "Bit 13 - SRAM/ROM configuration. SLP: sleep enable, high active, default is low."]
    #[inline(always)]
    #[must_use]
    pub fn u0_vdec_int_sram_config_slp(&mut self) -> U0VdecIntSramConfigSlpW<SysSyscfg33Spec> {
        U0VdecIntSramConfigSlpW::new(self, 13)
    }
    #[doc = "Bit 14 - SRAM/ROM configuration. SD: shutdown enable, high active, default is low."]
    #[inline(always)]
    #[must_use]
    pub fn u0_vdec_int_sram_config_sd(&mut self) -> U0VdecIntSramConfigSdW<SysSyscfg33Spec> {
        U0VdecIntSramConfigSdW::new(self, 14)
    }
    #[doc = "Bits 15:16 - SRAM/ROM configuration. RTSEL: timing setting for debug purpose, default is 2'b01."]
    #[inline(always)]
    #[must_use]
    pub fn u0_vdec_int_sram_config_rtsel(&mut self) -> U0VdecIntSramConfigRtselW<SysSyscfg33Spec> {
        U0VdecIntSramConfigRtselW::new(self, 15)
    }
    #[doc = "Bits 17:18 - SRAM/ROM configuration. PTSEL: timing setting for debug purpose, default is 2'b01."]
    #[inline(always)]
    #[must_use]
    pub fn u0_vdec_int_sram_config_ptsel(&mut self) -> U0VdecIntSramConfigPtselW<SysSyscfg33Spec> {
        U0VdecIntSramConfigPtselW::new(self, 17)
    }
    #[doc = "Bits 19:20 - SRAM/ROM configuration. TRB: timing setting for debug purpose, default is 2'b01."]
    #[inline(always)]
    #[must_use]
    pub fn u0_vdec_int_sram_config_trb(&mut self) -> U0VdecIntSramConfigTrbW<SysSyscfg33Spec> {
        U0VdecIntSramConfigTrbW::new(self, 19)
    }
    #[doc = "Bits 21:22 - SRAM/ROM configuration. WTSEL: timing setting for debug purpose, default is 2'b01."]
    #[inline(always)]
    #[must_use]
    pub fn u0_vdec_int_sram_config_wtsel(&mut self) -> U0VdecIntSramConfigWtselW<SysSyscfg33Spec> {
        U0VdecIntSramConfigWtselW::new(self, 21)
    }
    #[doc = "Bit 23 - SRAM/ROM configuration. VS: timing setting for debug purpose, default is 1'b1."]
    #[inline(always)]
    #[must_use]
    pub fn u0_vdec_int_sram_config_vs(&mut self) -> U0VdecIntSramConfigVsW<SysSyscfg33Spec> {
        U0VdecIntSramConfigVsW::new(self, 23)
    }
    #[doc = "Bit 24 - SRAM/ROM configuration. VG: timing setting for debug purpose, default is 1'b1."]
    #[inline(always)]
    #[must_use]
    pub fn u0_vdec_int_sram_config_vg(&mut self) -> U0VdecIntSramConfigVgW<SysSyscfg33Spec> {
        U0VdecIntSramConfigVgW::new(self, 24)
    }
}
#[doc = "SYS SYSCONSAIF SYSCFG 132\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_syscfg33::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_syscfg33::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SysSyscfg33Spec;
impl crate::RegisterSpec for SysSyscfg33Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sys_syscfg33::R`](R) reader structure"]
impl crate::Readable for SysSyscfg33Spec {}
#[doc = "`write(|w| ..)` method takes [`sys_syscfg33::W`](W) writer structure"]
impl crate::Writable for SysSyscfg33Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets sys_syscfg33 to value 0x01aa_8000"]
impl crate::Resettable for SysSyscfg33Spec {
    const RESET_VALUE: u32 = 0x01aa_8000;
}
