#[doc = "Register `gmac1_gtx` reader"]
pub type R = crate::R<Gmac1GtxSpec>;
#[doc = "Register `gmac1_gtx` writer"]
pub type W = crate::W<Gmac1GtxSpec>;
#[doc = "Field `clk_divcfg` reader - Clock divider coefficient: Max=15, Default=8, Min=12, Typical=10"]
pub type ClkDivcfgR = crate::FieldReader<u32>;
#[doc = "Field `clk_divcfg` writer - Clock divider coefficient: Max=15, Default=8, Min=12, Typical=10"]
pub type ClkDivcfgW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - Clock divider coefficient: Max=15, Default=8, Min=12, Typical=10"]
    #[inline(always)]
    pub fn clk_divcfg(&self) -> ClkDivcfgR {
        ClkDivcfgR::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - Clock divider coefficient: Max=15, Default=8, Min=12, Typical=10"]
    #[inline(always)]
    #[must_use]
    pub fn clk_divcfg(&mut self) -> ClkDivcfgW<Gmac1GtxSpec> {
        ClkDivcfgW::new(self, 0)
    }
}
#[doc = "Clock GMAC1 GTX\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmac1_gtx::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gmac1_gtx::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gmac1GtxSpec;
impl crate::RegisterSpec for Gmac1GtxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gmac1_gtx::R`](R) reader structure"]
impl crate::Readable for Gmac1GtxSpec {}
#[doc = "`write(|w| ..)` method takes [`gmac1_gtx::W`](W) writer structure"]
impl crate::Writable for Gmac1GtxSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets gmac1_gtx to value 0x08"]
impl crate::Resettable for Gmac1GtxSpec {
    const RESET_VALUE: u32 = 0x08;
}
