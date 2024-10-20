#[doc = "Register `clk_axi_cfg0` reader"]
pub type R = crate::R<ClkAxiCfg0Spec>;
#[doc = "Register `clk_axi_cfg0` writer"]
pub type W = crate::W<ClkAxiCfg0Spec>;
#[doc = "Field `clk_divcfg` reader - Clock divider coefficient: Max=3, Default=3, Min=3, Typical=3"]
pub type ClkDivcfgR = crate::FieldReader<u32>;
#[doc = "Field `clk_divcfg` writer - Clock divider coefficient: Max=3, Default=3, Min=3, Typical=3"]
pub type ClkDivcfgW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - Clock divider coefficient: Max=3, Default=3, Min=3, Typical=3"]
    #[inline(always)]
    pub fn clk_divcfg(&self) -> ClkDivcfgR {
        ClkDivcfgR::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - Clock divider coefficient: Max=3, Default=3, Min=3, Typical=3"]
    #[inline(always)]
    #[must_use]
    pub fn clk_divcfg(&mut self) -> ClkDivcfgW<ClkAxiCfg0Spec> {
        ClkDivcfgW::new(self, 0)
    }
}
#[doc = "Clock AXI Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_axi_cfg0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_axi_cfg0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkAxiCfg0Spec;
impl crate::RegisterSpec for ClkAxiCfg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk_axi_cfg0::R`](R) reader structure"]
impl crate::Readable for ClkAxiCfg0Spec {}
#[doc = "`write(|w| ..)` method takes [`clk_axi_cfg0::W`](W) writer structure"]
impl crate::Writable for ClkAxiCfg0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets clk_axi_cfg0 to value 0x03"]
impl crate::Resettable for ClkAxiCfg0Spec {
    const RESET_VALUE: u32 = 0x03;
}
