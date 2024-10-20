#[doc = "Register `gmac1_rmii_rtx` reader"]
pub type R = crate::R<Gmac1RmiiRtxSpec>;
#[doc = "Register `gmac1_rmii_rtx` writer"]
pub type W = crate::W<Gmac1RmiiRtxSpec>;
#[doc = "Field `clk_divcfg` reader - Clock divider coefficient: Max=30, Default=2, Min=2, Typical=2"]
pub type ClkDivcfgR = crate::FieldReader<u32>;
#[doc = "Field `clk_divcfg` writer - Clock divider coefficient: Max=30, Default=2, Min=2, Typical=2"]
pub type ClkDivcfgW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - Clock divider coefficient: Max=30, Default=2, Min=2, Typical=2"]
    #[inline(always)]
    pub fn clk_divcfg(&self) -> ClkDivcfgR {
        ClkDivcfgR::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - Clock divider coefficient: Max=30, Default=2, Min=2, Typical=2"]
    #[inline(always)]
    #[must_use]
    pub fn clk_divcfg(&mut self) -> ClkDivcfgW<Gmac1RmiiRtxSpec> {
        ClkDivcfgW::new(self, 0)
    }
}
#[doc = "Clock GMAC RMII RTX\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmac1_rmii_rtx::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gmac1_rmii_rtx::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gmac1RmiiRtxSpec;
impl crate::RegisterSpec for Gmac1RmiiRtxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gmac1_rmii_rtx::R`](R) reader structure"]
impl crate::Readable for Gmac1RmiiRtxSpec {}
#[doc = "`write(|w| ..)` method takes [`gmac1_rmii_rtx::W`](W) writer structure"]
impl crate::Writable for Gmac1RmiiRtxSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets gmac1_rmii_rtx to value 0x02"]
impl crate::Resettable for Gmac1RmiiRtxSpec {
    const RESET_VALUE: u32 = 0x02;
}
