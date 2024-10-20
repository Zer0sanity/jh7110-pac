#[doc = "Register `isp_syscfg0` reader"]
pub type R = crate::R<IspSyscfg0Spec>;
#[doc = "Register `isp_syscfg0` writer"]
pub type W = crate::W<IspSyscfg0Spec>;
#[doc = "Field `u0_vin_scfg_sram_config` reader - "]
pub type U0VinScfgSramConfigR = crate::FieldReader;
#[doc = "Field `u0_vin_scfg_sram_config` writer - "]
pub type U0VinScfgSramConfigW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `u0_vin_cfg_axi_dvp_en` reader - 0: Output to AXI is DVP, 1: Output to AXI is MIPI"]
pub type U0VinCfgAxiDvpEnR = crate::BitReader;
#[doc = "Field `u0_vin_cfg_axi_dvp_en` writer - 0: Output to AXI is DVP, 1: Output to AXI is MIPI"]
pub type U0VinCfgAxiDvpEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_vin_cfg_axird_axi_cnt_end` reader - The valid pixel of the AXI image. 1 pixel equals 64 bit."]
pub type U0VinCfgAxirdAxiCntEndR = crate::FieldReader<u16>;
#[doc = "Field `u0_vin_cfg_axird_axi_cnt_end` writer - The valid pixel of the AXI image. 1 pixel equals 64 bit."]
pub type U0VinCfgAxirdAxiCntEndW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn u0_vin_scfg_sram_config(&self) -> U0VinScfgSramConfigR {
        U0VinScfgSramConfigR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - 0: Output to AXI is DVP, 1: Output to AXI is MIPI"]
    #[inline(always)]
    pub fn u0_vin_cfg_axi_dvp_en(&self) -> U0VinCfgAxiDvpEnR {
        U0VinCfgAxiDvpEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:13 - The valid pixel of the AXI image. 1 pixel equals 64 bit."]
    #[inline(always)]
    pub fn u0_vin_cfg_axird_axi_cnt_end(&self) -> U0VinCfgAxirdAxiCntEndR {
        U0VinCfgAxirdAxiCntEndR::new(((self.bits >> 3) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    #[must_use]
    pub fn u0_vin_scfg_sram_config(&mut self) -> U0VinScfgSramConfigW<IspSyscfg0Spec> {
        U0VinScfgSramConfigW::new(self, 0)
    }
    #[doc = "Bit 2 - 0: Output to AXI is DVP, 1: Output to AXI is MIPI"]
    #[inline(always)]
    #[must_use]
    pub fn u0_vin_cfg_axi_dvp_en(&mut self) -> U0VinCfgAxiDvpEnW<IspSyscfg0Spec> {
        U0VinCfgAxiDvpEnW::new(self, 2)
    }
    #[doc = "Bits 3:13 - The valid pixel of the AXI image. 1 pixel equals 64 bit."]
    #[inline(always)]
    #[must_use]
    pub fn u0_vin_cfg_axird_axi_cnt_end(&mut self) -> U0VinCfgAxirdAxiCntEndW<IspSyscfg0Spec> {
        U0VinCfgAxirdAxiCntEndW::new(self, 3)
    }
}
#[doc = "ISP SYSCFG 0: isp_sysconsaif_syscfg_0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isp_syscfg0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isp_syscfg0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IspSyscfg0Spec;
impl crate::RegisterSpec for IspSyscfg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`isp_syscfg0::R`](R) reader structure"]
impl crate::Readable for IspSyscfg0Spec {}
#[doc = "`write(|w| ..)` method takes [`isp_syscfg0::W`](W) writer structure"]
impl crate::Writable for IspSyscfg0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets isp_syscfg0 to value 0"]
impl crate::Resettable for IspSyscfg0Spec {
    const RESET_VALUE: u32 = 0;
}
