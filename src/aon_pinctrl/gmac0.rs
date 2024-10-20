#[doc = "Register `gmac0[%s]` reader"]
pub type R = crate::R<Gmac0Spec>;
#[doc = "Register `gmac0[%s]` writer"]
pub type W = crate::W<Gmac0Spec>;
#[doc = "Field `cfg` reader - rxd0 - 0: GMAC0 IO voltage select 3.3V, 1: GMAC0 IO voltage select 2.5V, 2: GMAC0 IO voltage select 1.8V rxc_func_sel - Function selector of GMAC0_RXC: * Function 0: u0_aon_crg_clk_gmac0_rgmii_rx, * Function 1: u0_aon_crg_clk_gmac0_rmii_ref, * Function 2: None, * Function 3: None"]
pub type CfgR = crate::FieldReader;
#[doc = "Field `cfg` writer - rxd0 - 0: GMAC0 IO voltage select 3.3V, 1: GMAC0 IO voltage select 2.5V, 2: GMAC0 IO voltage select 1.8V rxc_func_sel - Function selector of GMAC0_RXC: * Function 0: u0_aon_crg_clk_gmac0_rgmii_rx, * Function 1: u0_aon_crg_clk_gmac0_rmii_ref, * Function 2: None, * Function 3: None"]
pub type CfgW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - rxd0 - 0: GMAC0 IO voltage select 3.3V, 1: GMAC0 IO voltage select 2.5V, 2: GMAC0 IO voltage select 1.8V rxc_func_sel - Function selector of GMAC0_RXC: * Function 0: u0_aon_crg_clk_gmac0_rgmii_rx, * Function 1: u0_aon_crg_clk_gmac0_rmii_ref, * Function 2: None, * Function 3: None"]
    #[inline(always)]
    pub fn cfg(&self) -> CfgR {
        CfgR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - rxd0 - 0: GMAC0 IO voltage select 3.3V, 1: GMAC0 IO voltage select 2.5V, 2: GMAC0 IO voltage select 1.8V rxc_func_sel - Function selector of GMAC0_RXC: * Function 0: u0_aon_crg_clk_gmac0_rgmii_rx, * Function 1: u0_aon_crg_clk_gmac0_rmii_ref, * Function 2: None, * Function 3: None"]
    #[inline(always)]
    #[must_use]
    pub fn cfg(&mut self) -> CfgW<Gmac0Spec> {
        CfgW::new(self, 0)
    }
}
#[doc = "AON IOMUX CFG SAIF SYSCFG - GMAC0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmac0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gmac0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gmac0Spec;
impl crate::RegisterSpec for Gmac0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gmac0::R`](R) reader structure"]
impl crate::Readable for Gmac0Spec {}
#[doc = "`write(|w| ..)` method takes [`gmac0::W`](W) writer structure"]
impl crate::Writable for Gmac0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets gmac0[%s]
to value 0x02"]
impl crate::Resettable for Gmac0Spec {
    const RESET_VALUE: u32 = 0x02;
}
