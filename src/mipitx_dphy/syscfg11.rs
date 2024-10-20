#[doc = "Register `syscfg11` reader"]
pub type R = crate::R<Syscfg11Spec>;
#[doc = "Register `syscfg11` writer"]
pub type W = crate::W<Syscfg11Spec>;
#[doc = "Field `scfg_c_hs_pre_zero_time` reader - SCFG C HS Pre Zero Time: u0_mipitx_dphy_SCFG_c_hs_pre_zero_time"]
pub type ScfgCHsPreZeroTimeR = crate::FieldReader<u32>;
#[doc = "Field `scfg_c_hs_pre_zero_time` writer - SCFG C HS Pre Zero Time: u0_mipitx_dphy_SCFG_c_hs_pre_zero_time"]
pub type ScfgCHsPreZeroTimeW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - SCFG C HS Pre Zero Time: u0_mipitx_dphy_SCFG_c_hs_pre_zero_time"]
    #[inline(always)]
    pub fn scfg_c_hs_pre_zero_time(&self) -> ScfgCHsPreZeroTimeR {
        ScfgCHsPreZeroTimeR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - SCFG C HS Pre Zero Time: u0_mipitx_dphy_SCFG_c_hs_pre_zero_time"]
    #[inline(always)]
    #[must_use]
    pub fn scfg_c_hs_pre_zero_time(&mut self) -> ScfgCHsPreZeroTimeW<Syscfg11Spec> {
        ScfgCHsPreZeroTimeW::new(self, 0)
    }
}
#[doc = "MIPITX DPHY SYSCFG 11: mipitx_apbifsaif_syscfg_44\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syscfg11::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`syscfg11::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Syscfg11Spec;
impl crate::RegisterSpec for Syscfg11Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`syscfg11::R`](R) reader structure"]
impl crate::Readable for Syscfg11Spec {}
#[doc = "`write(|w| ..)` method takes [`syscfg11::W`](W) writer structure"]
impl crate::Writable for Syscfg11Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets syscfg11 to value 0"]
impl crate::Resettable for Syscfg11Spec {
    const RESET_VALUE: u32 = 0;
}
