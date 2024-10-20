#[doc = "Register `isp_syscfg1` reader"]
pub type R = crate::R<IspSyscfg1Spec>;
#[doc = "Register `isp_syscfg1` writer"]
pub type W = crate::W<IspSyscfg1Spec>;
#[doc = "Field `u0_vin_cfg_axird_end_addr` reader - the start address of the next frame"]
pub type U0VinCfgAxirdEndAddrR = crate::FieldReader<u32>;
#[doc = "Field `u0_vin_cfg_axird_end_addr` writer - the start address of the next frame"]
pub type U0VinCfgAxirdEndAddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - the start address of the next frame"]
    #[inline(always)]
    pub fn u0_vin_cfg_axird_end_addr(&self) -> U0VinCfgAxirdEndAddrR {
        U0VinCfgAxirdEndAddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - the start address of the next frame"]
    #[inline(always)]
    #[must_use]
    pub fn u0_vin_cfg_axird_end_addr(&mut self) -> U0VinCfgAxirdEndAddrW<IspSyscfg1Spec> {
        U0VinCfgAxirdEndAddrW::new(self, 0)
    }
}
#[doc = "ISP SYSCFG 1: isp_sysconsaif_syscfg_4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isp_syscfg1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isp_syscfg1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IspSyscfg1Spec;
impl crate::RegisterSpec for IspSyscfg1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`isp_syscfg1::R`](R) reader structure"]
impl crate::Readable for IspSyscfg1Spec {}
#[doc = "`write(|w| ..)` method takes [`isp_syscfg1::W`](W) writer structure"]
impl crate::Writable for IspSyscfg1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets isp_syscfg1 to value 0"]
impl crate::Resettable for IspSyscfg1Spec {
    const RESET_VALUE: u32 = 0;
}
