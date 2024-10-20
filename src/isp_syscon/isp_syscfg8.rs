#[doc = "Register `isp_syscfg8` reader"]
pub type R = crate::R<IspSyscfg8Spec>;
#[doc = "Register `isp_syscfg8` writer"]
pub type W = crate::W<IspSyscfg8Spec>;
#[doc = "Field `u0_vin_cfg_axiwr0_start_addr` reader - This bit represents the start address of the AXI output image."]
pub type U0VinCfgAxiwr0StartAddrR = crate::FieldReader<u32>;
#[doc = "Field `u0_vin_cfg_axiwr0_start_addr` writer - This bit represents the start address of the AXI output image."]
pub type U0VinCfgAxiwr0StartAddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - This bit represents the start address of the AXI output image."]
    #[inline(always)]
    pub fn u0_vin_cfg_axiwr0_start_addr(&self) -> U0VinCfgAxiwr0StartAddrR {
        U0VinCfgAxiwr0StartAddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - This bit represents the start address of the AXI output image."]
    #[inline(always)]
    #[must_use]
    pub fn u0_vin_cfg_axiwr0_start_addr(&mut self) -> U0VinCfgAxiwr0StartAddrW<IspSyscfg8Spec> {
        U0VinCfgAxiwr0StartAddrW::new(self, 0)
    }
}
#[doc = "ISP SYSCFG 8: isp_sysconsaif_syscfg_32\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isp_syscfg8::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isp_syscfg8::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IspSyscfg8Spec;
impl crate::RegisterSpec for IspSyscfg8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`isp_syscfg8::R`](R) reader structure"]
impl crate::Readable for IspSyscfg8Spec {}
#[doc = "`write(|w| ..)` method takes [`isp_syscfg8::W`](W) writer structure"]
impl crate::Writable for IspSyscfg8Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets isp_syscfg8 to value 0"]
impl crate::Resettable for IspSyscfg8Spec {
    const RESET_VALUE: u32 = 0;
}
