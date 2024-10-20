#[doc = "Register `clk_u0_m31dphy_txclkesc_lan0` reader"]
pub type R = crate::R<ClkU0M31dphyTxclkescLan0Spec>;
#[doc = "Register `clk_u0_m31dphy_txclkesc_lan0` writer"]
pub type W = crate::W<ClkU0M31dphyTxclkescLan0Spec>;
#[doc = "Field `clk_divcfg` reader - Clock divider coefficient: Max=60, Default=30, Min=15, Typical=30"]
pub type ClkDivcfgR = crate::FieldReader<u32>;
#[doc = "Field `clk_divcfg` writer - Clock divider coefficient: Max=60, Default=30, Min=15, Typical=30"]
pub type ClkDivcfgW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - Clock divider coefficient: Max=60, Default=30, Min=15, Typical=30"]
    #[inline(always)]
    pub fn clk_divcfg(&self) -> ClkDivcfgR {
        ClkDivcfgR::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - Clock divider coefficient: Max=60, Default=30, Min=15, Typical=30"]
    #[inline(always)]
    #[must_use]
    pub fn clk_divcfg(&mut self) -> ClkDivcfgW<ClkU0M31dphyTxclkescLan0Spec> {
        ClkDivcfgW::new(self, 0)
    }
}
#[doc = "Clock U0 M31 DPHY TX Clock Escape LAN0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_u0_m31dphy_txclkesc_lan0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_u0_m31dphy_txclkesc_lan0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkU0M31dphyTxclkescLan0Spec;
impl crate::RegisterSpec for ClkU0M31dphyTxclkescLan0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk_u0_m31dphy_txclkesc_lan0::R`](R) reader structure"]
impl crate::Readable for ClkU0M31dphyTxclkescLan0Spec {}
#[doc = "`write(|w| ..)` method takes [`clk_u0_m31dphy_txclkesc_lan0::W`](W) writer structure"]
impl crate::Writable for ClkU0M31dphyTxclkescLan0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets clk_u0_m31dphy_txclkesc_lan0 to value 0x1e"]
impl crate::Resettable for ClkU0M31dphyTxclkescLan0Spec {
    const RESET_VALUE: u32 = 0x1e;
}
