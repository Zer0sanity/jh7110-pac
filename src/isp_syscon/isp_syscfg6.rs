#[doc = "Register `isp_syscfg6` reader"]
pub type R = crate::R<IspSyscfg6Spec>;
#[doc = "Register `isp_syscfg6` writer"]
pub type W = crate::W<IspSyscfg6Spec>;
#[doc = "Field `u0_vin_cfg_axiwr0_end_addr` reader - This bit represents the start address of the next frame."]
pub type U0VinCfgAxiwr0EndAddrR = crate::FieldReader<u32>;
#[doc = "Field `u0_vin_cfg_axiwr0_end_addr` writer - This bit represents the start address of the next frame."]
pub type U0VinCfgAxiwr0EndAddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - This bit represents the start address of the next frame."]
    #[inline(always)]
    pub fn u0_vin_cfg_axiwr0_end_addr(&self) -> U0VinCfgAxiwr0EndAddrR {
        U0VinCfgAxiwr0EndAddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - This bit represents the start address of the next frame."]
    #[inline(always)]
    #[must_use]
    pub fn u0_vin_cfg_axiwr0_end_addr(&mut self) -> U0VinCfgAxiwr0EndAddrW<IspSyscfg6Spec> {
        U0VinCfgAxiwr0EndAddrW::new(self, 0)
    }
}
#[doc = "ISP SYSCFG 6: isp_sysconsaif_syscfg_24\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isp_syscfg6::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isp_syscfg6::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IspSyscfg6Spec;
impl crate::RegisterSpec for IspSyscfg6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`isp_syscfg6::R`](R) reader structure"]
impl crate::Readable for IspSyscfg6Spec {}
#[doc = "`write(|w| ..)` method takes [`isp_syscfg6::W`](W) writer structure"]
impl crate::Writable for IspSyscfg6Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets isp_syscfg6 to value 0"]
impl crate::Resettable for IspSyscfg6Spec {
    const RESET_VALUE: u32 = 0;
}
