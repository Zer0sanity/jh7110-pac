#[doc = "Register `clk_u0_m31dphy_refclk_in` reader"]
pub type R = crate::R<ClkU0M31dphyRefclkInSpec>;
#[doc = "Register `clk_u0_m31dphy_refclk_in` writer"]
pub type W = crate::W<ClkU0M31dphyRefclkInSpec>;
#[doc = "Field `clk_divcfg` reader - Clock divider coefficient: Max=16, Default=12, Min=6, Typical=12"]
pub type ClkDivcfgR = crate::FieldReader<u32>;
#[doc = "Field `clk_divcfg` writer - Clock divider coefficient: Max=16, Default=12, Min=6, Typical=12"]
pub type ClkDivcfgW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - Clock divider coefficient: Max=16, Default=12, Min=6, Typical=12"]
    #[inline(always)]
    pub fn clk_divcfg(&self) -> ClkDivcfgR {
        ClkDivcfgR::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - Clock divider coefficient: Max=16, Default=12, Min=6, Typical=12"]
    #[inline(always)]
    #[must_use]
    pub fn clk_divcfg(&mut self) -> ClkDivcfgW<ClkU0M31dphyRefclkInSpec> {
        ClkDivcfgW::new(self, 0)
    }
}
#[doc = "Clock U0 M31 DPHY Reference In\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_u0_m31dphy_refclk_in::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_u0_m31dphy_refclk_in::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkU0M31dphyRefclkInSpec;
impl crate::RegisterSpec for ClkU0M31dphyRefclkInSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk_u0_m31dphy_refclk_in::R`](R) reader structure"]
impl crate::Readable for ClkU0M31dphyRefclkInSpec {}
#[doc = "`write(|w| ..)` method takes [`clk_u0_m31dphy_refclk_in::W`](W) writer structure"]
impl crate::Writable for ClkU0M31dphyRefclkInSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets clk_u0_m31dphy_refclk_in to value 0x0c"]
impl crate::Resettable for ClkU0M31dphyRefclkInSpec {
    const RESET_VALUE: u32 = 0x0c;
}
