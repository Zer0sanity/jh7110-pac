#[doc = "Register `current_power_mode` reader"]
pub type R = crate::R<CurrentPowerModeSpec>;
#[doc = "Register `current_power_mode` writer"]
pub type W = crate::W<CurrentPowerModeSpec>;
#[doc = "Field `systop_power_mode` reader - SYSTOP turn-on power mode"]
pub type SystopPowerModeR = crate::BitReader;
#[doc = "Field `systop_power_mode` writer - SYSTOP turn-on power mode"]
pub type SystopPowerModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cpu_power_mode` reader - CPU turn-on power mode"]
pub type CpuPowerModeR = crate::BitReader;
#[doc = "Field `cpu_power_mode` writer - CPU turn-on power mode"]
pub type CpuPowerModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `gpua_power_mode` reader - GPUA turn-on power mode"]
pub type GpuaPowerModeR = crate::BitReader;
#[doc = "Field `gpua_power_mode` writer - GPUA turn-on power mode"]
pub type GpuaPowerModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `vdec_power_mode` reader - VDEC turn-on power mode"]
pub type VdecPowerModeR = crate::BitReader;
#[doc = "Field `vdec_power_mode` writer - VDEC turn-on power mode"]
pub type VdecPowerModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `vout_power_mode` reader - VOUT turn-on power mode"]
pub type VoutPowerModeR = crate::BitReader;
#[doc = "Field `vout_power_mode` writer - VOUT turn-on power mode"]
pub type VoutPowerModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `isp_power_mode` reader - ISP turn-on power mode"]
pub type IspPowerModeR = crate::BitReader;
#[doc = "Field `isp_power_mode` writer - ISP turn-on power mode"]
pub type IspPowerModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `venc_power_mode` reader - VENC turn-on power mode"]
pub type VencPowerModeR = crate::BitReader;
#[doc = "Field `venc_power_mode` writer - VENC turn-on power mode"]
pub type VencPowerModeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SYSTOP turn-on power mode"]
    #[inline(always)]
    pub fn systop_power_mode(&self) -> SystopPowerModeR {
        SystopPowerModeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CPU turn-on power mode"]
    #[inline(always)]
    pub fn cpu_power_mode(&self) -> CpuPowerModeR {
        CpuPowerModeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - GPUA turn-on power mode"]
    #[inline(always)]
    pub fn gpua_power_mode(&self) -> GpuaPowerModeR {
        GpuaPowerModeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - VDEC turn-on power mode"]
    #[inline(always)]
    pub fn vdec_power_mode(&self) -> VdecPowerModeR {
        VdecPowerModeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - VOUT turn-on power mode"]
    #[inline(always)]
    pub fn vout_power_mode(&self) -> VoutPowerModeR {
        VoutPowerModeR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ISP turn-on power mode"]
    #[inline(always)]
    pub fn isp_power_mode(&self) -> IspPowerModeR {
        IspPowerModeR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - VENC turn-on power mode"]
    #[inline(always)]
    pub fn venc_power_mode(&self) -> VencPowerModeR {
        VencPowerModeR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SYSTOP turn-on power mode"]
    #[inline(always)]
    #[must_use]
    pub fn systop_power_mode(&mut self) -> SystopPowerModeW<CurrentPowerModeSpec> {
        SystopPowerModeW::new(self, 0)
    }
    #[doc = "Bit 1 - CPU turn-on power mode"]
    #[inline(always)]
    #[must_use]
    pub fn cpu_power_mode(&mut self) -> CpuPowerModeW<CurrentPowerModeSpec> {
        CpuPowerModeW::new(self, 1)
    }
    #[doc = "Bit 2 - GPUA turn-on power mode"]
    #[inline(always)]
    #[must_use]
    pub fn gpua_power_mode(&mut self) -> GpuaPowerModeW<CurrentPowerModeSpec> {
        GpuaPowerModeW::new(self, 2)
    }
    #[doc = "Bit 3 - VDEC turn-on power mode"]
    #[inline(always)]
    #[must_use]
    pub fn vdec_power_mode(&mut self) -> VdecPowerModeW<CurrentPowerModeSpec> {
        VdecPowerModeW::new(self, 3)
    }
    #[doc = "Bit 4 - VOUT turn-on power mode"]
    #[inline(always)]
    #[must_use]
    pub fn vout_power_mode(&mut self) -> VoutPowerModeW<CurrentPowerModeSpec> {
        VoutPowerModeW::new(self, 4)
    }
    #[doc = "Bit 5 - ISP turn-on power mode"]
    #[inline(always)]
    #[must_use]
    pub fn isp_power_mode(&mut self) -> IspPowerModeW<CurrentPowerModeSpec> {
        IspPowerModeW::new(self, 5)
    }
    #[doc = "Bit 6 - VENC turn-on power mode"]
    #[inline(always)]
    #[must_use]
    pub fn venc_power_mode(&mut self) -> VencPowerModeW<CurrentPowerModeSpec> {
        VencPowerModeW::new(self, 6)
    }
}
#[doc = "Current Power Mode register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`current_power_mode::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`current_power_mode::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CurrentPowerModeSpec;
impl crate::RegisterSpec for CurrentPowerModeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`current_power_mode::R`](R) reader structure"]
impl crate::Readable for CurrentPowerModeSpec {}
#[doc = "`write(|w| ..)` method takes [`current_power_mode::W`](W) writer structure"]
impl crate::Writable for CurrentPowerModeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets current_power_mode to value 0"]
impl crate::Resettable for CurrentPowerModeSpec {
    const RESET_VALUE: u32 = 0;
}
