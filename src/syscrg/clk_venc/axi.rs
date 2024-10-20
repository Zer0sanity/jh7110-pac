#[doc = "Register `axi` reader"]
pub type R = crate::R<AxiSpec>;
#[doc = "Register `axi` writer"]
pub type W = crate::W<AxiSpec>;
#[doc = "Field `clk_divcfg` reader - Clock divider coefficient: Max=15, Default=5, Min=5, Typical=5"]
pub type ClkDivcfgR = crate::FieldReader<u32>;
#[doc = "Field `clk_divcfg` writer - Clock divider coefficient: Max=15, Default=5, Min=5, Typical=5"]
pub type ClkDivcfgW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - Clock divider coefficient: Max=15, Default=5, Min=5, Typical=5"]
    #[inline(always)]
    pub fn clk_divcfg(&self) -> ClkDivcfgR {
        ClkDivcfgR::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - Clock divider coefficient: Max=15, Default=5, Min=5, Typical=5"]
    #[inline(always)]
    #[must_use]
    pub fn clk_divcfg(&mut self) -> ClkDivcfgW<AxiSpec> {
        ClkDivcfgW::new(self, 0)
    }
}
#[doc = "Clock Video Encoder AXI\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`axi::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`axi::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AxiSpec;
impl crate::RegisterSpec for AxiSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`axi::R`](R) reader structure"]
impl crate::Readable for AxiSpec {}
#[doc = "`write(|w| ..)` method takes [`axi::W`](W) writer structure"]
impl crate::Writable for AxiSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets axi to value 0x05"]
impl crate::Resettable for AxiSpec {
    const RESET_VALUE: u32 = 0x05;
}
