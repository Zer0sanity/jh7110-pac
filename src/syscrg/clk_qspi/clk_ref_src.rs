#[doc = "Register `clk_ref_src` reader"]
pub type R = crate::R<ClkRefSrcSpec>;
#[doc = "Register `clk_ref_src` writer"]
pub type W = crate::W<ClkRefSrcSpec>;
#[doc = "Field `clk_divcfg` reader - Clock divider coefficient: Max=16, Default=10, Min=10, Typical=10"]
pub type ClkDivcfgR = crate::FieldReader<u32>;
#[doc = "Field `clk_divcfg` writer - Clock divider coefficient: Max=16, Default=10, Min=10, Typical=10"]
pub type ClkDivcfgW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - Clock divider coefficient: Max=16, Default=10, Min=10, Typical=10"]
    #[inline(always)]
    pub fn clk_divcfg(&self) -> ClkDivcfgR {
        ClkDivcfgR::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - Clock divider coefficient: Max=16, Default=10, Min=10, Typical=10"]
    #[inline(always)]
    #[must_use]
    pub fn clk_divcfg(&mut self) -> ClkDivcfgW<ClkRefSrcSpec> {
        ClkDivcfgW::new(self, 0)
    }
}
#[doc = "Clock QSPI Reference Source\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_ref_src::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_ref_src::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkRefSrcSpec;
impl crate::RegisterSpec for ClkRefSrcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk_ref_src::R`](R) reader structure"]
impl crate::Readable for ClkRefSrcSpec {}
#[doc = "`write(|w| ..)` method takes [`clk_ref_src::W`](W) writer structure"]
impl crate::Writable for ClkRefSrcSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets clk_ref_src to value 0x0a"]
impl crate::Resettable for ClkRefSrcSpec {
    const RESET_VALUE: u32 = 0x0a;
}
