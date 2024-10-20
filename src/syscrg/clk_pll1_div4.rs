#[doc = "Register `clk_pll1_div4` reader"]
pub type R = crate::R<ClkPll1Div4Spec>;
#[doc = "Register `clk_pll1_div4` writer"]
pub type W = crate::W<ClkPll1Div4Spec>;
#[doc = "Field `clk_divcfg` reader - Clock divider coefficient: Max=2, Default=2, Min=2, Typical=2"]
pub type ClkDivcfgR = crate::FieldReader<u32>;
#[doc = "Field `clk_divcfg` writer - Clock divider coefficient: Max=2, Default=2, Min=2, Typical=2"]
pub type ClkDivcfgW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - Clock divider coefficient: Max=2, Default=2, Min=2, Typical=2"]
    #[inline(always)]
    pub fn clk_divcfg(&self) -> ClkDivcfgR {
        ClkDivcfgR::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - Clock divider coefficient: Max=2, Default=2, Min=2, Typical=2"]
    #[inline(always)]
    #[must_use]
    pub fn clk_divcfg(&mut self) -> ClkDivcfgW<ClkPll1Div4Spec> {
        ClkDivcfgW::new(self, 0)
    }
}
#[doc = "clk_pll1_div4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_pll1_div4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_pll1_div4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkPll1Div4Spec;
impl crate::RegisterSpec for ClkPll1Div4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk_pll1_div4::R`](R) reader structure"]
impl crate::Readable for ClkPll1Div4Spec {}
#[doc = "`write(|w| ..)` method takes [`clk_pll1_div4::W`](W) writer structure"]
impl crate::Writable for ClkPll1Div4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets clk_pll1_div4 to value 0x02"]
impl crate::Resettable for ClkPll1Div4Spec {
    const RESET_VALUE: u32 = 0x02;
}
