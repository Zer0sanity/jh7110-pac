#[doc = "Register `isp_syscfg4` reader"]
pub type R = crate::R<IspSyscfg4Spec>;
#[doc = "Register `isp_syscfg4` writer"]
pub type W = crate::W<IspSyscfg4Spec>;
#[doc = "Field `u0_vin_cfg_axird_start_addr` reader - This bit represents the valid start address of the AXI input test image's first line."]
pub type U0VinCfgAxirdStartAddrR = crate::FieldReader<u32>;
#[doc = "Field `u0_vin_cfg_axird_start_addr` writer - This bit represents the valid start address of the AXI input test image's first line."]
pub type U0VinCfgAxirdStartAddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - This bit represents the valid start address of the AXI input test image's first line."]
    #[inline(always)]
    pub fn u0_vin_cfg_axird_start_addr(&self) -> U0VinCfgAxirdStartAddrR {
        U0VinCfgAxirdStartAddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - This bit represents the valid start address of the AXI input test image's first line."]
    #[inline(always)]
    #[must_use]
    pub fn u0_vin_cfg_axird_start_addr(&mut self) -> U0VinCfgAxirdStartAddrW<IspSyscfg4Spec> {
        U0VinCfgAxirdStartAddrW::new(self, 0)
    }
}
#[doc = "ISP SYSCFG 4: isp_sysconsaif_syscfg_16\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isp_syscfg4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isp_syscfg4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IspSyscfg4Spec;
impl crate::RegisterSpec for IspSyscfg4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`isp_syscfg4::R`](R) reader structure"]
impl crate::Readable for IspSyscfg4Spec {}
#[doc = "`write(|w| ..)` method takes [`isp_syscfg4::W`](W) writer structure"]
impl crate::Writable for IspSyscfg4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets isp_syscfg4 to value 0"]
impl crate::Resettable for IspSyscfg4Spec {
    const RESET_VALUE: u32 = 0;
}
