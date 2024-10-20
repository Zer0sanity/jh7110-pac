#[doc = "Register `isp_syscfg5` reader"]
pub type R = crate::R<IspSyscfg5Spec>;
#[doc = "Register `isp_syscfg5` writer"]
pub type W = crate::W<IspSyscfg5Spec>;
#[doc = "Field `u0_vin_cfg_axiwr0_channel_sel` reader - Select 1 channel output of the 8 MIPI channels"]
pub type U0VinCfgAxiwr0ChannelSelR = crate::FieldReader;
#[doc = "Field `u0_vin_cfg_axiwr0_channel_sel` writer - Select 1 channel output of the 8 MIPI channels"]
pub type U0VinCfgAxiwr0ChannelSelW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `u0_vin_cfg_axiwr0_en` reader - Set this bit to 1 to enable the image output to AXI."]
pub type U0VinCfgAxiwr0EnR = crate::BitReader;
#[doc = "Field `u0_vin_cfg_axiwr0_en` writer - Set this bit to 1 to enable the image output to AXI."]
pub type U0VinCfgAxiwr0EnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Select 1 channel output of the 8 MIPI channels"]
    #[inline(always)]
    pub fn u0_vin_cfg_axiwr0_channel_sel(&self) -> U0VinCfgAxiwr0ChannelSelR {
        U0VinCfgAxiwr0ChannelSelR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Set this bit to 1 to enable the image output to AXI."]
    #[inline(always)]
    pub fn u0_vin_cfg_axiwr0_en(&self) -> U0VinCfgAxiwr0EnR {
        U0VinCfgAxiwr0EnR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Select 1 channel output of the 8 MIPI channels"]
    #[inline(always)]
    #[must_use]
    pub fn u0_vin_cfg_axiwr0_channel_sel(&mut self) -> U0VinCfgAxiwr0ChannelSelW<IspSyscfg5Spec> {
        U0VinCfgAxiwr0ChannelSelW::new(self, 0)
    }
    #[doc = "Bit 4 - Set this bit to 1 to enable the image output to AXI."]
    #[inline(always)]
    #[must_use]
    pub fn u0_vin_cfg_axiwr0_en(&mut self) -> U0VinCfgAxiwr0EnW<IspSyscfg5Spec> {
        U0VinCfgAxiwr0EnW::new(self, 4)
    }
}
#[doc = "ISP SYSCFG 5: isp_sysconsaif_syscfg_20\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isp_syscfg5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isp_syscfg5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IspSyscfg5Spec;
impl crate::RegisterSpec for IspSyscfg5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`isp_syscfg5::R`](R) reader structure"]
impl crate::Readable for IspSyscfg5Spec {}
#[doc = "`write(|w| ..)` method takes [`isp_syscfg5::W`](W) writer structure"]
impl crate::Writable for IspSyscfg5Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets isp_syscfg5 to value 0"]
impl crate::Resettable for IspSyscfg5Spec {
    const RESET_VALUE: u32 = 0;
}
