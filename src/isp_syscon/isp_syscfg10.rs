#[doc = "Register `isp_syscfg10` reader"]
pub type R = crate::R<IspSyscfg10Spec>;
#[doc = "Register `isp_syscfg10` writer"]
pub type W = crate::W<IspSyscfg10Spec>;
#[doc = "Field `u0_vin_test_generic_ctrl` reader - This configuration is not used by JH7110."]
pub type U0VinTestGenericCtrlR = crate::FieldReader<u16>;
#[doc = "Field `u0_vin_test_generic_status` reader - This configuration is not used by JH7110."]
pub type U0VinTestGenericStatusR = crate::FieldReader<u16>;
#[doc = "Field `u0_vin_test_generic_status` writer - This configuration is not used by JH7110."]
pub type U0VinTestGenericStatusW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - This configuration is not used by JH7110."]
    #[inline(always)]
    pub fn u0_vin_test_generic_ctrl(&self) -> U0VinTestGenericCtrlR {
        U0VinTestGenericCtrlR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - This configuration is not used by JH7110."]
    #[inline(always)]
    pub fn u0_vin_test_generic_status(&self) -> U0VinTestGenericStatusR {
        U0VinTestGenericStatusR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - This configuration is not used by JH7110."]
    #[inline(always)]
    #[must_use]
    pub fn u0_vin_test_generic_status(&mut self) -> U0VinTestGenericStatusW<IspSyscfg10Spec> {
        U0VinTestGenericStatusW::new(self, 16)
    }
}
#[doc = "ISP SYSCFG 10: isp_sysconsaif_syscfg_40\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isp_syscfg10::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isp_syscfg10::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IspSyscfg10Spec;
impl crate::RegisterSpec for IspSyscfg10Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`isp_syscfg10::R`](R) reader structure"]
impl crate::Readable for IspSyscfg10Spec {}
#[doc = "`write(|w| ..)` method takes [`isp_syscfg10::W`](W) writer structure"]
impl crate::Writable for IspSyscfg10Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets isp_syscfg10 to value 0"]
impl crate::Resettable for IspSyscfg10Spec {
    const RESET_VALUE: u32 = 0;
}
