#[doc = "Register `sys_syscfg34` reader"]
pub type R = crate::R<SysSyscfg34Spec>;
#[doc = "Register `sys_syscfg34` writer"]
pub type W = crate::W<SysSyscfg34Spec>;
#[doc = "Field `u0_venc_int_sram_config_slp` reader - SRAM/ROM configuration. SLP: sleep enable, high active, default is low."]
pub type U0VencIntSramConfigSlpR = crate::BitReader;
#[doc = "Field `u0_venc_int_sram_config_slp` writer - SRAM/ROM configuration. SLP: sleep enable, high active, default is low."]
pub type U0VencIntSramConfigSlpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_venc_int_sram_config_sd` reader - SRAM/ROM configuration. SD: shutdown enable, high active, default is low."]
pub type U0VencIntSramConfigSdR = crate::BitReader;
#[doc = "Field `u0_venc_int_sram_config_sd` writer - SRAM/ROM configuration. SD: shutdown enable, high active, default is low."]
pub type U0VencIntSramConfigSdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_venc_int_sram_config_rtsel` reader - SRAM/ROM configuration. RTSEL: timing setting for debug purpose, default is 2'b01."]
pub type U0VencIntSramConfigRtselR = crate::FieldReader;
#[doc = "Field `u0_venc_int_sram_config_rtsel` writer - SRAM/ROM configuration. RTSEL: timing setting for debug purpose, default is 2'b01."]
pub type U0VencIntSramConfigRtselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `u0_venc_int_sram_config_ptsel` reader - SRAM/ROM configuration. PTSEL: timing setting for debug purpose, default is 2'b01."]
pub type U0VencIntSramConfigPtselR = crate::FieldReader;
#[doc = "Field `u0_venc_int_sram_config_ptsel` writer - SRAM/ROM configuration. PTSEL: timing setting for debug purpose, default is 2'b01."]
pub type U0VencIntSramConfigPtselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `u0_venc_int_sram_config_trb` reader - SRAM/ROM configuration. TRB: timing setting for debug purpose, default is 2'b01."]
pub type U0VencIntSramConfigTrbR = crate::FieldReader;
#[doc = "Field `u0_venc_int_sram_config_trb` writer - SRAM/ROM configuration. TRB: timing setting for debug purpose, default is 2'b01."]
pub type U0VencIntSramConfigTrbW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `u0_venc_int_sram_config_wtsel` reader - SRAM/ROM configuration. WTSEL: timing setting for debug purpose, default is 2'b01."]
pub type U0VencIntSramConfigWtselR = crate::FieldReader;
#[doc = "Field `u0_venc_int_sram_config_wtsel` writer - SRAM/ROM configuration. WTSEL: timing setting for debug purpose, default is 2'b01."]
pub type U0VencIntSramConfigWtselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `u0_venc_int_sram_config_vs` reader - SRAM/ROM configuration. VS: timing setting for debug purpose, default is 1'b1."]
pub type U0VencIntSramConfigVsR = crate::BitReader;
#[doc = "Field `u0_venc_int_sram_config_vs` writer - SRAM/ROM configuration. VS: timing setting for debug purpose, default is 1'b1."]
pub type U0VencIntSramConfigVsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_venc_int_sram_config_vg` reader - SRAM/ROM configuration. VG: timing setting for debug purpose, default is 1'b1."]
pub type U0VencIntSramConfigVgR = crate::BitReader;
#[doc = "Field `u0_venc_int_sram_config_vg` writer - SRAM/ROM configuration. VG: timing setting for debug purpose, default is 1'b1."]
pub type U0VencIntSramConfigVgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wave420l_ipu_current_buffer` reader - This signal indicates which buffer is currently active so that the VPU can correctly use the ipu_end_of_row signal for row counter."]
pub type Wave420lIpuCurrentBufferR = crate::FieldReader;
#[doc = "Field `wave420l_ipu_current_buffer` writer - This signal indicates which buffer is currently active so that the VPU can correctly use the ipu_end_of_row signal for row counter."]
pub type Wave420lIpuCurrentBufferW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `wave420l_ipu_end_of_row` reader - This signal is flipped every time when the IPU completes writing a row."]
pub type Wave420lIpuEndOfRowR = crate::BitReader;
#[doc = "Field `wave420l_ipu_end_of_row` writer - This signal is flipped every time when the IPU completes writing a row."]
pub type Wave420lIpuEndOfRowW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wave420l_ipu_new_frame` reader - This signal is flipped every time when the IPU completes writing a new frame."]
pub type Wave420lIpuNewFrameR = crate::BitReader;
#[doc = "Field `wave420l_ipu_new_frame` writer - This signal is flipped every time when the IPU completes writing a new frame."]
pub type Wave420lIpuNewFrameW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wave420l_vpu_idle` reader - VPU monitoring signal. This signal gives out an opposite value of VPU_BUSY register."]
pub type Wave420lVpuIdleR = crate::BitReader;
#[doc = "Field `can_ctrl_fd_enable_1` reader - "]
pub type CanCtrlFdEnable1R = crate::BitReader;
#[doc = "Field `can_ctrl_fd_enable_1` writer - "]
pub type CanCtrlFdEnable1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `can_ctrl_host_ecc_disable_1` reader - "]
pub type CanCtrlHostEccDisable1R = crate::BitReader;
#[doc = "Field `can_ctrl_host_ecc_disable_1` writer - "]
pub type CanCtrlHostEccDisable1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SRAM/ROM configuration. SLP: sleep enable, high active, default is low."]
    #[inline(always)]
    pub fn u0_venc_int_sram_config_slp(&self) -> U0VencIntSramConfigSlpR {
        U0VencIntSramConfigSlpR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SRAM/ROM configuration. SD: shutdown enable, high active, default is low."]
    #[inline(always)]
    pub fn u0_venc_int_sram_config_sd(&self) -> U0VencIntSramConfigSdR {
        U0VencIntSramConfigSdR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - SRAM/ROM configuration. RTSEL: timing setting for debug purpose, default is 2'b01."]
    #[inline(always)]
    pub fn u0_venc_int_sram_config_rtsel(&self) -> U0VencIntSramConfigRtselR {
        U0VencIntSramConfigRtselR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - SRAM/ROM configuration. PTSEL: timing setting for debug purpose, default is 2'b01."]
    #[inline(always)]
    pub fn u0_venc_int_sram_config_ptsel(&self) -> U0VencIntSramConfigPtselR {
        U0VencIntSramConfigPtselR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - SRAM/ROM configuration. TRB: timing setting for debug purpose, default is 2'b01."]
    #[inline(always)]
    pub fn u0_venc_int_sram_config_trb(&self) -> U0VencIntSramConfigTrbR {
        U0VencIntSramConfigTrbR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - SRAM/ROM configuration. WTSEL: timing setting for debug purpose, default is 2'b01."]
    #[inline(always)]
    pub fn u0_venc_int_sram_config_wtsel(&self) -> U0VencIntSramConfigWtselR {
        U0VencIntSramConfigWtselR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - SRAM/ROM configuration. VS: timing setting for debug purpose, default is 1'b1."]
    #[inline(always)]
    pub fn u0_venc_int_sram_config_vs(&self) -> U0VencIntSramConfigVsR {
        U0VencIntSramConfigVsR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - SRAM/ROM configuration. VG: timing setting for debug purpose, default is 1'b1."]
    #[inline(always)]
    pub fn u0_venc_int_sram_config_vg(&self) -> U0VencIntSramConfigVgR {
        U0VencIntSramConfigVgR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14 - This signal indicates which buffer is currently active so that the VPU can correctly use the ipu_end_of_row signal for row counter."]
    #[inline(always)]
    pub fn wave420l_ipu_current_buffer(&self) -> Wave420lIpuCurrentBufferR {
        Wave420lIpuCurrentBufferR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - This signal is flipped every time when the IPU completes writing a row."]
    #[inline(always)]
    pub fn wave420l_ipu_end_of_row(&self) -> Wave420lIpuEndOfRowR {
        Wave420lIpuEndOfRowR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - This signal is flipped every time when the IPU completes writing a new frame."]
    #[inline(always)]
    pub fn wave420l_ipu_new_frame(&self) -> Wave420lIpuNewFrameR {
        Wave420lIpuNewFrameR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - VPU monitoring signal. This signal gives out an opposite value of VPU_BUSY register."]
    #[inline(always)]
    pub fn wave420l_vpu_idle(&self) -> Wave420lVpuIdleR {
        Wave420lVpuIdleR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn can_ctrl_fd_enable_1(&self) -> CanCtrlFdEnable1R {
        CanCtrlFdEnable1R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn can_ctrl_host_ecc_disable_1(&self) -> CanCtrlHostEccDisable1R {
        CanCtrlHostEccDisable1R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SRAM/ROM configuration. SLP: sleep enable, high active, default is low."]
    #[inline(always)]
    #[must_use]
    pub fn u0_venc_int_sram_config_slp(&mut self) -> U0VencIntSramConfigSlpW<SysSyscfg34Spec> {
        U0VencIntSramConfigSlpW::new(self, 0)
    }
    #[doc = "Bit 1 - SRAM/ROM configuration. SD: shutdown enable, high active, default is low."]
    #[inline(always)]
    #[must_use]
    pub fn u0_venc_int_sram_config_sd(&mut self) -> U0VencIntSramConfigSdW<SysSyscfg34Spec> {
        U0VencIntSramConfigSdW::new(self, 1)
    }
    #[doc = "Bits 2:3 - SRAM/ROM configuration. RTSEL: timing setting for debug purpose, default is 2'b01."]
    #[inline(always)]
    #[must_use]
    pub fn u0_venc_int_sram_config_rtsel(&mut self) -> U0VencIntSramConfigRtselW<SysSyscfg34Spec> {
        U0VencIntSramConfigRtselW::new(self, 2)
    }
    #[doc = "Bits 4:5 - SRAM/ROM configuration. PTSEL: timing setting for debug purpose, default is 2'b01."]
    #[inline(always)]
    #[must_use]
    pub fn u0_venc_int_sram_config_ptsel(&mut self) -> U0VencIntSramConfigPtselW<SysSyscfg34Spec> {
        U0VencIntSramConfigPtselW::new(self, 4)
    }
    #[doc = "Bits 6:7 - SRAM/ROM configuration. TRB: timing setting for debug purpose, default is 2'b01."]
    #[inline(always)]
    #[must_use]
    pub fn u0_venc_int_sram_config_trb(&mut self) -> U0VencIntSramConfigTrbW<SysSyscfg34Spec> {
        U0VencIntSramConfigTrbW::new(self, 6)
    }
    #[doc = "Bits 8:9 - SRAM/ROM configuration. WTSEL: timing setting for debug purpose, default is 2'b01."]
    #[inline(always)]
    #[must_use]
    pub fn u0_venc_int_sram_config_wtsel(&mut self) -> U0VencIntSramConfigWtselW<SysSyscfg34Spec> {
        U0VencIntSramConfigWtselW::new(self, 8)
    }
    #[doc = "Bit 10 - SRAM/ROM configuration. VS: timing setting for debug purpose, default is 1'b1."]
    #[inline(always)]
    #[must_use]
    pub fn u0_venc_int_sram_config_vs(&mut self) -> U0VencIntSramConfigVsW<SysSyscfg34Spec> {
        U0VencIntSramConfigVsW::new(self, 10)
    }
    #[doc = "Bit 11 - SRAM/ROM configuration. VG: timing setting for debug purpose, default is 1'b1."]
    #[inline(always)]
    #[must_use]
    pub fn u0_venc_int_sram_config_vg(&mut self) -> U0VencIntSramConfigVgW<SysSyscfg34Spec> {
        U0VencIntSramConfigVgW::new(self, 11)
    }
    #[doc = "Bits 12:14 - This signal indicates which buffer is currently active so that the VPU can correctly use the ipu_end_of_row signal for row counter."]
    #[inline(always)]
    #[must_use]
    pub fn wave420l_ipu_current_buffer(&mut self) -> Wave420lIpuCurrentBufferW<SysSyscfg34Spec> {
        Wave420lIpuCurrentBufferW::new(self, 12)
    }
    #[doc = "Bit 15 - This signal is flipped every time when the IPU completes writing a row."]
    #[inline(always)]
    #[must_use]
    pub fn wave420l_ipu_end_of_row(&mut self) -> Wave420lIpuEndOfRowW<SysSyscfg34Spec> {
        Wave420lIpuEndOfRowW::new(self, 15)
    }
    #[doc = "Bit 16 - This signal is flipped every time when the IPU completes writing a new frame."]
    #[inline(always)]
    #[must_use]
    pub fn wave420l_ipu_new_frame(&mut self) -> Wave420lIpuNewFrameW<SysSyscfg34Spec> {
        Wave420lIpuNewFrameW::new(self, 16)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    #[must_use]
    pub fn can_ctrl_fd_enable_1(&mut self) -> CanCtrlFdEnable1W<SysSyscfg34Spec> {
        CanCtrlFdEnable1W::new(self, 18)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    #[must_use]
    pub fn can_ctrl_host_ecc_disable_1(&mut self) -> CanCtrlHostEccDisable1W<SysSyscfg34Spec> {
        CanCtrlHostEccDisable1W::new(self, 19)
    }
}
#[doc = "SYS SYSCONSAIF SYSCFG 136\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_syscfg34::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_syscfg34::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SysSyscfg34Spec;
impl crate::RegisterSpec for SysSyscfg34Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sys_syscfg34::R`](R) reader structure"]
impl crate::Readable for SysSyscfg34Spec {}
#[doc = "`write(|w| ..)` method takes [`sys_syscfg34::W`](W) writer structure"]
impl crate::Writable for SysSyscfg34Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets sys_syscfg34 to value 0x0d54"]
impl crate::Resettable for SysSyscfg34Spec {
    const RESET_VALUE: u32 = 0x0d54;
}
