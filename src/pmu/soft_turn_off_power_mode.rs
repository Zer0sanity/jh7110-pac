#[doc = "Register `soft_turn_off_power_mode` reader"]
pub type R = crate::R<SoftTurnOffPowerModeSpec>;
#[doc = "Register `soft_turn_off_power_mode` writer"]
pub type W = crate::W<SoftTurnOffPowerModeSpec>;
#[doc = "Field `systop_power_mode` reader - SYSTOP turn-off power mode"]
pub type SystopPowerModeR = crate::BitReader;
#[doc = "Field `systop_power_mode` writer - SYSTOP turn-off power mode"]
pub type SystopPowerModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cpu_power_mode` reader - CPU turn-off power mode"]
pub type CpuPowerModeR = crate::BitReader;
#[doc = "Field `cpu_power_mode` writer - CPU turn-off power mode"]
pub type CpuPowerModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `gpua_power_mode` reader - GPUA turn-off power mode"]
pub type GpuaPowerModeR = crate::BitReader;
#[doc = "Field `gpua_power_mode` writer - GPUA turn-off power mode"]
pub type GpuaPowerModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `vdec_power_mode` reader - VDEC turn-off power mode"]
pub type VdecPowerModeR = crate::BitReader;
#[doc = "Field `vdec_power_mode` writer - VDEC turn-off power mode"]
pub type VdecPowerModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `vout_power_mode` reader - VOUT turn-off power mode"]
pub type VoutPowerModeR = crate::BitReader;
#[doc = "Field `vout_power_mode` writer - VOUT turn-off power mode"]
pub type VoutPowerModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `isp_power_mode` reader - ISP turn-off power mode"]
pub type IspPowerModeR = crate::BitReader;
#[doc = "Field `isp_power_mode` writer - ISP turn-off power mode"]
pub type IspPowerModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `venc_power_mode` reader - VENC turn-off power mode"]
pub type VencPowerModeR = crate::BitReader;
#[doc = "Field `venc_power_mode` writer - VENC turn-off power mode"]
pub type VencPowerModeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SYSTOP turn-off power mode"]
    #[inline(always)]
    pub fn systop_power_mode(&self) -> SystopPowerModeR {
        SystopPowerModeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CPU turn-off power mode"]
    #[inline(always)]
    pub fn cpu_power_mode(&self) -> CpuPowerModeR {
        CpuPowerModeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - GPUA turn-off power mode"]
    #[inline(always)]
    pub fn gpua_power_mode(&self) -> GpuaPowerModeR {
        GpuaPowerModeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - VDEC turn-off power mode"]
    #[inline(always)]
    pub fn vdec_power_mode(&self) -> VdecPowerModeR {
        VdecPowerModeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - VOUT turn-off power mode"]
    #[inline(always)]
    pub fn vout_power_mode(&self) -> VoutPowerModeR {
        VoutPowerModeR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ISP turn-off power mode"]
    #[inline(always)]
    pub fn isp_power_mode(&self) -> IspPowerModeR {
        IspPowerModeR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - VENC turn-off power mode"]
    #[inline(always)]
    pub fn venc_power_mode(&self) -> VencPowerModeR {
        VencPowerModeR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SYSTOP turn-off power mode"]
    #[inline(always)]
    #[must_use]
    pub fn systop_power_mode(&mut self) -> SystopPowerModeW<SoftTurnOffPowerModeSpec> {
        SystopPowerModeW::new(self, 0)
    }
    #[doc = "Bit 1 - CPU turn-off power mode"]
    #[inline(always)]
    #[must_use]
    pub fn cpu_power_mode(&mut self) -> CpuPowerModeW<SoftTurnOffPowerModeSpec> {
        CpuPowerModeW::new(self, 1)
    }
    #[doc = "Bit 2 - GPUA turn-off power mode"]
    #[inline(always)]
    #[must_use]
    pub fn gpua_power_mode(&mut self) -> GpuaPowerModeW<SoftTurnOffPowerModeSpec> {
        GpuaPowerModeW::new(self, 2)
    }
    #[doc = "Bit 3 - VDEC turn-off power mode"]
    #[inline(always)]
    #[must_use]
    pub fn vdec_power_mode(&mut self) -> VdecPowerModeW<SoftTurnOffPowerModeSpec> {
        VdecPowerModeW::new(self, 3)
    }
    #[doc = "Bit 4 - VOUT turn-off power mode"]
    #[inline(always)]
    #[must_use]
    pub fn vout_power_mode(&mut self) -> VoutPowerModeW<SoftTurnOffPowerModeSpec> {
        VoutPowerModeW::new(self, 4)
    }
    #[doc = "Bit 5 - ISP turn-off power mode"]
    #[inline(always)]
    #[must_use]
    pub fn isp_power_mode(&mut self) -> IspPowerModeW<SoftTurnOffPowerModeSpec> {
        IspPowerModeW::new(self, 5)
    }
    #[doc = "Bit 6 - VENC turn-off power mode"]
    #[inline(always)]
    #[must_use]
    pub fn venc_power_mode(&mut self) -> VencPowerModeW<SoftTurnOffPowerModeSpec> {
        VencPowerModeW::new(self, 6)
    }
}
#[doc = "Software Turn-Off Power Mode\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`soft_turn_off_power_mode::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`soft_turn_off_power_mode::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SoftTurnOffPowerModeSpec;
impl crate::RegisterSpec for SoftTurnOffPowerModeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`soft_turn_off_power_mode::R`](R) reader structure"]
impl crate::Readable for SoftTurnOffPowerModeSpec {}
#[doc = "`write(|w| ..)` method takes [`soft_turn_off_power_mode::W`](W) writer structure"]
impl crate::Writable for SoftTurnOffPowerModeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets soft_turn_off_power_mode to value 0"]
impl crate::Resettable for SoftTurnOffPowerModeSpec {
    const RESET_VALUE: u32 = 0;
}
