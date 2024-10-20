#[doc = "Register `syscfg10` reader"]
pub type R = crate::R<Syscfg10Spec>;
#[doc = "Register `syscfg10` writer"]
pub type W = crate::W<Syscfg10Spec>;
#[doc = "Field `rg_extd_cycle_sel` reader - RG EXTD Cycle Select: u0_mipitx_dphy_RG_EXTD_CYCLE_SEL"]
pub type RgExtdCycleSelR = crate::FieldReader;
#[doc = "Field `rg_extd_cycle_sel` writer - RG EXTD Cycle Select: u0_mipitx_dphy_RG_EXTD_CYCLE_SEL"]
pub type RgExtdCycleSelW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - RG EXTD Cycle Select: u0_mipitx_dphy_RG_EXTD_CYCLE_SEL"]
    #[inline(always)]
    pub fn rg_extd_cycle_sel(&self) -> RgExtdCycleSelR {
        RgExtdCycleSelR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - RG EXTD Cycle Select: u0_mipitx_dphy_RG_EXTD_CYCLE_SEL"]
    #[inline(always)]
    #[must_use]
    pub fn rg_extd_cycle_sel(&mut self) -> RgExtdCycleSelW<Syscfg10Spec> {
        RgExtdCycleSelW::new(self, 0)
    }
}
#[doc = "MIPITX DPHY SYSCFG 10: mipitx_apbifsaif_syscfg_40\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syscfg10::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`syscfg10::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Syscfg10Spec;
impl crate::RegisterSpec for Syscfg10Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`syscfg10::R`](R) reader structure"]
impl crate::Readable for Syscfg10Spec {}
#[doc = "`write(|w| ..)` method takes [`syscfg10::W`](W) writer structure"]
impl crate::Writable for Syscfg10Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets syscfg10 to value 0"]
impl crate::Resettable for Syscfg10Spec {
    const RESET_VALUE: u32 = 0;
}
